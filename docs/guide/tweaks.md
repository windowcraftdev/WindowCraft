<!-- Made by Nordan -->

# Tweaks & Privacy

Tweaks & Privacy configures Windows registry policies, services, power plans, DNS settings, and preinstalled AppX packages across the `Essential`, `Advanced`, `Preferences`, and `AppX Packages` tabs.

## Page controls and presets

### Minimal

Clears the current tweak checkboxes and queues the 6 `Minimal` preset tweaks from `preset.json`: `Create a restore point`, `Turn off Windows telemetry`, `Turn off activity history`, `Turn off location tracking`, `Stop automatic installs of promoted apps`, and `Block device metadata downloads`.

Undo: Click `Clear` or uncheck individual tweaks.

### Standard

Clears the current tweak checkboxes and queues the 13 `Standard` preset tweaks from `preset.json`: all 6 `Minimal` tweaks plus `Turn off Delivery Optimization peer sharing`, `Block Microsoft Store suggestions in Start search`, `Turn off folder type discovery in File Explorer`, `Turn off Windows Platform Binary Table execution`, `Set services to manual start`, `Show End task in the taskbar menu`, and `Delete temporary files`.

Undo: Click `Clear` or uncheck individual tweaks.

### Advanced

Clears the current tweak checkboxes and queues the 21 `Advanced` preset tweaks from `preset.json`: all 13 `Standard` tweaks plus `Remove Widgets`, `Remove Windows AI components`, `Turn off Microsoft Edge promotions and diagnostic data`, `Stop packaged apps from running in the background`, `Restore the classic right-click menu`, `Hide Home and Gallery in File Explorer`, `Block automatic Razer software installation`, and `Block Logitech Download Assistant`.

Undo: Click `Clear` or uncheck individual tweaks.

### Detect Applied

Reads the live registry and Service Control Manager state for every detectable tweak and ticks every `check` tweak that is currently fully applied.

Undo: Click `Clear` to untick the queued tweaks.

### Clear

Clears all queued checkboxes on `Essential` and `Advanced`, plus any pending imported toggles or segmented choices.

### Default Removals

Ticks the 14 default AppX packages in `preset.json` (`appx_default`) on the `AppX Packages` tab: `Feedback Hub`, `Get Help`, `Microsoft 365`, `Clipchamp`, `News`, `Weather`, `Bing Search`, `Start Experiences App`, `Solitaire Collection`, `Power Automate`, `Dev Home`, `To Do`, `Copilot`, and `Quick Assist`.

Undo: Untick individual packages.

### Import

Opens a file picker for WindowCraft `profile.json` or earlier WinUtil JSON configurations and queues matching applications, tweaks, toggles, segmented choices, optional features, AppX removals, and DNS selection. If the file sets an update mode or custom service rules, they stay queued for export or CLI `windowcraft.exe --config <path> --run` instead of applying from this page.

Undo: `Clear` removes only the queued tweaks and imported switches and choices. Imported applications, optional features, AppX removals and the DNS choice stay queued; untick them on their own pages.

### Export

Writes the current queued selections (tweaks, apps, features, AppX removals, non-default segmented options, DNS choice, and any imported update mode or service rules) to `%LocalAppData%\WindowCraft\profile.json` and copies the `windowcraft.exe --config ... --run` command to the clipboard.

Changes: `%LocalAppData%\WindowCraft\profile.json` and clipboard text.

Undo: Delete `%LocalAppData%\WindowCraft\profile.json`.

### Run Tweaks

Applies all queued checkbox tweaks and any imported toggles or segmented choices inside one [Time Machine](time-machine.md) snapshot session. Prompts for confirmation if any queued tweak is irreversible.

Changes: Applies each queued item; see individual entries below.

Undo: Time Machine snapshot for reversible items.

### Install

Available in the bottom action bar on the `AppX Packages` tab. Runs `winget install` for every ticked AppX package that has a Microsoft Store ID, leaving packages without a Store ID ticked.

Changes: Installs selected Microsoft Store packages via `winget`.

Undo: Tick the package and click `Remove`.

### Remove

Available in the bottom action bar on the `AppX Packages` tab. Removes every ticked AppX package for all users and deprovisions its family so new user accounts do not receive it.

Changes: `PackageManager.RemovePackageWithOptionsAsync` (`RemoveForAllUsers`) and `DeprovisionPackageForAllUsersAsync` for each ticked package.

Undo: Tick the package and click `Install` if it has a Microsoft Store ID; packages without a Store ID cannot be reinstalled from here.

## Essential

### Turn off activity history

