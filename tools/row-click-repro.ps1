param([string]$ProcessName = "zapdude")
$ErrorActionPreference = "Stop"
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class Win32Z3 {
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint flags, uint dx, uint dy, uint data, UIntPtr extra);
    [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
}
"@
[Win32Z3]::SetProcessDPIAware() | Out-Null

# 1) show the window first (the instance boots hidden to tray)
try { Invoke-RestMethod -Method Post "http://127.0.0.1:29977/api/ui/show" -TimeoutSec 5 | Out-Null } catch {}
Start-Sleep -Milliseconds 700

$proc = Get-Process -Name $ProcessName | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
if (-not $proc) { throw "no window handle for $ProcessName" }
$hwnd = $proc.MainWindowHandle
[Win32Z3]::ShowWindow($hwnd, 9) | Out-Null
[Win32Z3]::SetForegroundWindow($hwnd) | Out-Null
Start-Sleep -Milliseconds 900

$origin = New-Object Win32Z3+POINT
$origin.X = 0; $origin.Y = 0
[Win32Z3]::ClientToScreen($hwnd, [ref]$origin) | Out-Null

# 2) read the page's own opinion: visible? focused? row rect?
$expr = 'JSON.stringify((function(){ var r = document.querySelector("#pane-side div[data-testid^=''list-item'']"); var b = r ? r.getBoundingClientRect() : null; return { vis: document.visibilityState, focused: document.hasFocus(), dpr: window.devicePixelRatio, x: b ? (b.x + b.width/2 - 0) : -1, y: b ? (b.y + b.height/2) : -1, railZero: (function(){ var a = document.querySelectorInitializer || null; return null; })() }; })())'
$raw = .\tools\debug-eval.ps1 -Script $expr -Account account-1 2>&1 | Out-String
$clean = $raw.Replace('\', '')
Write-Output ("page state: " + $clean.Trim())
$mx = [regex]::Match($clean, '"x":([0-9.]+)')
$my = [regex]::Match($clean, '"y":([0-9.]+)')
$vis = [regex]::Match($clean, '"vis":"(\w+)"')
if ($vis.Success -and $vis.Groups[1].Value -ne "visible") { Write-Output "PAGE NOT VISIBLE - clicks would go nowhere; retry the show" }
if (-not ($mx.Success -and $my.Success)) { throw "row geometry unreadable" }
$rowX = [double]$mx.Groups[1].Value
$rowY = [double]$my.Groups[1].Value

# 3) the account webview starts at RAIL_WIDTH (75) client px: page(0,0)
#    in client = (75, 0). Screen = clientOrigin + (75 + pageX, pageY).
$railPx = 75
$slotX = 0
$slotY = 0
foreach ($line in (Get-Content "$env:APPDATA\app.zapzap.desktop\config.json" -Raw)) {}
# Slot offset lands from the host layout constants (RAIL_WIDTH+1px stroke);
# verify by probing: instead of guessing, click a SLIGHT GRID of points
# around the expected spot and stop when the row selects.
$baseScreenX = $origin.X + $railPx + [int]$rowX
$baseScreenY = $origin.Y + [int]$rowY
Write-Output ("expected screen point: " + $baseScreenX + "," + $baseScreenY)

function Click-At([int]$x, [int]$y) {
    [Win32Z3]::SetCursorPos($x, $y) | Out-Null
    Start-Sleep -Milliseconds 180
    [Win32Z3]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 50
    [Win32Z3]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 400
    $chk = 'JSON.stringify({sel: (document.querySelector("#pane-side div[data-testid^=''list-item'']")||{}).getAttribute ? document.querySelector("#pane-side div[data-testid^=''list-item'']").getAttribute("aria-selected") : null, main: !!document.getElementById("main")})'
    $out = .\tools\debug-eval.ps1 -Script $chk -Account account-1 2>&1 | Out-String
    $cl = $out.Replace('\', '')
    $selM = [regex]::Match($cl, '"sel":("?)(\w*)\1')
    $mainM = [regex]::Match($cl, '"main":(true|false)')
    $sel = if ($selM.Success) { $selM.Groups[2].Value } else { "?" }
    $main = if ($mainM.Success) { $mainM.Groups[1].Value } else { "?" }
    Write-Output ("click@ " + $x + "," + $y + " => sel=" + $sel + " main=" + $main)
    return ($main -eq "true")
}

$offsets = @(@(0,0), @(-75,0), @(75,0), @(0,-30), @(0,30))
foreach ($o in $offsets) {
    $hit = Click-At ($baseScreenX + $o[0]) ($baseScreenY + $o[1])
    if ($hit) { Write-Output ("CONVERSATION RENDERED with click at offset " + $o[0] + "," + $o[1]); break }
}
Start-Sleep -Seconds 2
Write-Output "--- final census ---"
.\tools\debug-eval.ps1 -ScriptFile pane-census.js -Account account-1 2>&1 | Select-Object -Last 2
