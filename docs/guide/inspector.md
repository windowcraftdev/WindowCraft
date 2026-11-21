<!-- Made by Nordan -->

# System & Gaming Inspector

System & Gaming Inspector inspects PCIe interrupt settings, GPU display flags, kernel timer resolution, startup entries, cache folders, and DNS latency, alongside the X-Ray Inspector side drawer.

## MSI Mode

### PCIe device interrupt priority and MSI mode

Lists PCIe devices in the GPU, NVMe (`stornvme` or NVM controller), Network, and USB controller classes from `HKLM\SYSTEM\CurrentControlSet\Enum\PCI`. Each device has an interrupt priority dropdown (`Undefined`, `Low`, `Normal`, `High`) and an MSI mode switch (`0` or `1`) that apply immediately inside individual [Time Machine](time-machine.md) snapshot sessions.

Changes: `HKLM\SYSTEM\CurrentControlSet\Enum\PCI\<instance>\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties\MSISupported` (`DWord`) and `...\Affinity Policy\DevicePriority` (`DWord`: `0` = Undefined, `1` = Low, `2` = Normal, `3` = High).

Undo: Toggle the switch, pick the previous priority, or roll back the session in [Time Machine](time-machine.md).

Note: Requires administrator rights. Changes take effect after a restart.

## Display

### Hardware-accelerated GPU scheduling