Stops Windows from recording and uploading user activity history while keeping the activity feed active for clipboard history.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\EnableActivityFeed = 1`, `PublishUserActivities = 0`, `UploadUserActivities = 0`.

Undo: Time Machine snapshot or X-Ray Inspector (removes all three values).

Note: Requires administrator rights.

### Turn off hibernation

Disables system hibernation, deletes `hiberfil.sys` to free disk space, and hides the Hibernate option from the power menu.

Changes: `HKLM\System\CurrentControlSet\Control\Session Manager\Power\HibernateEnabled = 0`, `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings\ShowHibernateOption = 0`, and `powercfg /hibernate off`.

Undo: Time Machine snapshot or X-Ray Inspector (restores both values to `1` and runs `powercfg /hibernate on` when firmware supports S4 sleep).

Note: Requires administrator rights.

### Remove Widgets

Stops Widget processes, uninstalls the Widgets runtime and Windows Web Experience packages for all users, and restarts File Explorer. The Widgets button and board no longer run.

Changes: Terminates `*Widget*` processes, removes `Microsoft.WidgetsPlatformRuntime` and `MicrosoftWindows.Client.WebExperience` AppX packages, and restarts `explorer.exe`.

Undo: Time Machine snapshot or X-Ray Inspector (runs `winget install 9MSSGKG348SP`).

Note: Requires administrator rights.

### Restore the previous Start menu layout

Sets a feature management override so Windows uses the earlier Start menu layout.

Changes: `HKLM\SYSTEM\ControlSet001\Control\FeatureManagement\Overrides\8\3036241548\EnabledState = 1` after checking that `KB5089573` is absent.

Undo: Time Machine snapshot or X-Ray Inspector (removes `EnabledState`).

Note: Requires administrator rights. Has no effect on builds that include `KB5089573` or later; WindowCraft refuses to apply this tweak if `KB5089573` is installed.

### Block Microsoft Store suggestions in Start search

Denies access to the Microsoft Store local `store.db` file so Start menu searches stop showing suggested Store apps.

Changes: `SetNamedSecurityInfoW` `DENY` `Full` for `S-1-1-0` (Everyone) on `%LOCALAPPDATA%\Packages\Microsoft.WindowsStore_8wekyb3d8bbwe\LocalState\store.db`.

Undo: Time Machine snapshot or X-Ray Inspector (clears the `DENY` rule for `S-1-1-0`).

### Turn off location tracking

Denies system-wide location access, disables sensor permissions and automatic offline map updates, and disables the Geolocation service.

Changes: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location\Value = "Deny"`, `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}\SensorPermissionState = 0`, `HKLM\SYSTEM\Maps\AutoUpdateEnabled = 0`, and service `lfsvc` set to `Disabled`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `"Allow"`, `1`, `1`, and `lfsvc` `Manual`).

Note: Requires administrator rights. Apps and system features that rely on Windows location services stop receiving coordinates.

### Set services to manual start

Sets `MapsBroker` and `StorSvc` to manual start and disables `CscService`, `DiagTrack`, and `SharedAccess`. These background services no longer start automatically at boot unless requested.

Changes: Services `CscService` (`Disabled`, optional), `DiagTrack` (`Disabled`), `MapsBroker` (`Manual`), `StorSvc` (`Manual`), and `SharedAccess` (`Disabled`).

Undo: Time Machine snapshot or X-Ray Inspector (restores prior startup types).

Note: Requires administrator rights. Disabling `SharedAccess` turns off Internet Connection Sharing; disabling `CscService` turns off Offline Files.

### Stop automatic installs of promoted apps

Sets the Windows CloudContent policy so Windows stops installing promoted Microsoft Store apps automatically.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent\DisableWindowsConsumerFeatures = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (removes `DisableWindowsConsumerFeatures`).

Note: Requires administrator rights.

### Turn off Windows telemetry

Turns off diagnostic data collection, advertising ID, online speech privacy, typing personalization, feedback prompts, PowerShell telemetry, and Windows Defender sample submission, and disables `diagtrack` and `WerSvc`.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo\Enabled = 0`, `...\Privacy\TailoredExperiencesWithDiagnosticDataEnabled = 0`, `HKCU\Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy\HasAccepted = 0`, `HKCU\Software\Microsoft\Input\TIPC\Enabled = 0`, `HKCU\Software\Microsoft\InputPersonalization\RestrictImplicitInkCollection = 1`, `RestrictImplicitTextCollection = 1`, `...\TrainedDataStore\HarvestContacts = 0`, `HKCU\Software\Microsoft\Personalization\Settings\AcceptedPrivacyPolicy = 0`, `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\Start_TrackProgs = 0`, `HKCU\Software\Microsoft\Siuf\Rules\NumberOfSIUFInPeriod = 0` (removes `PeriodInNanoSeconds`), `HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection\AllowTelemetry = 0`, `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\PublishUserActivities = 0`, `HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Spynet\SubmitSamplesConsent = 2`, `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment\POWERSHELL_TELEMETRY_OPTOUT = "1"`, services `diagtrack` (`Disabled`) and `WerSvc` (`Disabled`), and broadcasts `WM_SETTINGCHANGE` (`Environment`).

Undo: Time Machine snapshot or X-Ray Inspector (restores previous registry values and service startup types and broadcasts `Environment`).

Note: Requires administrator rights.

### Turn off Delivery Optimization peer sharing

Sets Delivery Optimization download mode to `0` so Windows downloads updates only from Microsoft servers without peer-to-peer sharing.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization\DODownloadMode = 0`.

Undo: Time Machine snapshot or X-Ray Inspector (removes `DODownloadMode`).

Note: Requires administrator rights.

### Turn off BitLocker

Starts background decryption of the system drive via WMI (`Win32_EncryptableVolume.Decrypt`). The drive stays usable while decryption runs.

