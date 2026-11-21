<!-- Made by Nordan -->

# Settings

Settings configures interface language, theme, scheduled drift checks, first-start guidance, and support links, saving preferences in `%LocalAppData%\WindowCraft\settings.json`.

## General

### Language

Selects the interface and catalog language (`English`, `Polski`, `Deutsch`, `Français`, `Español`, `Italiano`, `Português (Brasil)`, or `Українська`). Updates the UI immediately and persists the tag to `settings.json`.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`language` key). On startup, if `settings.json` has no `language`, WindowCraft checks `HKLM\SOFTWARE\WindowCraft\Language` and then `GetUserDefaultUILanguage`.

Undo: Select another language in the list.

### Theme

Switches between `Windows` (follows system light or dark mode and high contrast), `Dark`, and `Light`. Updates the window backdrop immediately; the moon/sun icon button at the bottom of the sidebar cycles through the same three modes.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`appearance` key).

Undo: Select another chip or cycle the sidebar appearance button.

Note: Window translucency (Mica) requires Windows transparency settings and the tiny-skia software renderer.

## Reverted tweaks

### Check automatically

Sets how often the background drift check compares applied tweaks with the live system: `Never` (`0` days), `Daily` (`1` day), `Every 7 days` (`7` days), or `Every 30 days` (`30` days).

Changes: `%LocalAppData%\WindowCraft\settings.json` (`drift_check.interval_days`) and registers or deletes Task Scheduler task `\WindowCraft drift check (<username>)` running `windowcraft.exe --scheduled-check` 3 minutes after sign-in and daily at 12:00 without elevation.

Undo: Select `Never` (with `Check after Windows updates` turned off, this deletes the scheduled task).

### Check after Windows updates

Runs a drift check on the next scheduled trigger whenever the Windows build and UBR string changes from the last recorded check.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`drift_check.after_updates`) and registers or deletes `\WindowCraft drift check (<username>)`.

Undo: Turn off the switch (if `Check automatically` is `Never`, this deletes the scheduled task).

### Check now

Compares all applied tweaks in the snapshot store against the live system immediately, updates the `Configuration Drift` banner if any changed, and records the timestamp and OS build.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`drift_check_state.last_run` and `drift_check_state.build`).

## Introduction and tips

### Show the introduction again

Sets the onboarding status back to pending and opens the 5-step first-start question wizard while keeping any previously recorded answers.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.status = "pending"`).

Undo: Click `Skip` on any wizard screen or press `Escape`.

### Show the tips again

Clears the list of seen page tips and turns tips back on so each page shows its tip card on the next visit. If the first-start questions were still pending, this sets them skipped so tips can display.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.tips_seen` cleared, `onboarding.tips_off = false`).

Undo: Click `Don't show again` on any page tip card.

## About

### Support

Launches the default mail client to `mailto:support@windowcraft.dev`.

### Website

Opens `https://windowcraft.dev` in the default web browser.

## First-start questions and page tips

The first-start wizard opens automatically on a first guided launch (`--open`, `--inspect`, `--config`, and `--preset` bypass it). Answering or pressing number keys `1` to `4` advances through four questions to a `Your plan` summary (`Back`, `Skip`, and `Start`; `Enter` triggers `Start`, `Escape` triggers `Skip`). Clicking `Start` ticks items on their pages without applying any system change.

### What do you want to do with WindowCraft?

Step 1 sets `onboarding.answers.goal`: `Fresh install, slimmed down` (queues default AppX removals too), `Privacy`, `Games and performance` (points to MSI mode and interrupt priority in the Inspector), or `General tidy-up` (opens the Cleaner tab instead of Tweaks & Privacy after `Start`).

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.answers.goal`).

Undo: Reopen via `Show the introduction again` or click `Skip`.

### Do you want to keep your files?

Step 2 sets `onboarding.answers.keep_files`: `Yes, keep them` ticks only `System` cleaner targets (temporary files, CBS/DISM logs, minidumps) after the next Cleaner scan; `No, the system is new` also ticks shader and developer caches (leaving ISO Studio work files unticked).

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.answers.keep_files`).

Undo: Change your selection in the Cleaner tab after scanning, or rerun the introduction.

### Will you build your own Windows ISO?

Step 3 sets `onboarding.answers.build_iso`: `Yes` adds [ISO Studio](iso.md) to the summary plan; `No` leaves it off the plan.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.answers.build_iso`).

Undo: Reopen via `Show the introduction again` or open ISO Studio from the sidebar.

### How much should change?

Step 4 sets `onboarding.answers.depth`: `Carefully` queues the `Minimal` preset (6 tweaks), `Standard` queues the `Standard` preset (13 tweaks), and `Everything` queues the `Advanced` preset (21 tweaks) plus default AppX package removals.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.answers.depth`).

Undo: Use `Clear` on [Tweaks & Privacy](tweaks.md) for the queued tweaks. AppX removals queued by `Everything` stay ticked on the `AppX Packages` tab until you untick them.

### Tip

Shown in the lower-right corner on the first visit to each sidebar page after the wizard finishes or is skipped. `Next` records that page's tip as seen and opens the next unseen page (`Done` on the last unseen page), `Don't show again` disables all tips, and `Escape`, clicking outside, or switching pages closes the card without saving it as seen.

Changes: `%LocalAppData%\WindowCraft\settings.json` (`onboarding.tips_seen` or `onboarding.tips_off`).

Undo: Click `Reset` on `Show the tips again` in Settings.
