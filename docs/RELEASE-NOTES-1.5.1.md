<!-- Made by Nordan -->
# WindowCraft 1.5.1

An extension can now put a list next to its button on the Extensions page, and the second extension uses it: the file converter is back, as a separate program from the WindowCraftExtensions repository.

## A list for extensions

The list entry of an extension can have a `choices` block inside `ui`, with a label and 1 to 40 options. WindowCraft shows them as a drop-down list between the description and the button. The value picked there goes to the extension as `--to <value>` before `--`, so the command line is `<exe> --json --lang <tag> --to <value> -- <files>`. Only a value from the extension's own list reaches the command line. An option can have its own file patterns, and the file picker then shows only the files that option can take; without them it uses the patterns of the whole extension. The list keeps the picked value until WindowCraft closes, and starts at the first option.

An entry whose list breaks a rule is left out of the extension list, the same as any other broken entry: no options or more than 40, a value used twice, a value that is not 1 to 16 lowercase letters, digits and hyphens or that starts with a hyphen, a label longer than 40 characters or without English, or patterns that do not look like `*.jpg;*.png`. WindowCraft 1.5.0 ignores the list, so an extension that needs one sets `min_app` to 1.5.1, and 1.5.0 shows that it needs a newer WindowCraft.

## File converter

File converter 1.0.0, by WindowCraft Dev Team, is the second extension. It is the converter code of 1.4.1 as a separate program, without ffmpeg, with data formats added. Everything runs on the computer, and it downloads nothing. Its list "Convert to" has 24 targets:

- Pictures: PNG, JPG, WebP (lossless), AVIF, BMP, TIFF, GIF, ICO and QOI, from PNG, JPEG, WebP, GIF, BMP, TIFF, ICO, TGA, QOI, PNM, HDR, EXR, DDS and Farbfeld.
- Audio: MP3, FLAC, WAV and AIFF from the sound of MP3, FLAC, WAV, AIFF, M4A, AAC, Ogg Vorbis, CAF, MKA, MP4, MOV, MKV, WebM and 3GP files. M4A and Opus are copies of an AAC, ALAC or Opus track.
- Video: MP4, MOV, MKV and WebM as stream copies. Nothing is re-encoded, so a stream that does not fit the target container is refused. WebM leaves out subtitles.
- Data: JSON, YAML, TOML and CSV from one another, and an HTML page from Markdown.

The new file is saved next to the original as `name.ext`, or `name (2).ext` when that name is taken. The original stays as it is. MP3 is written by LAME, which is under the LGPL; the rest is Rust under the MIT License. The README of the extension lists every rule.

The YouTube downloader is not available as an extension.

## Tested

`cargo clippy --workspace --all-targets -- -D warnings` has no warnings. `cargo test --workspace` passes 558 tests, 0 fail, and 28 are ignored: the live tests and the screenshot renders. The new tests check the list rules, which value reaches the command line, the picked value on the Extensions page, and that the real list entry of the file converter passes the rules of the app. A new live test installs a local build of the file converter into a temporary folder and runs it through the engine with `--to toml`.

The Extensions page with the file converter installed was drawn headless with tiny-skia at 1280x800 in English and Polish.

The file converter passes 33 tests in its repository. By hand, a PNG became JPG, WebP, AVIF, ICO, QOI, TIFF and BMP; a WAV became MP3, FLAC and AIFF, and each of them became WAV again with the same length as the original; JSON became YAML and TOML; Markdown became HTML with the HTML written inside it shown as text.

Not tested: the drop-down list by mouse in the real app, real video files (only containers built by the tests), and the ARM64 builds, which have never run on ARM64 hardware.