Changes: Decrypts the `%SystemDrive%` volume through `ROOT\CIMV2\Security\MicrosoftVolumeEncryption`.

Undo: Not reversible automatically (re-enable encryption from Windows Settings, Privacy & security, Device encryption).

Note: Requires administrator rights.

### Create a restore point

Removes the 1440-minute cooldown between restore points, enables System Protection on the system drive, and creates a `WindowCraft Automatic Restore Point`.

Changes: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore\SystemRestorePointCreationFrequency = 0`, `EnableSR`, and `SRSetRestorePointW`.

Undo: Time Machine snapshot reverts `SystemRestorePointCreationFrequency` to `1440`; the created restore point itself is not deleted automatically.

Note: Requires administrator rights.

### Show End task in the taskbar menu

Adds `End task` to the taskbar right-click context menu for running windows.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings\TaskbarEndTask = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (removes `TaskbarEndTask`).

### Turn off Windows Platform Binary Table execution

Prevents Windows from executing firmware-embedded binaries from the ACPI WPBT table during startup.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\DisableWpbtExecution = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (removes `DisableWpbtExecution`).

Note: Requires administrator rights.

### Block device metadata downloads

Stops Windows from downloading device metadata and companion apps when new hardware is connected.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\Device Metadata\PreventDeviceMetadataFromNetwork = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (removes `PreventDeviceMetadataFromNetwork`).

Note: Requires administrator rights.

### Run Disk Cleanup and component store reset

Runs automated Disk Cleanup on the system drive and removes superseded WinSxS component versions using DISM.

Changes: `cleanmgr.exe /d %SystemDrive% /VERYLOWDISK` and `Dism.exe /online /Cleanup-Image /StartComponentCleanup /ResetBase`.

Undo: Not reversible (deleted superseded files and component backups cannot be restored).

Note: Requires administrator rights. Because `/ResetBase` runs, updates installed so far can no longer be uninstalled.

### Delete temporary files

Deletes unlocked files and folders inside the user `%TEMP%` directory and `%SystemRoot%\Temp` without following junctions or symbolic links.

Changes: Empties `%TEMP%` and `%SystemRoot%\Temp` (skipping locked files).

Undo: Not reversible.

Note: Requires administrator rights to clear `%SystemRoot%\Temp`.

### Turn off folder type discovery in File Explorer

Deletes saved File Explorer folder view bags and sets the default folder type to `NotSpecified` so File Explorer stops scanning folders to guess media templates.

Changes: Deletes `HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags` and `...\BagMRU`, then sets `...\Bags\AllFolders\Shell\FolderType = "NotSpecified"`.

Undo: Time Machine snapshot or X-Ray Inspector removes `FolderType`; previously deleted `Bags` and `BagMRU` folder views are not restored.

Note: Sign out or restart Windows for the change to take effect.

## Advanced

Tweaks in `Advanced` display a `Caution` badge in the interface.

### Turn off svchost.exe process splitting

Sets the `svchost.exe` split threshold to total installed physical memory in KB so Windows groups services into fewer `svchost.exe` processes.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\SvcHostSplitThresholdInKB = {ram_kb}`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `3670016`).

Note: Requires administrator rights and a Windows restart.

### Turn off Brave browser add-ons and reporting

Applies Group Policy settings for Brave to disable Brave Rewards, Wallet, VPN, AI chat, News, Talk, Tor, stats pings, P3A, anonymized URL collection, Safe Browsing extended reporting, and metrics reporting.

Changes: Writes 12 `DWord` policies under `HKLM\SOFTWARE\Policies\BraveSoftware\Brave`: `BraveRewardsDisabled = 1`, `BraveWalletDisabled = 1`, `BraveVPNDisabled = 1`, `BraveAIChatEnabled = 0`, `BraveStatsPingEnabled = 0`, `BraveNewsDisabled = 1`, `BraveTalkDisabled = 1`, `TorDisabled = 1`, `BraveP3AEnabled = 0`, `UrlKeyedAnonymizedDataCollectionEnabled = 0`, `SafeBrowsingExtendedReportingEnabled = 0`, `MetricsReportingEnabled = 0`.

Undo: Time Machine snapshot or X-Ray Inspector (removes all 12 values).

Note: Requires administrator rights.

### Suppress warnings for unsigned RDP files

Skips security prompts when launching unsigned `.rdp` Remote Desktop connection files.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services\Client\RedirectionWarningDialogVersion = 1` and `HKCU\SOFTWARE\Microsoft\Terminal Server Client\RdpLaunchConsentAccepted = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (removes both values).

Note: Requires administrator rights for the `HKLM` policy key.

### Turn off Microsoft Edge promotions and diagnostic data

Applies Microsoft Edge Group Policy settings to disable desktop shortcuts, diagnostic data, personalization reporting, recommendations, first-run experience, feedback, Collections, Shopping Assistant, Insider promotions, Microsoft Rewards, Web Widget, Asset Delivery, Wallet donations, default browser prompts, block an extension ID, and enable Do Not Track.

