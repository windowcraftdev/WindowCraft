<!-- Made by Nordan -->

# System Config & Fixes

System Config & Fixes manages Windows optional components, runs repair workflows, configures the PowerShell profile and OpenSSH Server, and launches classic Windows panels.

## Page controls

### Install Features

Applies every checked item in `Windows Optional Features` in one [Time Machine](time-machine.md) snapshot session per feature. Appears in the bottom action bar whenever at least one optional feature is ticked.

Changes: See the selected entries under Windows Optional Features.

Undo: Time Machine snapshot for features that were off beforehand and for `Schedule daily registry backups`.

Note: Requires administrator rights.

## Windows Optional Features

### Turn on .NET Framework 2.0, 3.5, and 4.8

Enables the .NET Framework 3.5 and 4.8 advanced runtime components so applications targeting .NET Framework 2.0, 3.0, 3.5, and 4.x can run.

Changes: `DismEnableFeature NetFx4-AdvSrvs`, `DismEnableFeature NetFx3`.

Undo: Time Machine snapshot disables any of those features that were off beforehand.

Note: Requires administrator rights. Windows may download feature payloads from Windows Update if local sources are absent.

### Turn on Hyper-V

Enables the Hyper-V virtualization platform and management tools for creating and running hardware virtual machines.

Changes: `DismEnableFeature Microsoft-Hyper-V-All`.

Undo: Time Machine snapshot disables `Microsoft-Hyper-V-All` if it was off beforehand.

Note: Requires administrator rights, a restart, and Windows Pro, Enterprise, or Education with hardware virtualization support.

### Turn on Windows Media Player and DirectPlay

Enables older Windows media playback components and the DirectPlay API required by older games and media software.

Changes: `DismEnableFeature WindowsMediaPlayer`, `MediaPlayback`, `DirectPlay`, and `LegacyComponents`.

Undo: Time Machine snapshot disables any of those features that were off beforehand.

Note: Requires administrator rights.

### Turn on Windows Subsystem for Linux (WSL)

Enables the Virtual Machine Platform and WSL Windows components for running Linux distributions on Windows.

Changes: `DismEnableFeature VirtualMachinePlatform`, `DismEnableFeature Microsoft-Windows-Subsystem-Linux`.

Undo: Time Machine snapshot disables any of those features that were off beforehand.

Note: Requires administrator rights and a restart.

### Turn on the NFS network client

Enables Windows Services for NFS client components, maps anonymous access to UID and GID 0, and configures client file permissions and authentication flavors.

Changes: `DismEnableFeature ServicesForNFS-ClientOnly`, `ClientForNFS-Infrastructure`, `NFS-Administration`; `HKLM\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Default\AnonymousUID = 0`, `AnonymousGID = 0`; `nfsadmin client stop/start` and `nfsadmin client localhost config fileaccess=755 SecFlavors=+sys -krb5 -krb5i`.

Undo: Time Machine snapshot disables features that were off beforehand and removes the two registry values; the `nfsadmin` configuration is not reverted automatically.

Note: Requires administrator rights. Available on Windows editions that include the NFS client components.

### Schedule daily registry backups

Turns on periodic Configuration Manager registry backups with two retained copies and creates a daily scheduled task at 00:30 to trigger Windows' idle registry backup task.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager\EnablePeriodicBackup = 1`, `BackupCount = 2`; daily `SYSTEM` task `\AutoRegBackup` running `schtasks /run /i /tn "\Microsoft\Windows\Registry\RegIdleBackup"`.

Undo: Time Machine snapshot removes both registry values and deletes `\AutoRegBackup`.

Note: Requires administrator rights.

### Turn on the F8 boot recovery menu

Switches the Windows boot manager to the `legacy` boot menu policy so pressing F8 during startup opens Advanced Boot Options.

Changes: `bcdedit /set bootmenupolicy legacy`.

Undo: Run `Turn off the F8 boot recovery menu` (Time Machine does not reverse `bcdedit` automatically).

Note: Requires administrator rights.

### Turn off the F8 boot recovery menu

Restores the Windows boot manager policy to standard mode.

Changes: `bcdedit /set bootmenupolicy standard`.

Undo: Run `Turn on the F8 boot recovery menu` (Time Machine does not reverse `bcdedit` automatically).

Note: Requires administrator rights.

### Turn on Windows Sandbox

Enables the Windows Sandbox feature for running isolated temporary desktop sessions.

Changes: `DismEnableFeature Containers-DisposableClientVM`.

Undo: Time Machine snapshot disables `Containers-DisposableClientVM` if it was off beforehand.

Note: Requires administrator rights, a restart, hardware virtualization, and Windows Pro, Enterprise, or Education.

## One-Click Fixes

### Synchronize system time with pool.ntp.org

Starts the Windows Time service, configures `pool.ntp.org,0x8` as the manual NTP peer, and forces an immediate clock resynchronization.

Changes: Starts service `w32time`; runs `w32tm /config /manualpeerlist:pool.ntp.org,0x8 /syncfromflags:manual /reliable:no /update` and `w32tm /resync /force`.

Undo: Not reversible automatically (reconfigure time sync in Windows Settings or via `w32tm`).

Note: Requires administrator rights.

### Run Sysinternals Autologon

Downloads `Autologon.exe` from `https://live.sysinternals.com/Autologon.exe`, verifies its Authenticode signature, and launches it with `/accepteula` to configure automatic sign-in.

Changes: Writes `%LocalAppData%\WindowCraft\Autologon.exe` and starts it; any autologon registry changes depend on your actions inside Autologon.

