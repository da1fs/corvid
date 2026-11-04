param([string]$ProcessName = "zapdude")
$ErrorActionPreference = "Stop"
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class Win32Z4 {
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
[Win32Z4]::SetProcessDPIAware() | Out-Null

function Click-Row([int]$index, [IntPtr]$hwnd, $origin) {
    # Real OS click on chat row #index (trusted events, like the user).
    $expr = ('JSON.stringify((function(){ var rows = document.querySelectorAll("#pane-side div[data-testid^=''list-item'']"); var r = rows[' + $index + ']; if (!r) { return null; } var b = r.getBoundingClientRect(); return { x: b.x + b.width/2, y: b.y + b.height/2, title: (r.querySelector("span[title]")||{}).title }; })())')
    $raw = .\tools\debug-eval.ps1 -Script $expr -Account account-1 2>&1 | Out-String
    $clean = $raw.Replace('\', '')
    $mx = [regex]::Match($clean, '"x":([0-9.]+)')
    $my = [regex]::Match($clean, '"y":([0-9.]+)')
    if (-not ($mx.Success -and $my.Success)) { Write-Output ("row " + $index + " geometry unreadable"); return $false }
    $x = [double]$mx.Groups[1].Value
    $y = [double]$my.Groups[1].Value
    $screenX = $origin.X + 75 + [int]$x    # rail width offset
    $screenY = $origin.Y + [int]$y
    [Win32Z4]::SetCursorPos($screenX, $screenY) | Out-Null
    Start-Sleep -Milliseconds 200
    [Win32Z4]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 55
    [Win32Z4]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 1900

    $chk = ('JSON.stringify((function(){ var m = document.getElementById("main"); var t = document.querySelector("span[title]"); return { open: !!m, header: m ? !!m.querySelector("header") : null, composer: m ? !!m.querySelector("[contenteditable=true]") : null, rects: m ? m.getClientRects().length : -1, stampsInMain: m ? m.querySelectorAll("[data-zapdude-promo]").length : -1 }; })())')
    $out = .\tools\debug-eval.ps1 -Script $chk -Account account-1 2>&1 | Out-String
    $cl = $out.Replace('\', '')
    $mainM = [regex]::Match($cl, '"open":(true|false)')
    $compM = [regex]::Match($cl, '"composer":(true|false)')
    $rectM = [regex]::Match($cl, '"rects":(\d+)')
    $stampM = [regex]::Match($cl, '"stampsInMain":(\d+)')
    Write-Output ("row" + $index + " => open=" + $mainM.Groups[1].Value + " composer=" + $compM.Groups[1].Value + " rects=" + $rectM.Groups[1].Value + " stampsInMain=" + $stampM.Groups[1].Value)
    return ($mainM.Success -and $mainM.Groups[1].Value -eq "true")
}

try { Invoke-RestMethod -Method Post "http://127.0.0.1:29977/api/ui/show" -TimeoutSec 5 | Out-Null } catch {}
Start-Sleep -Milliseconds 700
$proc = Get-Process -Name $ProcessName | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
if (-not $proc) { throw "no window" }
$hwnd = $proc.MainWindowHandle
[Win32Z4]::ShowWindow($hwnd, 9) | Out-Null
[Win32Z4]::SetForegroundWindow($hwnd) | Out-Null
Start-Sleep -Milliseconds 800
$origin = New-Object Win32Z4+POINT
$origin.X = 0; $origin.Y = 0
[Win32Z4]::ClientToScreen($hwnd, [ref]$origin) | Out-Null

Write-Output "--- chat->chat transitions with REAL clicks ---"
Click-Row 0 $hwnd $origin
Click-Row 1 $hwnd $origin
Click-Row 0 $hwnd $origin
Write-Output "--- sweeper health (stamps must stay tiny and outside the chat) ---"
.\tools\debug-eval.ps1 -ScriptFile sweeper-health.js -Account account-1 2>&1 | Select-Object -Last 2