Changes: Writes `HKLM\SOFTWARE\Policies\Microsoft\EdgeUpdate\CreateDesktopShortcutDefault = 0`, `HKLM\SOFTWARE\Policies\Microsoft\Edge\ExtensionInstallBlocklist\1 = "ofefcgjbeghpigppfmkologfjadafddi"`, and 15 `DWord` values under `HKLM\SOFTWARE\Policies\Microsoft\Edge` (`PersonalizationReportingEnabled = 0`, `ShowRecommendationsEnabled = 0`, `HideFirstRunExperience = 1`, `UserFeedbackAllowed = 0`, `ConfigureDoNotTrack = 1`, `AlternateErrorPagesEnabled = 0`, `EdgeCollectionsEnabled = 0`, `EdgeShoppingAssistantEnabled = 0`, `MicrosoftEdgeInsiderPromotionEnabled = 0`, `ShowMicrosoftRewards = 0`, `WebWidgetAllowed = 0`, `DiagnosticData = 0`, `EdgeAssetDeliveryServiceEnabled = 0`, `WalletDonationEnabled = 0`, `DefaultBrowserSettingsCampaignEnabled = 0`).

Undo: Time Machine snapshot or X-Ray Inspector (removes all 17 values).

Note: Requires administrator rights.

### Remove Microsoft Edge

Removes Edge for all users and deletes its profile data (bookmarks, history, saved passwords) for good. Creates a zero-byte marker file in the old Edge path so the official Edge installer permits system-level removal, then runs `setup.exe --uninstall --system-level --force-uninstall --delete-profile`.

Changes: Writes `%SystemRoot%\SystemApps\Microsoft.MicrosoftEdge_8wekyb3d8bbwe\MicrosoftEdge.exe` and runs Edge `setup.exe` uninstaller.

Undo: Rolling back the session in Time Machine runs `winget install Microsoft.Edge` and marks the session partly rolled back. That brings back the browser only; the deleted profile data cannot be restored. X-Ray Inspector shows no undo button for this tweak.

Note: Requires administrator rights. `Run Tweaks` asks for confirmation first.

### Use UTC for the hardware clock

Configures Windows to treat the motherboard real-time clock as Coordinated Universal Time (UTC) instead of local time, preventing clock jumps when dual-booting Linux or macOS.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\TimeZoneInformation\RealTimeIsUniversal = 1` (`QWord`).

Undo: Time Machine snapshot or X-Ray Inspector (restores `0`).

Note: Requires administrator rights.

### Remove OneDrive

Disables `OneSyncSvc`, shields the user's `OneDrive` folder with a temporary `DENY` delete ACL while running `OneDriveSetup.exe /uninstall`, terminates `FileCoAuth` and `explorer`, cleans leftover OneDrive app folders, deletes the `OneDrive` folder only if empty, and restarts File Explorer.

Changes: Service `OneSyncSvc` (`Disabled`, optional); runs `OneDriveSetup.exe /uninstall`; deletes `%LOCALAPPDATA%\Microsoft\OneDrive` and `%ProgramData%\Microsoft OneDrive`; removes `HKCU\Environment\OneDrive` if the user OneDrive folder was empty; restarts `explorer.exe`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `OneSyncSvc` startup and runs `winget install Microsoft.OneDrive`).

Note: Requires administrator rights.

### Hide Home and Gallery in File Explorer

Unpins the Home and Gallery navigation nodes in File Explorer and sets File Explorer to open directly to This PC.

Changes: `HKCU\Software\Classes\CLSID\{f874310e-b6b7-47dc-bc84-b9e6b38f5903}\System.IsPinnedToNameSpaceTree = 0`, `HKCU\Software\Classes\CLSID\{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}\System.IsPinnedToNameSpaceTree = 0`, and `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\LaunchTo = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (removes all three values).

### Set visual effects for performance

Disables window drag-contents, shortens the menu show delay from 400 to 200 ms, disables minimize/maximize animations, list-view shadows and alpha selection, taskbar animations, Aero Peek, and Chat taskbar icon, and sets keyboard delay to 0.

Changes: `HKCU\Control Panel\Desktop` (`DragFullWindows = "0"`, `MenuShowDelay = "200"`, `UserPreferencesMask = 9012038010000000`), `...\WindowMetrics\MinAnimate = "0"`, `HKCU\Control Panel\Keyboard\KeyboardDelay = "0"`, `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced` (`ListviewAlphaSelect = 0`, `ListviewShadow = 0`, `TaskbarAnimations = 0`, `TaskbarMn = 0`), `...\Explorer\VisualEffects\VisualFXSetting = 3`, and `HKCU\Software\Microsoft\Windows\DWM\EnableAeroPeek = 0`.

Undo: Time Machine snapshot or X-Ray Inspector (restores previous values and removes `UserPreferencesMask`).

### Turn off reserved storage

Disables the disk space Windows reserves for updates and servicing via DISM.

Changes: `DismSetReservedStorageState(false)`.

Undo: Time Machine snapshot or X-Ray Inspector (`DismSetReservedStorageState(true)`).

Note: Requires administrator rights.

### Turn off Storage Sense

Disables automatic disk cleanup by Storage Sense for the current user.

Changes: `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy\01 = 0`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `1`).

### Remove Windows AI components

Hides the AI components page in Windows Settings, disables AI in Notepad, disables `WSAIFabricSvc`, registers CoreAI under `EndOfLife`, uninstalls Copilot, Microsoft 365 Hub, and CoreAI AppX packages, and disables the Recall feature.

