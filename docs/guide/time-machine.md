<!-- Made by Nordan -->

# Time Machine

Time Machine tracks tweak, feature, update-mode, autorun, and PCIe interrupt sessions in `%LocalAppData%\WindowCraft` and detects when Windows reverts applied tweaks.

## Configuration Drift

### Re-apply

Re-writes expected registry values and service startup types for every tweak listed under `Configuration Drift`, recording the repair in a new session while preserving the original pre-tweak baseline session for future rollbacks.

Changes: Re-applies drifted registry keys and service startup types from `APPLIED` state records in `%LocalAppData%\WindowCraft\snapshots.redb`.

Undo: Time Machine `Roll Back` on the original baseline session, or X-Ray Inspector `Undo This Tweak`.

Note: Requires administrator rights for `HKLM` keys and services.

## Snapshot Timeline

### Roll Back

Appears on `Committed`, `Open`, and `Failed` sessions. Prompts for confirmation (`Roll back session <id>?`), restores recorded registry values, service startup types, scheduled task states, and Windows optional features that were off beforehand in reverse order, and runs catalog `undo` operations for each tweak in the session.

Changes: Reverts targets recorded in the selected session in `%LocalAppData%\WindowCraft\snapshots.redb` and sets the session state to `Rolled back` or `Partially rolled back`.

Undo: Re-apply the tweaks from Tweaks & Privacy or System Config & Fixes.

Note: Requires administrator rights for system-wide keys, services, tasks, and features. Native steps without an automatic undo (for example, deleted temporary files or BitLocker decryption) are skipped and listed in the completion notice.

### Session expansion and per-change undo

Clicking a session row expands its recorded changes (`<before> -> <after> -> live <live>`), with an undo icon button on each non-native registry, service, task, or feature step that has not been rolled back yet.

Changes: Reverts the single selected registry value, service startup type, scheduled task state, or Windows feature in `%LocalAppData%\WindowCraft\snapshots.redb` and sets that record to `Rolled back`.

Undo: Re-apply the tweak or feature.

Note: Individual native action rows without a `FeatureStep` cannot be undone per row.

## Reports and logs

### Export Environment Report

Writes a JSON summary of OS product, display version, build, architecture, rounded RAM in GB, logical processor count, WinGet and Chocolatey presence, and boolean states of detectable tweaks to `%LocalAppData%\WindowCraft\environment-report.json`. User names, machine names, serial numbers, paths, and IP addresses are excluded by schema.

Changes: Writes `%LocalAppData%\WindowCraft\environment-report.json`.

Undo: Delete `%LocalAppData%\WindowCraft\environment-report.json`.

### Last job log

Displays up to the last 40 lines of the most recent job below the timeline. WindowCraft appends full job histories to `%LocalAppData%\WindowCraft\windowcraft.log` (excluding extension runs on user files) and rotates logs larger than 1 MB to `windowcraft.log.old`.

Changes: Read-only view of `%LocalAppData%\WindowCraft\windowcraft.log`.

