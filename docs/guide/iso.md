<!-- Made by Nordan -->

# ISO Studio

ISO Studio opens a Windows 11 ISO, applies unattended setup and privacy options, and exports a new bootable ISO or writes directly to a USB drive.

## 1. Source

### Browse

Opens a file picker filtered to `*.iso` and puts the selected path into the source ISO box.

Undo: Clear or replace the path text.

### Open

Mounts the selected ISO, copies the media tree into `%LocalAppData%\WindowCraft\iso\media`, and reads available Windows 11 editions from `sources\install.wim` or `sources\install.esd`. If an earlier session for the same source ISO was interrupted, WindowCraft resumes from the saved step.

Changes: `%LocalAppData%\WindowCraft\iso\media` and `%LocalAppData%\WindowCraft\iso\session.json`.

Undo: Clean `ISO Studio work files` in the Cleaner tab of [System & Gaming Inspector](inspector.md). The Cleaner skips them while an ISO is open, so restart WindowCraft first. This also deletes an ISO downloaded into that folder.

Note: Rejects media that has no Windows 11 image.

### Direct download URL from microsoft.com/software-download/windows11

Accepts a direct ISO download URL and optional expected SHA-256 hash in the adjacent `Expected SHA-256 (optional)` box.

Undo: Clear the text boxes.

### Download

Downloads the ISO from the URL to `%LocalAppData%\WindowCraft\iso\Win11.iso`, checks the SHA-256 hash when provided, and sets the source ISO path to that file.

Changes: `%LocalAppData%\WindowCraft\iso\Win11.iso`.

Undo: Delete `%LocalAppData%\WindowCraft\iso\Win11.iso` manually or via the Cleaner tab in [System & Gaming Inspector](inspector.md).

## 2. Edition

### Edition selection

Lists each edition inside the loaded install image with its index, architecture, and build number. Selecting an edition sets the `/IMAGE/INDEX` value in `autounattend.xml`, writes `sources\ei.cfg` with `Retail` channel and `VL = 0`, deletes `sources\PID.txt`, and sets the answer file architecture to `amd64` or `arm64`.

Changes: `autounattend.xml` and `sources\ei.cfg` inside `%LocalAppData%\WindowCraft\iso\media` when built or written.

Undo: Select another edition before building or writing.

Note: Defaults to `Professional` when present in the image, or the first listed edition otherwise.

## 3. Configuration

Every staged image also runs `sources\$OEM$\$$\Setup\Scripts\SetupComplete.cmd` at first boot to remove default AppX packages from `preset.json` plus `MSTeams`, uninstall OneDrive, apply baseline privacy registry policies to `HKLM` and `Users\Default\NTUSER.DAT`, and disable 7 telemetry scheduled tasks.

### Bypass hardware checks

Adds registry commands to the `windowsPE` pass so Windows Setup skips TPM 2.0, Secure Boot, CPU, RAM, and storage checks and accepts the EULA.

Changes: `autounattend.xml` adds `BypassTPMCheck`, `BypassSecureBootCheck`, `BypassCPUCheck`, `BypassRAMCheck`, and `BypassStorageCheck` (`REG_DWORD = 1`) under `HKLM\SYSTEM\Setup\LabConfig`, plus `<AcceptEula>true</AcceptEula>`.

Undo: Turn off the switch before building or writing.

### Local account setup

Bypasses the online network requirement and hides Microsoft account sign-in screens during OOBE. You can create a local user account during Windows setup.

Changes: `autounattend.xml` adds `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\OOBE\BypassNRO = 1` in `specialize` and `<HideOnlineAccountScreens>true</HideOnlineAccountScreens>` in `oobeSystem`.

Undo: Turn off the switch before building or writing.

### Dark mode by default

Configures Windows OOBE and the Default user profile hive to use dark theme colors for apps and the Windows shell.

