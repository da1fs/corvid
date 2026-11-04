//! External clicks: where links leave Corvid.
//!
//! Chat links must never hijack an account webview: "open in the app's own
//! browser" lands the user on an unauthenticated page (no cookies, login
//! demanded, trust broken). The rule set lives in window.rs:
//! links to the EMBEDDED services stay inside (the chat page navigates
//! itself); everything else opens in the user's BROWSER, system default
//! unless they picked one. Windows announces every installed desktop
//! browser under `Software\Clients\StartMenuInternet` (HKCU overrides
//! HKLM): its `shell\open\command` verb carries `%1` as the URL slot.
//!
//! Safety rails: only http/https/mailto ever reaches a spawn, registry
//! halves with suspicious key names never enumerate, and a broken custom
//! command falls back to the system default instead of failing silently.

pub struct BrowserEntry {
    /// Registry key name (the stable identity persisted in prefs, e.g.
    /// "Chrome", "Firefox-308046B0AF4A39CB", "Microsoft Edge").
    pub key: String,
    /// Human display name ("Google Chrome", "Firefox", ...).
    pub name: String,
}

#[cfg(windows)]
pub fn list_browsers() -> Vec<BrowserEntry> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;
    let mut out: Vec<BrowserEntry> = Vec::new();
    for root in [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        let Ok(clients) = RegKey::predef(root).open_subkey(START_MENU_INTERNET) else {
            continue;
        };
        for key_name in clients.enum_keys().flatten() {
            // Enumerated keys must be plain: no escaping subpaths, no
            // argument-lookalikes, nothing that could lie later when the
            // key is re-opened by name.
            if key_name.is_empty() || !registry_key_name_ok(&key_name) {
                continue;
            }
            // HKCU already answered for this key (its entry wins).
            if out.iter().any(|entry| entry.key == key_name) {
                continue;
            }
            let name: String = clients
                .open_subkey(&key_name)
                .ok()
                .and_then(|browser| browser.get_value("DisplayName").ok())
                .unwrap_or_else(|| key_name.clone());
            if name.trim().is_empty() {
                continue;
            }
            // Only browsers carrying a REAL open verb are candidates: a
            // broken choice would degrade to the default anyway, so better
            // never offered.
            if command_for(&key_name).is_none() {
                continue;
            }
            out.push(BrowserEntry {
                key: key_name,
                name,
            });
        }
    }
    out
}

#[cfg(not(windows))]
pub fn list_browsers() -> Vec<BrowserEntry> {
    Vec::new()
}

#[cfg(windows)]
const START_MENU_INTERNET: &str = r"Software\Clients\StartMenuInternet";

/// Enumerated key names get a strict charset: letters, digits and a few
/// punctuation marks. Stops a weird entry from escaping the subpath when
/// re-opened (`Chrome\..\..\Pwn`) or injecting into the command parser.
#[cfg(windows)]
fn registry_key_name_ok(key: &str) -> bool {
    key.chars().all(|c| {
        c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '(' | ')' | ' ' | '+' | '#')
    })
}

/// The `shell\open\command` default value for one browser key (HKCU first,
/// HKLM second). None is an honest "this browser has no usable verb".
#[cfg(windows)]
fn command_for(key: &str) -> Option<String> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ};
    use winreg::RegKey;
    if !registry_key_name_ok(key) {
        return None;
    }
    for root in [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        let Ok(clients) =
            RegKey::predef(root).open_subkey_with_flags(START_MENU_INTERNET, KEY_READ)
        else {
            continue;
        };
        let Ok(verb) = clients.open_subkey_with_flags(
            format!(r"{key}\shell\open\command"),
            winreg::enums::KEY_QUERY_VALUE,
        ) else {
            continue;
        };
        let command: String = verb.get_value("").ok()?;
        if !command.trim().is_empty() {
            return Some(command);
        }
    }
    None
}

