<!-- Made by Nordan -->
# WindowCraft 1.5.0

The Extensions page now installs extensions from a separate repository instead of shipping them inside the app. The four built-in extensions of 1.2 to 1.4 and the libraries that only they used are gone from `windowcraft.exe`, so the portable x64 exe is 15.7 MB instead of 18.9 MB. This release also changes the credit to "Made by Nordan" in the app, the CLI help, the installers, the exe version data, `LICENSE`, `Cargo.toml`, the README and the website, while email addresses and links stay the same.

## The Extensions page

Extensions now come from <https://github.com/windowcraftdev/WindowCraftExtensions>, which is under the MIT License. Each extension is a separate Windows program written in Rust and published as a GitHub release of that repository for x64 and ARM64. The "Made by WindowCraft" badge of the old page is gone.

At the top of the page, the "Available extensions" group has one line and a "Get list" button, which reads "Refresh" once the list is fetched. WindowCraft fetches the list only when you ask. It downloads `index.json` from the repository through the GitHub REST API first, with the raw file as fallback. Entries that break a rule are left out and the rest stay. A list in a newer format says to update WindowCraft. An extension that needs a newer WindowCraft (`min_app`) shows that and cannot be installed, and one with no build for the processor says so.

When you install an extension, WindowCraft downloads the exe for the processor from the extension's release, checks it against the SHA-256 in the list, retries up to 4 times when GitHub fails, and keeps it with a copy of its list entry in `%LOCALAPPDATA%\WindowCraft\extensions\<id>`. That folder is locked to Administrators and SYSTEM, and users may read and run. Before an extension runs, WindowCraft checks that the folder, the extension's folder and its exe are owned by Administrators or SYSTEM and that no one else can write to them; otherwise it refuses and asks to uninstall and install again. An "Update" button appears when the list has a newer version, and a failed update keeps the installed one. "Uninstall" deletes the extension's folder. Uninstalling WindowCraft deletes the whole data folder as before.

Each installed extension has its own group under the list, with the button its list entry defines. WindowCraft runs `<exe> --json --lang <tag> -- <files>`, shows the progress the extension prints and shows its summary at the end. These runs are not written to `windowcraft.log`, because they name the user's files.

## Metadata stripper

The first extension is Metadata stripper 1.0.1, by WindowCraft Dev Team under the MIT License. It is the stripper code of 1.4.1 as a separate program, all in Rust, with no other program and no network access. Its C runtime is linked statically, so it starts on a clean Windows 11. Version 1.0.0, published a few hours earlier, needed `VCRUNTIME140.dll` and failed to start there with 0xC0000135; the VM test below found it. Its group on the Extensions page has a "Choose files" button, which opens the file picker.

The stripper writes a copy with `-clean` in the name and leaves the original as it is. It handles JPEG, PNG, WebP, GIF, MP3, AAC, FLAC, MP4, MOV, M4A, 3GP, MKV, WebM, WAV, AVI, AIFF, Ogg, Opus, WMA and WMV. FLV, OGV, TS and HEIF are no longer handled, because only ffmpeg did that.

## What was removed

The four built-in extensions of 1.2 to 1.4 (metadata stripper, file converter, Additional formats with ffmpeg and YouTube downloader) and their code (the metadata, converter, container, remux, YouTube and tool download modules) are removed from the app, together with the libraries that only they used: `image` in the engine, Symphonia, flacenc and LAME (`mp3lame-encoder`, `mp3lame-sys`). The portable x64 exe is 15.7 MB instead of 18.9 MB.

The file converter and the YouTube downloader are not available as extensions yet.

## Cleanup of the old tools folder

On every start until it succeeds, 1.5.0 deletes `%LOCALAPPDATA%\WindowCraft\tools` and `youtube-staging`, where 1.2 to 1.4 kept yt-dlp, ffmpeg and Deno. 1.4 locked the tools folder to Administrators, so without elevation the delete fails and the next elevated start tries again. Videos and files the user saved stay.

## Making extensions

Extensions are separate Windows programs written in Rust, published as x64 and ARM64 releases in <https://github.com/windowcraftdev/WindowCraftExtensions> (MIT) and listed in `index.json`. WindowCraft runs an installed extension as `<exe> --json --lang <tag> -- <files>` and shows the progress and summary it prints. The README in <https://github.com/windowcraftdev/WindowCraftExtensions> has the details.

## Tested

`cargo clippy --workspace --all-targets -- -D warnings` has no warnings. `cargo test --workspace` passes 554 tests, 0 fail, and 27 are ignored: the live tests and the screenshot renders. 1.4.1 had 598; most of the difference are tests of the removed extension code. The tests of the metadata stripper moved with it to the WindowCraftExtensions repository, where 34 pass. The new code in WindowCraft has unit tests for the list rules, installing into a folder, a failed update that keeps the installed version, and the output lines of an extension. The live tests download the list from GitHub, install the metadata stripper from its release and run it on a JPEG.

The Extensions page was drawn headless with tiny-skia at 1280x800 in English and Polish, in each state: before the list is fetched, while it loads, empty, with entries, installed, with an update, and after the list failed to load.

A short pass ran in a VirtualBox VM with Windows 11 Home 26H2, build 26300, in Polish, from a clean snapshot, with the x64 packages of this release. 1.5.0 was installed over a 1.4.1 that had a `tools` folder locked to Administrators and a `youtube-staging` folder; both were gone after the first start. Then, with real mouse clicks in the app: Get list, Install, Refresh, Update from 1.0.0 to 1.0.1, Choose files with the path of a JPEG with a GPS tag pasted into the file picker, and Uninstall. The clean copy had no GPS tag, the original was unchanged, and `windowcraft.log` did not name the file. The extensions folder was owned by Administrators, did not inherit, and gave users read and run only. The engine test, run elevated, installed the metadata stripper from GitHub into a new folder, ran it, and refused to run it after `icacls` gave Users write access to the exe.

The first run of this pass found that metadata stripper 1.0.0 did not start on the clean system, see above. 1.0.1 passed every step.

The installers passed 64 of 64 checks: the SHA-256 of each file, `verify-install.ps1`, the upgrade from 1.4.1 with history and settings, `WCLANGUAGE` in all eight languages and the uninstall.

Not checked on a live system: the ARM64 builds of WindowCraft and of the metadata stripper, an extension run without elevation, and a download that fails during an update.
