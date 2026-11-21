<!-- Made by Nordan -->

# Windows Updates

Windows Updates controls how Windows Update downloads and installs feature updates, quality updates, and hardware drivers.

## Page controls

### Active

Shows which update mode matches the current Windows Update policy values in the registry. Read-only status badge determined by `NoAutoUpdate` and `DeferFeatureUpdatesPeriodInDays`.

### Apply

Writes the selected update mode's registry values, service startup types, and scheduled task states inside a [Time Machine](time-machine.md) snapshot session.

Changes: See the individual update mode entries below.

Undo: See the individual update mode entries below.

Note: Requires administrator rights. If Windows Update Medic restores `BITS` or `wuauserv` immediately after the write, WindowCraft reverts the registry writes and reports an error.

## Update modes

### Default

Removes update policies managed by WindowCraft, sets `BITS` and `wuauserv` to `Manual` and `UsoSvc` to `Automatic`, and re-enables Windows Update scheduled task folders. Windows returns to its standard update schedule.

Changes: Removes 11 values under `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate` (`ExcludeWUDriversInQualityUpdate`, `DeferFeatureUpdates`, `DeferFeatureUpdatesPeriodInDays`, `DeferQualityUpdates`, `DeferQualityUpdatesPeriodInDays`), `...\WindowsUpdate\AU` (`NoAutoUpdate`, `NoAutoRebootWithLoggedOnUsers`), `...\Device Metadata\PreventDeviceMetadataFromNetwork`, `...\DriverSearching\DontSearchWindowsUpdate`, and `HKLM\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings` (`DeferFeatureUpdatesPeriodInDays`, `DeferQualityUpdatesPeriodInDays`); sets `BITS`, `wuauserv`, and `UsoSvc`; enables tasks in `\Microsoft\Windows\InstallService`, `\Microsoft\Windows\UpdateOrchestrator`, `\Microsoft\Windows\UpdateAssistant`, `\Microsoft\Windows\WaaSMedic`, `\Microsoft\Windows\WindowsUpdate`, and `\Microsoft\WindowsUpdate`.

Undo: Time Machine snapshot or applying another update mode.

Note: Requires administrator rights.

### Security / Recommended

Excludes driver updates from Windows Update, defers feature updates by 365 days and quality updates by 4 days, and blocks automatic restarts while a user is signed in. It also turns automatic updates, the update services and the update tasks back on, so it works straight after `Disable ALL Updates`. Quality updates, security fixes included, are offered 4 days after release; feature updates wait a year.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate` (`ExcludeWUDriversInQualityUpdate = 1`, `DeferFeatureUpdates = 1`, `DeferFeatureUpdatesPeriodInDays = 365`, `DeferQualityUpdates = 1`, `DeferQualityUpdatesPeriodInDays = 4`), `HKLM\SOFTWARE\Microsoft\WindowsUpdate\UX\Settings` (`DeferFeatureUpdatesPeriodInDays = 365`, `DeferQualityUpdatesPeriodInDays = 4`), `HKLM\SOFTWARE\Policies\Microsoft\Windows\Device Metadata\PreventDeviceMetadataFromNetwork = 1`, `HKLM\SOFTWARE\Policies\Microsoft\Windows\DriverSearching\DontSearchWindowsUpdate = 1`, `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU\NoAutoRebootWithLoggedOnUsers = 1`; removes `...\AU\NoAutoUpdate`; sets `BITS` and `wuauserv` to `Manual` and `UsoSvc` to `Automatic`; enables the tasks in the six update task folders listed under `Default`.

Undo: Time Machine snapshot or applying `Default`.

Note: Requires administrator rights.

### Disable ALL Updates

Turns off automatic updates, stops and disables the update services, disables all update scheduled tasks, and deletes files in `%SystemRoot%\SoftwareDistribution`. Confirmation is required before this runs.

Changes: `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU\NoAutoUpdate = 1`, `BITS` (`Disabled`), `wuauserv` (`Disabled`), `UsoSvc` (`Disabled`), disables tasks in `\Microsoft\Windows\InstallService`, `\Microsoft\Windows\UpdateOrchestrator`, `\Microsoft\Windows\UpdateAssistant`, `\Microsoft\Windows\WaaSMedic`, `\Microsoft\Windows\WindowsUpdate`, `\Microsoft\WindowsUpdate`, and deletes `%SystemRoot%\SoftwareDistribution` contents.

Undo: Time Machine snapshot or applying `Default` restores the registry policy, services, and scheduled tasks; deleted `%SystemRoot%\SoftwareDistribution` files are not restored.

Note: Requires administrator rights. The system stops receiving security fixes. Unattended CLI `--run` refuses this mode.