/// Open a chat link OUTSIDE the app. Only http/https/mailto qualifies; the
/// picked browser (prefs.external_browser) is honored when its registry
/// verb still exists, the system default covers every failure path.
#[cfg(windows)]
pub fn open_external(url: &url::Url) {
    let scheme = url.scheme().to_ascii_lowercase();
    if !matches!(scheme.as_str(), "http" | "https" | "mailto") {
        return;
    }
    let target = url.as_str().trim().to_string();
    if target.is_empty() || target.len() > 2048 {
        return;
    }
    let chosen = crate::SHARED
        .get()
        .map(|shared| shared.settings.current_prefs().external_browser)
        .unwrap_or_default();
    if !chosen.trim().is_empty() {
        if let Some(command) = command_for(chosen.trim()) {
            if let Some((program, args)) = build_browser_invocation(&command, &target) {
                if spawn_hidden(program, args) {
                    return;
                }
            }
        }
        // Broken or vanished choice: fall through to the system default.
    }
    open_default(&target);
}

#[cfg(not(windows))]
pub fn open_external(_url: &url::Url) {}

/// The system default browser route: `start` with an EMPTY title token,
/// so the URL itself is never mistaken for a console title. One argument,
/// one quoted value: no tokenization games.
#[cfg(windows)]
fn open_default(target: &str) {
    let _ = spawn_hidden(
        "cmd".to_string(),
        vec![
            "/C".to_string(),
            "start".to_string(),
            String::new(),
            target.to_string(),
        ],
    );
}

#[cfg(windows)]
fn spawn_hidden(program: String, args: Vec<String>) -> bool {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let mut command = std::process::Command::new(program);
    command
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    command.spawn().is_ok()
}

/// Turn a registry `shell\open\command` string into (program, args) with
/// `%1` replaced by the URL. Windows command lines: spaces split tokens,
/// double quotes group them; a first token without quotes is just the exe
/// path (spaces inside paths always arrive quoted).
#[cfg(windows)]
fn split_command_line(cmd: &str) -> (String, Vec<String>) {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in cmd.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    let mut iterator = tokens.into_iter();
    let program = iterator.next().unwrap_or_default();
    (program, iterator.collect())
}

/// `Chrome.exe --single-argument %1` -> (Chrome.exe, [--single-argument,
/// <url>]). A command WITHOUT `%1` gets the URL appended (the old-school
/// pattern some packs still ship); a command with NO program is refused.
#[cfg(windows)]
pub(crate) fn build_browser_invocation(cmd: &str, target: &str) -> Option<(String, Vec<String>)> {
    let (program, raw_args) = split_command_line(cmd);
    if program.trim().is_empty() {
        return None;
    }
    let verb_slot = raw_args.iter().any(|arg| arg.contains("%1"));
    let args = raw_args
        .into_iter()
        .map(|arg| arg.replace("%1", target))
        .collect::<Vec<String>>();
    let args = if verb_slot {
        args
    } else {
        let mut args = args;
        args.push(target.to_string());
        args
    };
    Some((program, args))
}

#[cfg(windows)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_quoted_and_replaces_the_slot() {
        let (program, args) = split_command_line(
            r#""C:\Program Files\Google\Chrome\Application\chrome.exe" --single-argument %1"#,
        );
        assert_eq!(
            program,
            r"C:\Program Files\Google\Chrome\Application\chrome.exe"
        );
        assert_eq!(
            args,
            vec!["--single-argument".to_string(), "%1".to_string()]
        );

        let (program, args) = build_browser_invocation(
            r#""C:\Program Files\Google\Chrome\Application\chrome.exe" --single-argument %1"#,
            "https://example.com/x",
        )
        .unwrap();
        assert!(program.ends_with("chrome.exe"));
        assert_eq!(args[1], "https://example.com/x");

        // No %1 slot: the URL appends.
        let (_, args) =
            build_browser_invocation(r#""C:\bin\ Gordito\app.exe""#, "https://a.b").unwrap();
        assert_eq!(args.last().unwrap(), "https://a.b");

        // No program at all: refused, the default browser takes over.
        assert!(build_browser_invocation("   ", "https://a.b").is_none());
    }

    #[test]
    fn key_names_cannot_escape_the_registry_path() {
        assert!(registry_key_name_ok("Chrome"));
        assert!(registry_key_name_ok("Firefox-308046B0AF4A39CB"));
        assert!(registry_key_name_ok("Microsoft Edge"));
        assert!(!registry_key_name_ok(r"Chrome\..\..\Pwn"));
        assert!(!registry_key_name_ok(r"Chrome\shell\open\command\x"));
        assert!(!registry_key_name_ok("A%1"));
    }
}
