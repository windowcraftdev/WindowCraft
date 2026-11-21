<!-- Made by Nordan -->

# Extensions

Extensions installs and runs optional standalone tools from the WindowCraftExtensions GitHub repository under `%LocalAppData%\WindowCraft\extensions`.

## Page controls

### Get list / Refresh

Downloads `index.json` from the WindowCraftExtensions GitHub repository (`api.github.com` first, then `raw.githubusercontent.com`), checks schema version `1` and field limits, and lists extensions not yet installed. Nothing is fetched or installed until you click this button.

### Install

Downloads the build matching this processor (`x64` or `arm64`) with up to 4 retry attempts, verifies its SHA-256 hash against the index, locks `%LocalAppData%\WindowCraft\extensions` to `Administrators` and `SYSTEM`, and writes the executable and its `extension.json` manifest into `%LocalAppData%\WindowCraft\extensions\<id>`.

Changes: Creates `%LocalAppData%\WindowCraft\extensions\<id>\<entry>` and `extension.json`.

Undo: Click `Uninstall`.

Note: Running an extension while elevated requires its folder and executable ACLs to remain locked to `Administrators` and `SYSTEM`.

### Update

Shown on an installed extension when the fetched list contains a newer version compatible with the running WindowCraft version and processor architecture. Downloads the new executable into a `.download` staging folder, verifies SHA-256, and replaces the previous folder only after the download completes.

Changes: Replaces `%LocalAppData%\WindowCraft\extensions\<id>`.

Undo: Click `Uninstall`.

### Uninstall

Deletes the extension's folder under `%LocalAppData%\WindowCraft\extensions\<id>`.

Changes: Removes `%LocalAppData%\WindowCraft\extensions\<id>`.

Undo: Click `Get list` and `Install` again.

## Metadata stripper

Manifest id `metadata-stripper` (version `1.0.1`, MIT license, requires WindowCraft `1.5.0` or newer). Runs `metadata-stripper.exe --json --lang <tag> -- <files...>` locally with no network access.

### Remove metadata from files

Opens the `Choose files to clean` picker (`Photos, audio and video`) and writes a clean copy beside each input file with `-clean` added to the file stem (or `-clean-2` if that name exists), stripping GPS location, camera tags, dates, author, comments, editing history, thumbnails, and embedded cover art. JPEG files preserve their color profile, Adobe color transform, and EXIF orientation tag.

Accepted input patterns: `*.jpg;*.jpeg;*.jfif;*.png;*.webp;*.gif;*.mp3;*.aac;*.flac;*.mp4;*.m4a;*.m4v;*.m4b;*.mov;*.3gp;*.mkv;*.mka;*.webm;*.avi;*.wav;*.aif;*.aiff;*.ogg;*.oga;*.opus;*.wma;*.wmv;*.asf`.

Changes: Creates `<name>-clean.<ext>` next to each chosen file; leaves the original file unchanged.

Undo: Delete the `-clean` output file.

Note: FLV, OGV, TS, and HEIF files are not supported.

## File converter

Manifest id `file-converter` (version `1.0.0`, `MIT AND LGPL-2.0-or-later` license, requires WindowCraft `1.5.1` or newer). Runs `file-converter.exe --json --lang <tag> --to <target> -- <files...>` locally with no network access and no bundled `ffmpeg`.

### Convert files

Opens the `Choose files to convert` picker filtered to the selected `Convert to` target format and saves the converted output next to the original as `<name>.<ext>` (or `<name> (2).<ext>` if taken), skipping files already in the target format.

Changes: Creates the converted file beside each input file; leaves the original file unchanged.

Undo: Delete the converted output file.