Undo: Not reversible by WindowCraft (disable autologon inside `Autologon.exe`).

### Reset Windows Update

Stops update services, clears BITS queue files, renames update caches to `.wcbak` backups, removes WSUS client identifiers, re-registers 15 update DLLs in-process, resets Winsock, restarts the services, and starts an update scan. The dropdown chooses `Standard` (renames `SoftwareDistribution\Download`) or `Aggressive` (also renames `SoftwareDistribution\DataStore` and `System32\catroot2`).

Changes: Deletes `%ProgramData%\Microsoft\Network\Downloader\qmgr*.dat`; renames `%SystemRoot%\SoftwareDistribution\Download` (and `DataStore` plus `System32\catroot2` in `Aggressive` mode) to `.wcbak`; removes `AccountDomainSid`, `PingID`, `SusClientId`, `SusClientIDValidation` under `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate`; calls `DllRegisterServer` on update DLLs; runs `netsh winsock reset` and `usoclient StartScan`.

Undo: Not reversible automatically.

Note: Requires administrator rights. The `.wcbak` backup folders can stay on disk using several gigabytes until deleted manually.

### Reset the network stack

Resets the Winsock catalog and both IPv4 and IPv6 TCP/IP stacks using `netsh`.

Changes: `netsh winsock reset`, `netsh int ipv4 reset`, `netsh int ipv6 reset`.

Undo: Not reversible automatically.

Note: Requires administrator rights and a restart for full effect.

### Run the DISM and SFC repair scan

Runs a three-stage scan of the system volume, protected Windows files, and the component store (`chkdsk`, `sfc`, and `DISM`).

Changes: `chkdsk %SystemDrive% /scan /perf`, `sfc /scannow`, `dism /online /cleanup-image /restorehealth`.

Undo: Not reversible.

Note: Requires administrator rights. Reports `needs attention` if any stage exits with a code other than `0`, `1`, `2`, or `3010`; exit codes `2` and `3010` indicate a restart is required.

### Repair WinGet

Re-registers the `Microsoft.DesktopAppInstaller` AppX package for the current user to restore the `winget` command.

Changes: `RegisterPackageByFamilyNameAsync` for `Microsoft.DesktopAppInstaller_8wekyb3d8bbwe` with `ForceApplicationShutdown`.

Undo: Not reversible automatically.

## PowerShell Profile and OpenSSH

### Install the WindowCraft PowerShell profile

Writes a built-in PowerShell 7 profile with PSReadLine history search (filtering out lines matching password/secret/token/apikey words), a prompt showing the git branch from `.git\HEAD` and `[admin]` status, and helpers (`which`, `ll`, `touch`, `mkcd`, `grep`, `df`, `uptime`, `flushdns`, `admin`, `reload`). Makes no network requests and saves any existing user profile as `.bak` once.

Changes: Writes `PowerShell\Microsoft.PowerShell_profile.ps1` inside the user's Documents folder (`FOLDERID_Documents`), moving a prior user profile to `Microsoft.PowerShell_profile.ps1.bak`.

Undo: Run `Remove the WindowCraft PowerShell profile`.

### Remove the WindowCraft PowerShell profile

Deletes the WindowCraft PowerShell profile and restores `Microsoft.PowerShell_profile.ps1.bak` if one was saved. Leaves any custom profile you wrote yourself after installing untouched.

Changes: Removes `PowerShell\Microsoft.PowerShell_profile.ps1` (if it starts with `# WindowCraft PowerShell profile`) and renames `.bak` back if present.

Undo: Run `Install the WindowCraft PowerShell profile`.

### Set up OpenSSH Server

Installs the OpenSSH Server Windows capability, sets `sshd` and `ssh-agent` to `Automatic` and starts them, adds an inbound TCP port 22 firewall rule, and merges `%USERPROFILE%\.ssh\authorized_keys` into `%ProgramData%\ssh\administrators_authorized_keys` locked to `SYSTEM` and `Administrators`.

Changes: `DismAddCapability OpenSSH.Server~~~~0.0.1.0`; sets `sshd` and `ssh-agent` startup to `Automatic` and starts them; creates firewall rule `WindowCraft OpenSSH Server (TCP 22)`; writes `%ProgramData%\ssh\administrators_authorized_keys` with SDDL `D:P(A;;FA;;;S-1-5-18)(A;;FA;;;S-1-5-32-544)`.

Undo: Time Machine rollback removes only the firewall rule `WindowCraft OpenSSH Server (TCP 22)`; the OpenSSH capability, services, and `administrators_authorized_keys` remain.

Note: Requires administrator rights.

## Classic Windows Panels

Every panel entry calls `ShellExecuteExW` (`control.exe` for `.cpl` applets) to open the corresponding Windows administrative tool without modifying system configuration.

| Panel | Opens |
| --- | --- |
| Computer Management | `compmgmt.msc` |
| Control Panel | `control` |
| Mouse Properties | `control.exe main.cpl` |
| Network Connections | `control.exe ncpa.cpl` |
| Power options | `control.exe powercfg.cpl` |
| Devices and Printers | `shell:::{A8A91A66-3A7D-4424-8D24-04E180695C7A}` |
| Programs and Features | `control.exe appwiz.cpl` |
| Region | `control.exe intl.cpl` |
| Security and Maintenance | `control.exe wscui.cpl` |
| Sound | `control.exe mmsys.cpl` |
| System Properties | `control.exe sysdm.cpl` |
| Date and Time | `control.exe timedate.cpl` |
| Windows Defender Firewall | `control.exe firewall.cpl` |
| System Restore | `rstrui.exe` |
