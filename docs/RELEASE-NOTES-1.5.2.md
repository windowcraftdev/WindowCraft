<!-- Made by Nordan -->
# WindowCraft 1.5.2

Three catalog fixes and a user guide. The fixes came up while every tweak was written down with what it changes and how to undo it.

## Fixes

Security / Recommended on the Windows Updates page turns updates back on. It now removes `NoAutoUpdate`, sets BITS and wuauserv to Manual and UsoSvc to Automatic, and enables the tasks in the six update task folders, the same as Default does. Before, a switch from Disable ALL Updates to Security / Recommended kept automatic updates, the services and the tasks off.

Remove Microsoft Edge is marked as irreversible. The Edge uninstaller deletes the browser profile with bookmarks, history and saved passwords, and nothing brings that back. Run Tweaks now asks for confirmation when this tweak is queued, and X-Ray Inspector shows "This change cannot be undone automatically." instead of an undo button. Rolling back the session in Time Machine still reinstalls Edge with `winget install Microsoft.Edge` and shows the session as partly rolled back.

New Outlook for Windows wrote `DoNewOutlookAutoMigration = 0` in both positions, so that value never changed. On now removes the policy value and off sets it to 0, which stops the automatic move to the new Outlook.

## User guide

`docs/guide/` has one page for each page of the app except Applications: Tweaks and Privacy, System, Windows Updates, ISO Studio, Time Machine, the Inspector, Extensions and Settings. Every tweak, option and extension has an entry with the registry values, services, tasks and commands it touches, how to undo it, and what to watch for. The same text is on windowcraft.dev under /docs/.

## Tested

`cargo clippy --workspace --all-targets --release -- -D warnings` has no warnings. `cargo test --release --workspace` passes 558 tests, 0 fail, and 28 are ignored: the live tests and the screenshot renders. The catalog tests check the changed entries against the schema, the translations in all eight languages, and that every update mode and tweak still loads. `cargo build --release --locked` builds 1.5.2.

Not tested: the three fixes on a live system. They change catalog data only; the code that applies registry values, services and task folders is the same one that Default and the tweaks have used since 1.0.