Inputs: `*.png;*.jpg;*.jpeg;*.jfif;*.webp;*.gif;*.bmp;*.tif;*.tiff;*.ico;*.tga;*.qoi;*.ppm;*.pgm;*.pbm;*.pnm;*.hdr;*.exr;*.dds;*.ff`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| PNG image | `png` | Converts raster images to PNG. Animated GIF inputs use their first frame. |
| JPG image | `jpg` | Converts raster images to JPEG, compositing transparent pixels onto a white background. Animated GIF inputs use their first frame. |
| WebP image (lossless) | `webp` | Converts raster images to lossless WebP. Refuses animated GIF inputs so frames are not lost. |
| AVIF image | `avif` | Encodes raster images as AVIF at quality 80. Animated GIF inputs use their first frame. |
| BMP image | `bmp` | Converts raster images to uncompressed BMP. Animated GIF inputs use their first frame. |
| TIFF image | `tiff` | Converts raster images to TIFF (saved with `.tif` extension). Animated GIF inputs use their first frame. |
| GIF image | `gif` | Converts still raster images to GIF. Refuses animated GIF inputs. |
| ICO icon | `ico` | Converts raster images to Windows ICO icons, scaling dimensions down to at most 256 px. Animated GIF inputs use their first frame. |
| QOI image | `qoi` | Converts raster images to Quite OK Image (QOI) format. Animated GIF inputs use their first frame. |

Inputs: `*.mp3;*.flac;*.wav;*.aif;*.aiff;*.m4a;*.aac;*.ogg;*.oga;*.caf;*.mka;*.mp4;*.m4v;*.mov;*.mkv;*.webm;*.3gp`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| MP3 audio | `mp3` | Decodes audio tracks from audio or video files and encodes MP3 with LAME at VBR V0, folding surround sound to stereo and writing a seekable VBR header. |
| FLAC audio | `flac` | Decodes audio tracks from audio or video files and encodes lossless FLAC, preserving 24-bit depth when present in the source. |
| WAV audio | `wav` | Decodes audio tracks from audio or video files into uncompressed WAV. |
| AIFF audio | `aiff` | Decodes audio tracks from audio or video files into uncompressed AIFF. |

Inputs: `*.mp4;*.m4v;*.mov;*.m4a;*.mkv;*.mka;*.webm;*.3gp`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| M4A audio | `m4a` | Remuxes the first compatible AAC or ALAC audio track into an M4A container without re-encoding. Refuses inputs without an AAC or ALAC track. |

Inputs: `*.webm;*.mkv;*.mka;*.mp4;*.m4a;*.mov`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| Opus audio | `opus` | Remuxes the first Opus audio track into an Ogg/Opus container without re-encoding. Refuses inputs without an Opus track. |

Inputs: `*.mp4;*.m4v;*.mov;*.mkv;*.webm;*.3gp`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| MP4 video | `mp4` | Remuxes video and audio streams into an MP4 container without re-encoding. Refuses inputs with incompatible codecs (like Vorbis audio). |
| MOV video | `mov` | Remuxes video and audio streams into a MOV container without re-encoding. Refuses inputs with incompatible codecs (like VP9 video). |
| MKV video | `mkv` | Remuxes video, audio, and subtitle streams into a Matroska MKV container without re-encoding. |
| WebM video | `webm` | Remuxes compatible video and audio streams (VP8/VP9/AV1 and Vorbis/Opus) into a WebM container without re-encoding, omitting subtitles because WebM cannot store them. |

Inputs: `*.json;*.yaml;*.yml;*.toml;*.csv`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| JSON data | `json` | Converts YAML, TOML, or CSV files to formatted JSON. CSV rows produce arrays/objects whose cell values are kept as text. |

Inputs: `*.json;*.yaml;*.yml;*.toml`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| YAML data | `yaml` | Converts JSON or TOML files into YAML. |
| TOML data | `toml` | Converts JSON or YAML files into TOML. Requires a top-level object/table and no `null` values. |

Inputs: `*.json;*.yaml;*.yml`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| CSV table | `csv` | Converts JSON or YAML arrays of objects into CSV written with a UTF-8 BOM for Excel compatibility. |

Inputs: `*.md;*.markdown`

| Convert to | `--to` | What it does |
| --- | --- | --- |
| HTML page from Markdown | `html` | Renders Markdown files to a standalone HTML document, escaping raw HTML inside the Markdown so it displays as text instead of executing. |