Changes: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\SettingsPageVisibility = "hide:aicomponents"`, `HKLM\SOFTWARE\Policies\WindowsNotepad\DisableAIFeatures = 1`, service `WSAIFabricSvc` (`Disabled`, optional), `AppxAllUserStore\EndOfLife` key for `MicrosoftWindows.Client.CoreAI`, removes `*Copilot*`, `Microsoft.MicrosoftOfficeHub`, and `MicrosoftWindows.Client.CoreAI`, and runs `DismDisableFeature Recall`.

Undo: Not reversible automatically.

Note: Requires administrator rights.

### Block automatic Razer software installation

Disables automatic Windows Update driver searches and device co-installers, clears `%SystemRoot%\Installer\Razer`, and denies write permissions for Everyone (`S-1-1-0`) on that folder.

Changes: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching\SearchOrderConfig = 0`, `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Device Installer\DisableCoInstallers = 1`, purges `%SystemRoot%\Installer\Razer`, and denies `Write` for `S-1-1-0`.

Undo: Time Machine snapshot or X-Ray Inspector (restores both registry values and removes the `DENY` ACE on `%SystemRoot%\Installer\Razer`).

Note: Requires administrator rights. Emptied installer files inside `%SystemRoot%\Installer\Razer` are not restored; disabling `SearchOrderConfig` and `DisableCoInstallers` affects driver/co-installer behavior for other hardware too.

### Block Logitech Download Assistant

Terminates `logi_download_assistant`, clears `%ProgramW6432%\LogiDownloadAssistant`, and denies write permissions for Everyone (`S-1-1-0`) on that folder.

Changes: Terminates `logi_download_assistant`, purges `%ProgramW6432%\LogiDownloadAssistant`, and denies `Write` for `S-1-1-0`.

Undo: Time Machine snapshot or X-Ray Inspector (removes the `DENY` ACE on `%ProgramW6432%\LogiDownloadAssistant`).

Note: Requires administrator rights. Emptied files inside `%ProgramW6432%\LogiDownloadAssistant` are not restored.

### Turn off Notification Center and toast popups

Disables the taskbar Notification Center flyout and desktop toast notifications for the current user.

Changes: `HKCU\Software\Policies\Microsoft\Windows\Explorer\DisableNotificationCenter = 1` and `HKCU\Software\Microsoft\Windows\CurrentVersion\PushNotifications\ToastEnabled = 0`.

Undo: Time Machine snapshot or X-Ray Inspector (removes `DisableNotificationCenter` and restores `ToastEnabled = 1`).

### Block Adobe activation and tracking hosts

Downloads a commit-pinned, SHA-256-verified Adobe blocklist from GitHub, writes it inside a fenced `ADOBE` section of the system `hosts` file, and flushes the DNS resolver cache.

Changes: `%SystemRoot%\System32\drivers\etc\hosts` (`ADOBE` section) and `DnsFlushResolverCache`.

Undo: Time Machine snapshot or X-Ray Inspector (removes the `ADOBE` section from `hosts` and flushes DNS cache).

Note: Requires administrator rights and internet access on apply.

### Restore the classic right-click menu

Registers an empty in-process COM override for the Windows 11 modern context menu CLSID and restarts File Explorer so right-clicking shows the classic full menu directly.

Changes: `HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32` default value set to `""`, and restarts `explorer.exe`.

Undo: Time Machine snapshot or X-Ray Inspector (deletes the CLSID key tree and restarts `explorer.exe`).

### Prefer IPv4 over IPv6

Sets `Tcpip6\Parameters\DisabledComponents` to `32` (`0x20`) so Windows prefers IPv4 connections over IPv6 while keeping IPv6 enabled. Mutually exclusive with `Turn off Teredo tunneling` and `Turn off IPv6` in the selection queue.

Changes: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters\DisabledComponents = 32`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `0`).

Note: Requires administrator rights.

### Turn off Teredo tunneling

Sets `Tcpip6\Parameters\DisabledComponents` to `1` and disables the Teredo transition interface with `netsh`. Mutually exclusive with `Prefer IPv4 over IPv6` and `Turn off IPv6` in the selection queue.

Changes: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters\DisabledComponents = 1` and `netsh interface teredo set state disabled`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `0` and runs `netsh interface teredo set state default`).

Note: Requires administrator rights.

### Turn off IPv6

Sets `Tcpip6\Parameters\DisabledComponents` to `255` (`0xFF`) and unbinds `ms_tcpip6` from network adapters through WMI (`MSFT_NetAdapterBindingSettingData.Disable`). Mutually exclusive with `Prefer IPv4 over IPv6` and `Turn off Teredo tunneling` in the selection queue.

Changes: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters\DisabledComponents = 255` and unbinds `ms_tcpip6`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `0` and re-enables `ms_tcpip6`).

Note: Requires administrator rights and a Windows restart.

### Stop packaged apps from running in the background

Prevents Microsoft Store / UWP apps from running background tasks for the current user.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications\GlobalUserDisabled = 1`.

Undo: Time Machine snapshot or X-Ray Inspector (restores `0`).

### Run O&O ShutUp10++