Changes: `autounattend.xml` sets `<SystemUsesLightTheme>false</SystemUsesLightTheme>` and `<UWPAppsUseLightTheme>false</UWPAppsUseLightTheme>`; `SetupComplete.cmd` loads `Users\Default\NTUSER.DAT` and sets `AppsUseLightTheme = 0` and `SystemUsesLightTheme = 0` under `Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`.

Undo: Turn off the switch before building or writing.

### Disable device encryption

Prevents automatic BitLocker device encryption during Windows setup.

Changes: `autounattend.xml` adds `HKLM\SYSTEM\CurrentControlSet\Control\BitLocker\PreventDeviceEncryption = 1` in `specialize`.

Undo: Turn off the switch before building or writing.

### Pause updates during OOBE

Temporarily disables Windows Update during OOBE and removes the policy key at the end of `SetupComplete.cmd` on first boot. Setup finishes without downloading large updates mid-OOBE.

Changes: `autounattend.xml` adds `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU\NoAutoUpdate = 1` in `specialize`; `SetupComplete.cmd` runs `reg delete` on that value at first boot.

Undo: Turn off the switch before building or writing.

### Inject system drivers

Exports third-party drivers from the running PC, keeps the newest driver per INF name, injects `SCSIAdapter` and `HDC` drivers into `sources\boot.wim` (index 2), and injects all other exported drivers into the selected edition of `sources\install.wim`.

Changes: `%LocalAppData%\WindowCraft\iso\drivers`, `sources\boot.wim`, and `sources\install.wim` inside `%LocalAppData%\WindowCraft\iso\media` (converts `sources\install.esd` to `sources\install.wim` first if needed).

Undo: Turn off the switch before building. Once the drivers are in the work tree, opening the same ISO again resumes the saved session with the changed images. To start over, restart WindowCraft, clean `ISO Studio work files` in the Cleaner, then open the source ISO again.

Note: Requires administrator rights for DISM offline servicing.

### Bundle selected applications

Copies the running `windowcraft.exe` and a `profile.json` containing the applications currently ticked on the Applications page into the media so they install on first user sign-in, then removes the bundled files and data folder.

Changes: Adds `sources\$OEM$\$$\Setup\Scripts\WindowCraft\windowcraft.exe` and `profile.json`, plus `FirstLogonCommands` in `autounattend.xml` running `--config ... --run`, `--purge-data`, and `rd /s /q`.

Undo: Turn off the switch before building or writing.

## 4. Export

### Build ISO

Creates a dual BIOS/UEFI UDF ISO labeled `WIN11` at the chosen output path (`Browse` picks the save path, defaulting to `Windows11-WindowCraft.iso`). If `oscdimg.exe` is not found in the WinGet packages folder or Windows ADK, WindowCraft temporarily installs `Microsoft.OSCDIMG` via WinGet and uninstalls it when finished.

Changes: Writes the target `.iso` file; updates `%LocalAppData%\WindowCraft\iso\session.json`.

Undo: Delete the created `.iso` file.

Note: Requires an opened ISO (`Open` completed) and an output path.

### Write to USB drive

Controls `Find Drives` and `Erase and Write` for eligible USB drives. `Find Drives` lists only physical USB drives between 8 GB and 2048 GB with at most 8 partitions that do not host Windows or the system drive letter.

Changes: `Find Drives` only reads disk properties.

### Erase and Write

Prompts for confirmation (`Erase disk <number>?`), verifies the disk identity (`serial` or MBR/GPT `layout_id`), wipes the first 4 MB, creates a single GPT FAT32 partition labeled `WIN11` (up to 32 GB), converts an oversized `install.esd` to `install.wim`, splits `install.wim` larger than 3.8 GB into `install.swm` files, and copies the media to the USB drive.

Changes: Destroys all partitions and data on the selected USB drive and writes Windows 11 installation media.

Undo: Not reversible.

Note: Requires administrator rights. Drives with no serial number and no partition table are rejected for safety.