Controls Windows Hardware-accelerated GPU scheduling (`HwSchMode`) in the graphics driver configuration.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\HwSchMode = 2` (on) or `1` (off).

Undo: Turn off the switch or roll back in Time Machine.

Note: Requires administrator rights and a restart to take effect.

### Variable refresh rate

Controls per-user DirectX variable refresh rate optimization for games.

Changes: `HKCU\Software\Microsoft\DirectX\UserGpuPreferences\DirectXUserGlobalSettings = "VRROptimizeEnable=1;"` (on) or `"VRROptimizeEnable=0;"` (off).

Undo: Turn off the switch or roll back in Time Machine.

### Fullscreen optimizations

Controls GameConfigStore fullscreen optimization behavior flags for the current user.

Changes: `HKCU\System\GameConfigStore\GameDVR_FSEBehaviorMode = 0` (on) or `2` (off), and `GameDVR_HonorUserFSEBehaviorMode` (`<RemoveEntry>` when on, `1` when off).

Undo: Turn off the switch or roll back in Time Machine.

### Kernel Timer

Displays read-only `Current`, `Finest` (`Maximum resolution`), and `Coarsest` (`Minimum resolution`) kernel timer periods in milliseconds queried from `NtQueryTimerResolution` in `ntdll.dll`.

## Autoruns

### Startup entries

Scans four startup sources in parallel, checks Authenticode embedded and catalog signatures offline (showing a green or amber dot with the publisher name, `Unsigned`, or `COM handler`), and lets you turn entries on or off without deleting them. Every switch records a [Time Machine](time-machine.md) snapshot.

Changes:
- `Run key`: writes a 12-byte `StartupApproved\Run` flag (`0x02` on, `0x03` off) in `HKLM` or `HKCU` for entries from `Run` and `RunOnce`.
- `Startup folder`: writes a 12-byte `StartupApproved\StartupFolder` flag in `HKLM` (for `ProgramData`) or `HKCU` (for user `AppData`).
- `Task scheduler`: sets `Enabled` (`VARIANT_TRUE` or `VARIANT_FALSE`) on tasks in `\`.
- `Services`: sets non-Microsoft services (`Automatic`, `Automatic (Delayed)`, or `Disabled`) to `Automatic` or `Disabled`. Services on the protected list (`RpcSs`, `DcomLaunch`, `LSM`, `BFE`, `mpssvc`, `WinDefend`, `ProfSvc`) show a `Protected` badge and cannot be disabled.

Undo: Toggle the switch back or roll back the session in Time Machine.

Note: Requires administrator rights for machine-wide `HKLM` keys, system tasks, and services.

## Cleaner

### Scan

Scans the 12 built-in cache targets and `%AppData%` / `%LocalAppData%` for potentially orphaned folders in parallel without following junctions or symbolic links. Displays file counts and sizes without deleting anything.

### Clean Selected

Deletes files inside the checked targets and reviewed orphan folders, leaving root cache directories intact and skipping locked files. Reparse points (junctions or symlinks) are removed as links without touching their targets.

Changes: Deletes files inside the selected directories listed below.

Undo: Not reversible.

Targets:

| Target | Folders | Note |
| --- | --- | --- |
| User temporary files | `%TEMP%` |  |
| System temporary files | `%SystemRoot%\Temp` | Requires administrator rights for protected system files |
| CBS and DISM logs | `%SystemRoot%\Logs\CBS`, `%SystemRoot%\Logs\DISM` | Requires administrator rights |
| Minidump files | `%SystemRoot%\Minidump` | Requires administrator rights |
| NVIDIA shader cache | `%LOCALAPPDATA%\NVIDIA\DXCache`, `%LOCALAPPDATA%\NVIDIA\GLCache` |  |
| AMD shader cache | `%LOCALAPPDATA%\AMD\DxCache`, `%LOCALAPPDATA%\AMD\GLCache`, `%LOCALAPPDATA%\AMD\DxcCache` |  |
| DirectX shader cache | `%LOCALAPPDATA%\D3DSCache` |  |
| Cargo registry cache | `%USERPROFILE%\.cargo\registry\cache`, `%USERPROFILE%\.cargo\git\db` |  |
| npm cache | `%LOCALAPPDATA%\npm-cache` |  |
| pip cache | `%LOCALAPPDATA%\pip\Cache` |  |
| NuGet package cache | `%USERPROFILE%\.nuget\packages` |  |
| ISO Studio work files | `%LOCALAPPDATA%\WindowCraft\iso` | Skipped while an ISO is open in ISO Studio in this run of WindowCraft or a WIM image is mounted under `iso\mount`. Also deletes `Win11.iso` downloaded by ISO Studio |

### Possibly Orphaned Folders

Lists direct non-link subdirectories in `%AppData%` and `%LocalAppData%` whose names do not match an installed program or the keep list (`microsoft`, `packages`, `temp`, `windowcraft`, `programs`, `nvidia`). Entries are never preselected.

Changes: Deletes checked folders (and any non-junction files inside them).

Undo: Not reversible.

Note: Review every folder manually before cleaning, because folder names alone do not prove an application is uninstalled.

## DNS Benchmark

### Run Benchmark

Sends 6 rounds of DNS A-record queries across 5 domains (`example.com`, `www.microsoft.com`, `www.wikipedia.org`, `github.com`, `cloudflare.com`) concurrently to all 8 providers in `dns.json` over UDP port 53, TCP port 53, and DNS-over-HTTPS (with a 2-second per-query timeout). Displays min, average, and jitter in milliseconds per protocol, plus a combined score, fastest first.

Changes: None (read-only network queries; to apply a DNS provider, use `DNS server` in [Tweaks & Privacy](tweaks.md)).

## X-Ray Inspector

### X-Ray Inspector drawer

Opens on the right side of the window when you click the eye icon on any tweak, feature, or AppX package, pick an item from global search (`Ctrl+K` or `Ctrl+F`), or pass `--inspect <tweak-id>` on the command line. Drag the left grip to resize or flick closed, or press `Escape` or the close button.

- Tweaks: shows each registry key and service rule with its `Current` live value, `Target` desired value, `Undo` pre-tweak value (from the baseline snapshot when applied, or catalog `original`), value kind, `Native Operations`, and an `Undo This Tweak` button (or `This change cannot be undone automatically.` when marked irreversible).
- Features: lists `DismEnableFeature` components, registry entries, and native operations.
- AppX packages: shows `Package` identifier, `Store ID` (or `none`), and any dependency warning.

Undo: Re-apply the tweak if you clicked `Undo This Tweak`.