Downloads `OOSU10.exe` from `https://dl5.oo-software.com/files/ooshutup10/OOSU10.exe` into `%LocalAppData%\WindowCraft\OOSU10.exe`, verifies its Authenticode signature, and launches it.

Changes: `%LocalAppData%\WindowCraft\OOSU10.exe` and any changes you apply inside O&O ShutUp10++.

Undo: Revert settings inside `OOSU10.exe` (not tracked by Time Machine).

### DNS server

Configures IPv4, IPv6, and DNS-over-HTTPS (`FALLBACK_TO_UDP` enabled) on all connected (`IfOperStatusUp`) adapters using `SetInterfaceDnsSettings`, then flushes the DNS cache. Options in the dropdown:

- `Default` / `DHCP`: clears static DNS servers on connected adapters so they use DHCP-assigned DNS.
- `Fastest`: benchmarks `Google` and `Cloudflare` (`benchmark: true` in `dns.json`) for 5 rounds over UDP, TCP, and DoH, then applies the fastest responding provider.
- `Google`: IPv4 `8.8.8.8`, `8.8.4.4`; IPv6 `2001:4860:4860::8888`, `2001:4860:4860::8844`; DoH `https://dns.google/dns-query`.
- `Cloudflare`: IPv4 `1.1.1.1`, `1.0.0.1`; IPv6 `2606:4700:4700::1111`, `2606:4700:4700::1001`; DoH `https://cloudflare-dns.com/dns-query`.
- `Cloudflare_Malware`: IPv4 `1.1.1.2`, `1.0.0.2`; IPv6 `2606:4700:4700::1112`, `2606:4700:4700::1002`; DoH `https://security.cloudflare-dns.com/dns-query`.
- `Cloudflare_Malware_Adult`: IPv4 `1.1.1.3`, `1.0.0.3`; IPv6 `2606:4700:4700::1113`, `2606:4700:4700::1003`; DoH `https://family.cloudflare-dns.com/dns-query`.
- `Open_DNS`: IPv4 `208.67.222.222`, `208.67.220.220`; IPv6 `2620:119:35::35`, `2620:119:53::53`; DoH `https://doh.opendns.com/dns-query`.
- `Quad9`: IPv4 `9.9.9.9`, `149.112.112.112`; IPv6 `2620:fe::fe`, `2620:fe::9`; DoH `https://dns.quad9.net/dns-query`.
- `AdGuard_Ads_Trackers`: IPv4 `94.140.14.14`, `94.140.15.15`; IPv6 `2a10:50c0::ad1:ff`, `2a10:50c0::ad2:ff`; DoH `https://dns.adguard-dns.com/dns-query`.
- `AdGuard_Ads_Trackers_Malware_Adult`: IPv4 `94.140.14.15`, `94.140.15.16`; IPv6 `2a10:50c0::bad1:ff`, `2a10:50c0::bad2:ff`; DoH `https://family.adguard-dns.com/dns-query`.

Changes: Adapter IPv4, IPv6, and DoH DNS configuration via `SetInterfaceDnsSettings`, plus `DnsFlushResolverCache`.

Undo: Select `Default` or `DHCP` in the dropdown (not recorded in Time Machine snapshots).

Note: Requires administrator rights.

## Preferences

All `Preferences` items apply immediately when switched or selected and create a [Time Machine](time-machine.md) snapshot.

### Detailed stop error screen

Shows bugcheck parameters on the stop error screen and hides the sad face emoticon.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\CrashControl\DisplayParameters = 1` (off: `0`) and `DisableEmoticon = 1` (off: `0`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights.

### Battery percentage on the taskbar

Displays the battery charge percentage next to the system tray battery icon.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\IsBatteryPercentageEnabled = 1` (off: `<RemoveEntry>`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

### Dark theme

Switches Windows applications and shell surfaces between dark and light themes and broadcasts a shell theme refresh.

Changes: `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize\AppsUseLightTheme = 0` (off: `1`), `SystemUsesLightTheme = 0` (off: `1`), and broadcasts `WM_SETTINGCHANGE` (`ImmersiveColorSet`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

### File name extensions in File Explorer

Shows file suffixes (`.exe`, `.txt`, and others) on all files in File Explorer and restarts File Explorer.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\HideFileExt = 0` (off: `1`) and restarts `explorer.exe`.

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

### Hidden files and folders in File Explorer

Shows files and directories with the hidden attribute in File Explorer and restarts File Explorer.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\Hidden = 1` (off: `0`) and restarts `explorer.exe`.

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

### Detailed startup and shutdown status messages

Displays verbose service and policy progress messages during startup, sign-in, sign-out, and shutdown.

Changes: `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\VerboseStatus = 1` (off: `0`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights.

### New Outlook for Windows

Controls whether Office 16 uses the new Outlook app and whether the switch toggle appears inside classic Outlook.

Changes: `HKCU\SOFTWARE\Microsoft\Office\16.0\Outlook\Preferences\UseNewOutlook = 1` (off: `0`), `HKCU\Software\Microsoft\Office\16.0\Outlook\Options\General\HideNewOutlookToggle = 0` (off: `1`), `HKCU\Software\Policies\Microsoft\Office\16.0\Outlook\Options\General\DoNewOutlookAutoMigration` removed (off: `0`, which stops the automatic move to the new Outlook), and `HKCU\Software\Policies\Microsoft\Office\16.0\Outlook\Preferences\NewOutlookMigrationUserSetting = 0` (off: `<RemoveEntry>`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Always show scrollbars

Keeps UWP/XAML scrollbars at full width instead of auto-collapsing when the pointer moves away.

Changes: `HKCU\Control Panel\Accessibility\DynamicScrollbars = 0` (off: `1`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

### Multiplane overlay

Controls Desktop Window Manager Multiplane Overlay (MPO) using a three-option selector:

- `Enabled`: removes `OverlayTestMode` and `DisableOverlays` to use Windows' default behavior.
- `Disabled (Compatibility)`: sets `OverlayTestMode = 5` and removes `DisableOverlays`.
- `Fully Disabled`: sets `OverlayTestMode = 5` and `DisableOverlays = 1`.

Changes: `HKLM\SOFTWARE\Microsoft\Windows\Dwm\OverlayTestMode` and `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\DisableOverlays`.

Undo: Select `Enabled`, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights.

### Mouse acceleration

Controls pointer acceleration speed and movement thresholds.

Changes: `HKCU\Control Panel\Mouse\MouseSpeed = "1"` (off: `"0"`), `MouseThreshold1 = "6"` (off: `"0"`), and `MouseThreshold2 = "10"` (off: `"0"`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Num Lock at startup

Controls whether Num Lock turns on automatically on the sign-in screen and after signing in.

Changes: `HKU\.Default\Control Panel\Keyboard\InitialKeyboardIndicators = "2"` (off: `"0"`) and `HKCU\Control Panel\Keyboard\InitialKeyboardIndicators = "2"` (off: `"0"`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights for `HKU\.Default`.

### Window snapping

Controls Snap Windows drag-to-edge and corner tiling behavior.

Changes: `HKCU\Control Panel\Desktop\WindowArrangementActive = "1"` (off: `"0"`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Network connectivity during Modern Standby

Keeps network adapters connected on AC power while the computer is in S0 Modern Standby.

Changes: `HKCU\SOFTWARE\Policies\Microsoft\Power\PowerSettings\f15576e8-98b7-4186-b944-eafa664402d9\ACSettingIndex = 1` (off: `0`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Traditional S3 sleep mode

Sets `PlatformAoAcOverride = 0` to disable S0 Modern Standby and request traditional S3 suspend-to-RAM.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\Power\PlatformAoAcOverride = 0` (off: `<RemoveEntry>`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights and hardware/BIOS support for S3 sleep.

### Home page in Settings

Controls whether the Home landing page is shown or hidden in Windows Settings.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer\SettingsPageVisibility = "show:home"` (off: `"hide:home"`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Bing web results in Start menu search

Controls whether Start menu and taskbar search include Bing web results.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Search\BingSearchEnabled = 1` (off: `0`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Sign-in screen background blur

Controls the acrylic blur effect behind the sign-in screen background.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\DisableAcrylicBackgroundOnLogon = 0` (off: `1`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights.

### Skip the lock screen

Hides the Windows lock screen so startup and wake go directly to the sign-in prompt.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\Personalization\NoLockScreen = 1` (off: `<RemoveEntry>`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights.

### Recommended section in the Start menu

Shows or hides the Recommended section in the Start menu and restarts File Explorer.

Changes: `HKLM\SOFTWARE\Microsoft\PolicyManager\current\device\Start\HideRecommendedSection = 0` (off: `1`), `HKLM\SOFTWARE\Microsoft\PolicyManager\current\device\Education\IsEducationEnvironment = 0` (off: `1`), `HKLM\SOFTWARE\Policies\Microsoft\Windows\Explorer\HideRecommendedSection = 0` (off: `1`), and restarts `explorer.exe`.

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights. Turning this off also disables lock screen Windows Spotlight.

### Sticky Keys shortcut

Controls the accessibility prompt triggered by pressing Shift five times.

Changes: `HKCU\Control Panel\Accessibility\StickyKeys\Flags = 506` (off: `58`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Centered taskbar icons

Centers taskbar icons when on, aligns them to the left edge when off, and restarts File Explorer.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarAl = 1` (off: `0`) and restarts `explorer.exe`.

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Search button on the taskbar

Shows or hides the search icon on the Windows taskbar.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Search\SearchboxTaskbarMode = 1` (off: `0`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Task View button on the taskbar

Shows or hides the Task View virtual desktop button on the taskbar.

Changes: `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\ShowTaskViewButton = 1` (off: `0`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Game Mode

Turns Windows Game Mode on or off for the current user.

Changes: `HKCU\Software\Microsoft\GameBar\AllowAutoGameMode = 1` (off: `0`) and `AutoGameModeEnabled = 1` (off: `0`).

Undo: Toggle the switch, or use Time Machine or X-Ray Inspector.

### Win32 file paths over 260 characters

Removes the 260-character `MAX_PATH` limit for Win32 applications that declare long-path support.

Changes: `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\LongPathsEnabled = 1` (off: `0`).

Undo: Turn off the switch, or use Time Machine or X-Ray Inspector.

Note: Requires administrator rights.

## Performance

Displayed at the bottom of the `Preferences` tab.

### Add the Ultimate Performance power plan

Duplicates Windows' built-in Ultimate Performance scheme template (`e9a42b02-d5df-448d-aa00-03f14749eb61`) if no copy exists yet and sets it as the active power scheme.

Changes: `PowerDuplicateScheme` and `PowerSetActiveScheme` for `Ultimate Performance`.

Undo: Click `Run` on `Remove the Ultimate Performance power plan` (Time Machine does not reverse this automatically).

### Remove the Ultimate Performance power plan

Switches to the `Balanced` power scheme (`381b4222-f694-41f0-9685-ff5bb260df2e`) if an Ultimate Performance copy is active and deletes every copied Ultimate Performance scheme.

Changes: `PowerSetActiveScheme` (`Balanced`) and `PowerDeleteScheme` for each copied Ultimate Performance plan.

Undo: Click `Run` on `Add the Ultimate Performance power plan` (Time Machine does not reverse this automatically).

## AppX Packages

Every removal on the `AppX Packages` tab calls `RemovePackageWithOptionsAsync` (`RemoveForAllUsers`) and `DeprovisionPackageForAllUsersAsync` for the package ID in the table; removals do not create Time Machine snapshots.

Undo: Tick the package and click `Install`, which runs `winget install <Store ID>`. Packages without a Store ID cannot be reinstalled from WindowCraft.

| Package | Package ID | Store ID | Note |
| --- | --- | --- | --- |
| Feedback Hub | `Microsoft.WindowsFeedbackHub` | `9NBLGGH4R32N` |  |
| Get Help | `Microsoft.GetHelp` | `9PKDZBMV1H3T` |  |
| Outlook for Windows | `Microsoft.OutlookForWindows` | `9NRX63209R7B` |  |
| Microsoft Teams | `MSTeams` | `XP8BT8DW290MPQ` |  |
| Clipchamp | `Clipchamp.Clipchamp` | `9P1J8S7CCWWT` |  |
| Microsoft 365 | `Microsoft.MicrosoftOfficeHub` | `9WZDNCRD29V9` |  |
| Media Player | `Microsoft.ZuneMusic` | `9WZDNCRFJ3PT` |  |
| Bing Search | `Microsoft.BingSearch` | `9NZBF4GT040C` |  |
| Quick Assist | `MicrosoftCorporationII.QuickAssist` | `9P7BP5VNWKX5` |  |
| Dev Home | `Microsoft.Windows.DevHome` | `9N8MHTPHNGVV` |  |
| Mobile Devices | `MicrosoftWindows.CrossDevice` | `9NTXGKQ8P7N0` | Removing this stops phone camera streaming, file sharing, and hotspot linking |
| To Do | `Microsoft.Todos` | `9NBLGGH5R558` |  |
| Power Automate | `Microsoft.PowerAutomateDesktop` | `9NFTCH6J7FHV` |  |
| Phone Link | `Microsoft.YourPhone` | `9NMPJ99VJBWV` |  |
| Sticky Notes | `Microsoft.MicrosoftStickyNotes` | `9NBLGGH4QGHW` |  |
| Sound Recorder | `Microsoft.WindowsSoundRecorder` | `9WZDNCRFHWKN` |  |
| Clock | `Microsoft.WindowsAlarms` | `9WZDNCRFJ3PR` |  |
| Paint | `Microsoft.Paint` | `9PCFS5B6T72H` |  |
| Notepad | `Microsoft.WindowsNotepad` | `9MSMLRH6LZF3` |  |
| Snipping Tool | `Microsoft.ScreenSketch` | `9MZ95KL8MR0L` |  |
| Copilot | `Microsoft.Copilot` | `9NHT9RB2F4HD` |  |
| Calculator | `Microsoft.WindowsCalculator` | `9WZDNCRFHVN5` |  |
| Camera | `Microsoft.WindowsCamera` | `9WZDNCRFJBBG` |  |
| Photos | `Microsoft.Windows.Photos` | `9WZDNCRFJBH4` |  |
| News | `Microsoft.BingNews` | `9WZDNCRFHVFW` |  |
| Weather | `Microsoft.BingWeather` | `9WZDNCRFJ3Q2` |  |
| Xbox App | `Microsoft.GamingApp` | `9MV0B5HZVK9Z` |  |
| Xbox Game Bar | `Microsoft.XboxGamingOverlay` | `9NZKPSTSNW4P` |  |
| Xbox Identity Provider | `Microsoft.XboxIdentityProvider` | `9WZDNCRD1HKW` | Removing this may break Microsoft account sign-in for non-Xbox games and apps that rely on this authentication pipeline |
| Xbox Speech To Text Overlay | `Microsoft.XboxSpeechToTextOverlay` | none |  |
| Xbox TCUI | `Microsoft.Xbox.TCUI` | none | Removing this may break Microsoft account authentication in games and apps that do not otherwise require the Xbox app |
| Start Experiences App | `Microsoft.StartExperiencesApp` | `9PC1H9VN18CM` |  |
| Solitaire Collection | `Microsoft.MicrosoftSolitaireCollection` | none |  |
| Movies & TV | `Microsoft.ZuneVideo` | `9WZDNCRFJ3P2` |  |
