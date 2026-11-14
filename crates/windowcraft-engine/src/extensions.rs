// Made by Nordan
//! Extensions from the WindowCraftExtensions repository.
//!
//! The repository keeps an `index.json` with one entry per extension. The
//! file comes from the internet, so every entry is checked here and an entry
//! that breaks a rule is left out instead of failing the whole list. Fields
//! this version does not know are ignored.
//!
//! An extension is one exe from a release of that repository. Installing
//! downloads it, checks it against the SHA-256 in the list, and keeps it
//! with a copy of its list entry in `extensions\<id>` of WindowCraft's data
//! folder, which uninstalling WindowCraft removes. WindowCraft runs elevated
//! and starts these programs, so the folder is locked to Administrators and
//! SYSTEM, and an extension whose files anyone else could have changed does
//! not run.
//!
//! WindowCraft runs an extension as `<entry> --json --lang <tag> -- <file>...`
//! and reads one JSON object per line from its output: `progress` with
//! `step`, then `summary` on success or `error` on failure.

use crate::download::{block_on, download, download_accepting, fetch_text, fetch_text_accepting};
use crate::error::{EngineError, Result};
use crate::jobs::CancelToken;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub const REPOSITORY: &str = "https://github.com/windowcraftdev/WindowCraftExtensions";
/// The REST API first: the raw and release hosts of github.com answered 504
/// for hours from some edges, while the API kept working.
const INDEX_API: &str = "https://api.github.com/repos/windowcraftdev/WindowCraftExtensions/contents/index.json?ref=main";
const INDEX_RAW: &str = "https://raw.githubusercontent.com/windowcraftdev/WindowCraftExtensions/main/index.json";
const RELEASES_API: &str = "https://api.github.com/repos/windowcraftdev/WindowCraftExtensions/releases/tags/";
const RELEASE_DOWNLOADS: &str = "https://github.com/windowcraftdev/WindowCraftExtensions/releases/download/";
/// The newest index format this version reads.
pub const SCHEMA: u32 = 1;
/// The error text `run` gives when the files of an extension are not locked.
pub const UNTRUSTED: &str = "extension-untrusted";
/// The build this computer needs.
pub const ARCH: &str = if cfg!(target_arch = "aarch64") { "arm64" } else { "x64" };
const LANGUAGES: [&str; 8] = ["en", "pl", "de", "fr", "es", "it", "pt-BR", "uk"];
const MANIFEST: &str = "extension.json";
const MAX_NAME: usize = 48;
const MAX_SUMMARY: usize = 280;
const MAX_AUTHOR: usize = 48;
const MAX_LICENSE: usize = 64;
/// A real index is a few kilobytes; anything far bigger is not one.
const MAX_INDEX_BYTES: usize = 1 << 20;
/// Longest text taken from an extension's output.
const MAX_OUTPUT_TEXT: usize = 500;
const RUN_TIMEOUT: Duration = Duration::from_secs(4 * 3600);
/// GitHub's release CDN sometimes answers 502 or 504 for a moment.
const ATTEMPTS: u32 = 4;
const FIRST_RETRY_WAIT: Duration = Duration::from_secs(2);
const OCTET: Option<&str> = Some("application/octet-stream");

pub type Texts = BTreeMap<String, String>;

/// The row of an installed extension: a button that opens a file picker
/// and runs the extension on the chosen files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ui {
    pub title: Texts,
    pub button: Texts,
    pub note: Option<Texts>,
    /// Title of the file picker.
    pub dialog: Texts,
    /// Name of the file type filter in the picker.
    pub files: Texts,
    /// `*.jpg;*.png` and so on.
    pub patterns: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub url: String,
    pub sha256: String,
}

/// One extension from the list, already checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listing {
    pub id: String,
    pub version: String,
    /// By language tag; always has "en".
    pub names: Texts,
    pub summaries: Texts,
    pub author: String,
    pub license: String,
    pub min_app: Option<String>,
    /// Always a github.com page.
    pub homepage: Option<String>,
    /// File name of the exe.
    pub entry: Option<String>,
    pub ui: Option<Ui>,
    /// By architecture, "x64" and "arm64".
    pub downloads: BTreeMap<String, Download>,
    /// The entry as it came, kept beside the installed exe.
    raw: String,
}

impl Listing {
    pub fn name(&self, tag: &str) -> &str {
        localized(&self.names, tag)
    }

    pub fn summary(&self, tag: &str) -> &str {
        localized(&self.summaries, tag)
    }

    /// The extension asks for a newer WindowCraft than `app_version`.
    pub fn needs_newer_app(&self, app_version: &str) -> bool {
        match (self.min_app.as_deref().and_then(parse_version), parse_version(app_version)) {
            (Some(needed), Some(current)) => needed > current,
            _ => false,
        }
    }

    /// The build for this computer, when the list has one.
    pub fn download_here(&self) -> Option<&Download> {
        self.entry.as_ref().and(self.downloads.get(ARCH))
    }

    pub fn is_newer_than(&self, other: &Listing) -> bool {
        parse_version(&self.version) > parse_version(&other.version)
    }
}

/// The text for `tag`, else the English one.
pub fn localized<'a>(texts: &'a Texts, tag: &str) -> &'a str {
    texts.get(tag).or_else(|| texts.get("en")).map_or("", String::as_str)
}

#[derive(Deserialize)]
struct RawIndex {
    schema: u32,
    #[serde(default)]
    extensions: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct RawListing {
    id: String,
    version: String,
    name: Texts,
    summary: Texts,
    author: String,
    license: String,
    #[serde(default)]
    min_app: Option<String>,
    #[serde(default)]
    homepage: Option<String>,
    #[serde(default)]
    entry: Option<String>,
    #[serde(default)]
    ui: Option<RawUi>,
    #[serde(default)]
    downloads: Option<BTreeMap<String, RawDownload>>,
}

#[derive(Deserialize)]
struct RawUi {
    title: Texts,
    button: Texts,
    #[serde(default)]
    note: Option<Texts>,
    dialog: Texts,
    files: Texts,
    patterns: String,
}

#[derive(Deserialize)]
struct RawDownload {
    url: String,
    sha256: String,
}

/// Downloads the list and returns the valid entries, sorted by id.
/// `EngineError::Unsupported` means the list needs a newer WindowCraft.
pub fn fetch_index() -> Result<Vec<Listing>> {
    let text = block_on(async {
        match fetch_text_accepting(INDEX_API, Some("application/vnd.github.raw+json")).await {
            Ok(text) => Ok(text),
            Err(_) => fetch_text(INDEX_RAW).await,
        }
    })??;
    parse_index(&text)
}

pub fn parse_index(text: &str) -> Result<Vec<Listing>> {
    if text.len() > MAX_INDEX_BYTES {
        return Err(EngineError::Invalid("the extension list is too large".to_owned()));
    }
    let raw: RawIndex = serde_json::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|error| EngineError::Invalid(format!("extension list: {error}")))?;
    if raw.schema > SCHEMA {
        return Err(EngineError::Unsupported(format!("extension list schema {}", raw.schema)));
    }
    let mut listings: Vec<Listing> = Vec::new();
    for value in raw.extensions {
        let Some(listing) = listing_from(value) else {
            continue;
        };
        if !listings.iter().any(|known| known.id == listing.id) {
            listings.push(listing);
        }
    }
    listings.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(listings)
}

fn listing_from(value: serde_json::Value) -> Option<Listing> {
    let raw_text = value.to_string();
    let raw: RawListing = serde_json::from_value(value).ok()?;
    check(raw, raw_text)
}

fn valid_id(id: &str) -> bool {
    (1..=48).contains(&id.len())
        && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !id.starts_with('-')
}

/// A plain file name ending in `.exe`, so it cannot point anywhere else.
fn valid_entry(entry: &str) -> bool {
    (5..=64).contains(&entry.len())
        && entry.ends_with(".exe")
        && !entry.starts_with('.')
        && entry.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
        && !entry.contains("..")
}

fn valid_patterns(patterns: &str) -> bool {
    patterns.len() <= 1000
        && patterns.split(';').all(|pattern| {
            pattern.strip_prefix("*.").is_some_and(|extension| {
                (1..=10).contains(&extension.len())
                    && extension.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            })
        })
}

fn check(raw: RawListing, raw_text: String) -> Option<Listing> {
    if !valid_id(&raw.id) || parse_version(&raw.version).is_none() {
        return None;
    }
    let min_app = match raw.min_app {
        Some(version) if parse_version(&version).is_none() => return None,
        other => other,
    };
    let homepage = match raw.homepage {
        Some(url) if !url.starts_with("https://github.com/") || url.chars().any(char::is_whitespace) => return None,
        other => other,
    };
    if raw.entry.as_deref().is_some_and(|entry| !valid_entry(entry)) {
        return None;
    }
    if (raw.ui.is_some() || raw.downloads.is_some()) && raw.entry.is_none() {
        return None;
    }
    let ui = match raw.ui {
        None => None,
        Some(ui) => {
            if !valid_patterns(&ui.patterns) {
                return None;
            }
            Some(Ui {
                title: texts(ui.title, 80)?,
                button: texts(ui.button, 32)?,
                note: match ui.note {
                    Some(note) => Some(texts(note, 280)?),
                    None => None,
                },
                dialog: texts(ui.dialog, 80)?,
                files: texts(ui.files, 60)?,
                patterns: ui.patterns,
            })
        }
    };
    let mut downloads = BTreeMap::new();
    for (arch, download) in raw.downloads.unwrap_or_default() {
        // The release workflow names every file this way, so any other URL
        // is not one of its builds.
        let url = format!("{RELEASE_DOWNLOADS}{id}-v{version}/{id}-{arch}.exe", id = raw.id, version = raw.version);
        let hash_ok = download.sha256.len() == 64
            && download.sha256.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        if !matches!(arch.as_str(), "x64" | "arm64") || download.url != url || !hash_ok {
            return None;
        }
        downloads.insert(arch, Download { url: download.url, sha256: download.sha256 });
    }
    Some(Listing {
        id: raw.id,
        version: raw.version,
        names: texts(raw.name, MAX_NAME)?,
        summaries: texts(raw.summary, MAX_SUMMARY)?,
        author: text(&raw.author, MAX_AUTHOR)?,
        license: text(&raw.license, MAX_LICENSE)?,
        min_app,
        homepage,
        entry: raw.entry,
        ui,
        downloads,
        raw: raw_text,
    })
}

/// Language map with only known tags, each text valid, and "en" present.
fn texts(raw: Texts, max: usize) -> Option<Texts> {
    let mut checked = BTreeMap::new();
    for (tag, value) in raw {
        if !LANGUAGES.contains(&tag.as_str()) {
            return None;
        }
        checked.insert(tag, text(&value, max)?);
    }
    checked.contains_key("en").then_some(checked)
}

/// Trimmed text of 1 to `max` characters with no control characters.
fn text(value: &str, max: usize) -> Option<String> {
    let trimmed = value.trim();
    let count = trimmed.chars().count();
    ((1..=max).contains(&count) && !trimmed.chars().any(char::is_control)).then(|| trimmed.to_owned())
}

/// "1.5.0" as numbers; anything else is not a version.
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.split('.');
    let mut next = || parts.next().filter(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))?.parse().ok();
    let parsed = (next()?, next()?, next()?);
    parts.next().is_none().then_some(parsed)
}

// Installed extensions

/// An installed extension and its exe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installed {
    pub listing: Listing,
    pub exe: PathBuf,
}

pub fn extensions_dir() -> PathBuf {
    crate::win::local_app_dir().join("extensions")
}

/// Installed extensions, sorted by id. A folder without a valid entry or
/// without its exe is skipped.
pub fn installed() -> Vec<Installed> {
    installed_in(&extensions_dir())
}

fn installed_in(root: &Path) -> Vec<Installed> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    let mut found: Vec<Installed> = entries
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .filter_map(|entry| {
            let folder = entry.file_name().to_str()?.to_owned();
            let text = std::fs::read_to_string(entry.path().join(MANIFEST)).ok()?;
            let listing = listing_from(serde_json::from_str(&text).ok()?)?;
            let exe = entry.path().join(listing.entry.as_deref()?);
            (listing.id == folder && exe.is_file()).then_some(Installed { listing, exe })
        })
        .collect();
    found.sort_by(|a, b| a.listing.id.cmp(&b.listing.id));
    found
}

/// Where an installation stands, for the progress line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Bytes received and the total, when known.
    Fetching(u64, Option<u64>),
    /// GitHub failed; this attempt starts after a short wait.
    Retrying(u32),
}

/// Downloads the build for this computer, checks its SHA-256 and puts it in
/// place. An installed older version is replaced only once the new one is
/// complete.
pub fn install(listing: &Listing, cancel: &CancelToken, mut progress: impl FnMut(Stage)) -> Result<()> {
    install_in(&extensions_dir(), listing, cancel, |download, file| {
        with_retries(cancel, &mut progress, FIRST_RETRY_WAIT, |progress| fetch(download, file, cancel, progress))
    })
}

fn install_in(
    root: &Path,
    listing: &Listing,
    cancel: &CancelToken,
    fetch: impl FnOnce(&Download, &Path) -> Result<()>,
) -> Result<()> {
    let (Some(entry), Some(download)) = (listing.entry.as_deref(), listing.download_here()) else {
        return Err(EngineError::Unsupported(format!("{} has no {ARCH} build", listing.id)));
    };
    lock_dir(root)?;
    let staging = root.join(format!("{}.download", listing.id));
    remove_dir(&staging)?;
    let result = (|| {
        std::fs::create_dir_all(&staging).map_err(io_error(&staging))?;
        fetch(download, &staging.join(entry))?;
        let manifest = staging.join(MANIFEST);
        std::fs::write(&manifest, &listing.raw).map_err(io_error(&manifest))?;
        cancel.check()?;
        let target = root.join(&listing.id);
        remove_dir(&target)?;
        std::fs::rename(&staging, &target).map_err(io_error(&target))
    })();
    if result.is_err() {
        let _ = std::fs::remove_dir_all(&staging);
    }
    result
}

pub fn uninstall(id: &str) -> Result<()> {
    uninstall_in(&extensions_dir(), id)
}

fn uninstall_in(root: &Path, id: &str) -> Result<()> {
    if !valid_id(id) {
        return Err(EngineError::Invalid(format!("extension id {id}")));
    }
    remove_dir(&root.join(id))
}

/// Deletes a folder; one that is not there is fine. `remove_dir_all` does
/// not follow links or junctions.
fn remove_dir(path: &Path) -> Result<()> {
    match std::fs::remove_dir_all(path) {
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(EngineError::io(path.display().to_string(), error)),
        _ => Ok(()),
    }
}

fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> EngineError + '_ {
    move |error| EngineError::io(path.display().to_string(), error)
}

/// A symbolic link or a junction.
fn is_link(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|meta| meta.file_type().is_symlink())
}

/// Locks the extensions folder to Administrators and SYSTEM. A link in place
/// of the folder is removed first, so the lock cannot land somewhere else.
/// Extensions found in a folder that was not locked could have been changed,
/// so they are deleted.
fn lock_dir(root: &Path) -> Result<()> {
    if is_link(root) {
        std::fs::remove_dir(root).or_else(|_| std::fs::remove_file(root)).map_err(io_error(root))?;
    }
    std::fs::create_dir_all(root).map_err(io_error(root))?;
    if !crate::sysinfo::is_elevated() {
        return Ok(());
    }
    let sddl = crate::security::owner_and_dacl_sddl(root)?;
    if crate::security::admin_only(&sddl, true) {
        return Ok(());
    }
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.filter_map(std::result::Result::ok) {
            remove_dir(&entry.path())?;
        }
    }
    crate::security::apply_owner_and_dacl(root, crate::security::ADMIN_ONLY_SDDL)
}

/// The folder, the extension's folder and its exe can be changed only by
/// administrators. Without elevation a changed exe gains nothing, so there
/// the check is skipped.
fn trusted(installed: &Installed) -> bool {
    if !crate::sysinfo::is_elevated() {
        return true;
    }
    let locked = |path: &Path, protected: bool| {
        !is_link(path)
            && crate::security::owner_and_dacl_sddl(path).is_ok_and(|sddl| crate::security::admin_only(&sddl, protected))
    };
    let folder = installed.exe.parent();
    let root = folder.and_then(Path::parent);
    root.is_some_and(|root| locked(root, true))
        && folder.is_some_and(|folder| locked(folder, false))
        && locked(&installed.exe, false)
}

/// Server errors and dropped connections are worth another try; a 404 or a
/// checksum mismatch is not.
fn transient(error: &EngineError) -> bool {
    matches!(error, EngineError::Network(detail) if !detail.contains("client error"))
}

/// Waits in short steps so Cancel still answers at once.
fn pause(wait: Duration, cancel: &CancelToken) -> Result<()> {
    let until = std::time::Instant::now() + wait;
    while std::time::Instant::now() < until {
        cancel.check()?;
        std::thread::sleep(Duration::from_millis(100));
    }
    cancel.check()
}

fn with_retries<T>(
    cancel: &CancelToken,
    progress: &mut impl FnMut(Stage),
    wait: Duration,
    mut attempt: impl FnMut(&mut dyn FnMut(Stage)) -> Result<T>,
) -> Result<T> {
    let mut wait = wait;
    let mut number = 1;
    loop {
        match attempt(progress) {
            Err(error) if number < ATTEMPTS && transient(&error) => {
                number += 1;
                progress(Stage::Retrying(number));
                pause(wait, cancel)?;
                wait *= 3;
            }
            other => return other,
        }
    }
}

/// Through the API first, then the release link. Both check the SHA-256
/// from the list.
fn fetch(download: &Download, file: &Path, cancel: &CancelToken, progress: &mut dyn FnMut(Stage)) -> Result<()> {
    match fetch_through_api(download, file, cancel, progress) {
        Ok(()) => Ok(()),
        Err(EngineError::Cancelled) => Err(EngineError::Cancelled),
        Err(error @ EngineError::Checksum { .. }) => Err(error),
        Err(_) => block_on(download_file(download, file, cancel, progress))?,
    }
}

async fn download_file(download_info: &Download, file: &Path, cancel: &CancelToken, progress: &mut dyn FnMut(Stage)) -> Result<()> {
    download(&download_info.url, file, Some(&download_info.sha256), cancel, |received, total| {
        progress(Stage::Fetching(received, total));
    })
    .await
}

fn fetch_through_api(download: &Download, file: &Path, cancel: &CancelToken, progress: &mut dyn FnMut(Stage)) -> Result<()> {
    let (tag, asset) = download
        .url
        .strip_prefix(RELEASE_DOWNLOADS)
        .and_then(|rest| rest.split_once('/'))
        .ok_or_else(|| EngineError::Invalid(download.url.clone()))?;
    let release = block_on(fetch_text_accepting(&format!("{RELEASES_API}{tag}"), Some("application/vnd.github+json")))??;
    let api_url = asset_url(&release, asset).ok_or_else(|| EngineError::NotFound(asset.to_owned()))?;
    block_on(download_accepting(&api_url, OCTET, file, Some(&download.sha256), cancel, |received, total| {
        progress(Stage::Fetching(received, total));
    }))?
}

/// The API address of a release asset, from the release as the REST API
/// lists it.
fn asset_url(release_json: &str, name: &str) -> Option<String> {
    let release: serde_json::Value = serde_json::from_str(release_json).ok()?;
    let asset = release
        .get("assets")?
        .as_array()?
        .iter()
        .find(|asset| asset.get("name").and_then(serde_json::Value::as_str) == Some(name))?;
    let url = asset.get("url")?.as_str()?;
    url.starts_with("https://api.github.com/").then(|| url.to_owned())
}

// Running

/// What one line of an extension's output says.
#[derive(Debug, Clone, PartialEq)]
enum Line {
    Progress(f32, String),
    Summary(String),
    Error(String),
    Other,
}

fn parse_line(line: &str) -> Line {
    let Ok(serde_json::Value::Object(object)) = serde_json::from_str::<serde_json::Value>(line.trim()) else {
        return Line::Other;
    };
    let text_of = |key: &str| object.get(key).and_then(serde_json::Value::as_str).map(clean_output);
    if let Some(summary) = text_of("summary") {
        return Line::Summary(summary);
    }
    if let Some(error) = text_of("error") {
        return Line::Error(error);
    }
    match object.get("progress").and_then(serde_json::Value::as_f64) {
        Some(share) if share.is_finite() => Line::Progress(share.clamp(0.0, 1.0) as f32, text_of("step").unwrap_or_default()),
        _ => Line::Other,
    }
}

/// Text from an extension, shown in the app: no control characters, and
/// short.
fn clean_output(text: &str) -> String {
    text.chars().filter(|c| !c.is_control()).take(MAX_OUTPUT_TEXT).collect::<String>().trim().to_owned()
}

/// Runs an installed extension on `files` and returns its summary. A
/// sentence from the extension comes back as `EngineError::Message`.
pub fn run(installed: &Installed, lang: &str, files: &[PathBuf], cancel: &CancelToken, mut progress: impl FnMut(f32, String)) -> Result<String> {
    if !trusted(installed) {
        return Err(EngineError::Invalid(UNTRUSTED.to_owned()));
    }
    let paths: Vec<String> = files.iter().map(|file| file.display().to_string()).collect();
    let mut args = vec!["--json", "--lang", lang, "--"];
    args.extend(paths.iter().map(String::as_str));
    let (mut summary, mut error, mut last) = (None, None, String::new());
    let output = crate::process::run(&installed.exe, &args, cancel, RUN_TIMEOUT, |line| match parse_line(line) {
        Line::Progress(share, step) => progress(share, step),
        Line::Summary(text) => summary = Some(text),
        Line::Error(text) => error = Some(text),
        Line::Other => {
            if !line.trim().is_empty() {
                last = clean_output(line);
            }
        }
    })?;
    match (output.code, summary, error) {
        (0, Some(summary), _) => Ok(summary),
        (_, _, Some(error)) => Err(EngineError::Message(error)),
        (code, _, None) => Err(EngineError::Process { program: installed.listing.id.clone(), code, detail: last }),
    }
}

/// WindowCraft 1.2 to 1.4 kept yt-dlp, ffmpeg and Deno for its built-in
/// extensions in these folders. 1.5 has none of those extensions, so the
/// folders are deleted. `remove_dir_all` does not follow links or junctions.
/// It fails without elevation, because 1.4 locked the folder to
/// Administrators; the next elevated start tries again.
pub fn remove_legacy_tools() {
    let base = crate::win::local_app_dir();
    for name in ["tools", "youtube-staging"] {
        let _ = std::fs::remove_dir_all(base.join(name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    fn entry(id: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "version": "1.0.0",
            "name": { "en": "Hello", "pl": "Cześć" },
            "summary": { "en": "Says hello." },
            "author": "Nordan",
            "license": "MIT",
            "min_app": "1.5.0",
            "homepage": "https://github.com/windowcraftdev/WindowCraftExtensions/tree/main/extensions/hello",
            "entry": "hello.exe",
            "ui": {
                "title": { "en": "Say hello" },
                "button": { "en": "Choose files" },
                "note": { "en": "Any file." },
                "dialog": { "en": "Choose files" },
                "files": { "en": "Files" },
                "patterns": "*.txt;*.md"
            },
            "downloads": {
                "x64": { "url": format!("{RELEASE_DOWNLOADS}{id}-v1.0.0/{id}-x64.exe"), "sha256": HASH },
                "arm64": { "url": format!("{RELEASE_DOWNLOADS}{id}-v1.0.0/{id}-arm64.exe"), "sha256": HASH }
            },
            "later": "a field from a newer list"
        })
    }

    fn index(entries: Vec<serde_json::Value>) -> String {
        serde_json::json!({ "schema": 1, "extensions": entries }).to_string()
    }

    fn hello() -> Listing {
        parse_index(&index(vec![entry("hello")])).unwrap().remove(0)
    }

    #[test]
    fn reads_a_valid_entry_and_ignores_unknown_fields() {
        let hello = hello();
        assert_eq!((hello.id.as_str(), hello.version.as_str(), hello.author.as_str()), ("hello", "1.0.0", "Nordan"));
        assert_eq!(hello.name("pl"), "Cześć");
        assert_eq!(hello.name("de"), "Hello");
        assert_eq!(hello.summary("pl"), "Says hello.");
        let ui = hello.ui.as_ref().unwrap();
        assert_eq!((localized(&ui.button, "pl"), ui.patterns.as_str()), ("Choose files", "*.txt;*.md"));
        assert_eq!(hello.download_here().unwrap().sha256, HASH);
    }

    #[test]
    fn an_empty_list_is_valid() {
        assert!(parse_index(r#"{"schema": 1, "extensions": []}"#).unwrap().is_empty());
        assert!(parse_index("\u{feff}{\"schema\": 1}").unwrap().is_empty());
    }

    #[test]
    fn a_newer_schema_asks_for_a_newer_app() {
        let result = parse_index(r#"{"schema": 2, "extensions": []}"#);
        assert!(matches!(result, Err(EngineError::Unsupported(_))));
    }

    #[test]
    fn broken_json_is_an_error() {
        assert!(matches!(parse_index("<html>"), Err(EngineError::Invalid(_))));
    }

    #[test]
    fn entries_that_break_a_rule_are_dropped_and_the_rest_stay() {
        let mut bad = Vec::new();
        let other_url = serde_json::json!({ "x64": { "url": "https://github.com/someone/else/releases/download/a/hello-x64.exe", "sha256": HASH } });
        for (key, value) in [
            ("id", serde_json::json!("Bad_Id")),
            ("id", serde_json::json!("-dash")),
            ("version", serde_json::json!("1.0")),
            ("min_app", serde_json::json!("soon")),
            ("homepage", serde_json::json!("https://example.com/")),
            ("homepage", serde_json::json!("http://github.com/x")),
            ("name", serde_json::json!({ "pl": "Bez angielskiego" })),
            ("name", serde_json::json!({ "en": "x".repeat(49) })),
            ("name", serde_json::json!({ "en": "Hello", "xx": "Unknown tag" })),
            ("summary", serde_json::json!({ "en": "Line\nbreak" })),
            ("summary", serde_json::json!({ "en": "   " })),
            ("author", serde_json::json!("")),
            ("entry", serde_json::json!("..\\evil.exe")),
            ("entry", serde_json::json!("sub/hello.exe")),
            ("entry", serde_json::json!("hello.bat")),
            ("entry", serde_json::Value::Null),
            ("downloads", other_url),
            ("downloads", serde_json::json!({ "x64": { "url": format!("{RELEASE_DOWNLOADS}broken-v1.0.0/broken-x64.exe"), "sha256": "AB" } })),
            ("downloads", serde_json::json!({ "x86": { "url": format!("{RELEASE_DOWNLOADS}broken-v1.0.0/broken-x86.exe"), "sha256": HASH } })),
            ("ui", serde_json::json!({ "title": { "en": "T" }, "button": { "en": "B" }, "dialog": { "en": "D" }, "files": { "en": "F" }, "patterns": "*.JPG" })),
            ("ui", serde_json::json!({ "title": { "en": "T" }, "button": { "en": "B" }, "dialog": { "en": "D" }, "files": { "en": "F" }, "patterns": "*.jpg;evil" })),
            ("ui", serde_json::json!({ "title": { "en": "T" }, "dialog": { "en": "D" }, "files": { "en": "F" }, "patterns": "*.jpg" })),
        ] {
            let mut broken = entry("broken");
            broken[key] = value;
            bad.push(broken);
        }
        bad.push(serde_json::json!({ "id": "partial" }));
        bad.push(serde_json::json!("not an object"));
        bad.push(entry("good"));
        let listings = parse_index(&index(bad)).unwrap();
        assert_eq!(listings.iter().map(|l| l.id.as_str()).collect::<Vec<_>>(), ["good"]);
    }

    #[test]
    fn an_entry_without_downloads_or_ui_is_listed() {
        let mut plain = entry("plain");
        let object = plain.as_object_mut().unwrap();
        object.remove("downloads");
        object.remove("ui");
        object.remove("entry");
        let listing = parse_index(&index(vec![plain])).unwrap().remove(0);
        assert!(listing.download_here().is_none() && listing.ui.is_none());
    }

    #[test]
    fn duplicates_keep_the_first_and_the_list_is_sorted() {
        let mut second = entry("alpha");
        second["name"] = serde_json::json!({ "en": "Second" });
        let listings = parse_index(&index(vec![entry("zeta"), entry("alpha"), second])).unwrap();
        assert_eq!(listings.iter().map(|l| (l.id.as_str(), l.name("en"))).collect::<Vec<_>>(), [("alpha", "Hello"), ("zeta", "Hello")]);
    }

    #[test]
    fn texts_are_trimmed() {
        let mut padded = entry("hello");
        padded["author"] = serde_json::json!("  Nordan  ");
        assert_eq!(parse_index(&index(vec![padded])).unwrap()[0].author, "Nordan");
    }

    #[test]
    fn min_app_and_versions_are_compared_as_numbers() {
        let mut listing = hello();
        assert!(!listing.needs_newer_app("1.5.0"));
        assert!(listing.needs_newer_app("1.4.1"));
        listing.min_app = Some("1.10.0".to_owned());
        assert!(listing.needs_newer_app("1.9.9"));
        assert!(!listing.needs_newer_app("1.10.0"));
        listing.min_app = None;
        assert!(!listing.needs_newer_app("0.1.0"));
        let mut newer = hello();
        newer.version = "1.10.0".to_owned();
        assert!(newer.is_newer_than(&listing) && !listing.is_newer_than(&newer));
    }

    #[test]
    fn versions_parse_strictly() {
        assert_eq!(parse_version("1.5.0"), Some((1, 5, 0)));
        for bad in ["1.5", "1.5.0.1", "1..0", "v1.5.0", "1.5.0-rc1", ""] {
            assert_eq!(parse_version(bad), None, "{bad}");
        }
    }

    #[test]
    fn install_keeps_the_entry_beside_the_exe_and_uninstall_removes_both() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("extensions");
        let listing = hello();
        install_in(&root, &listing, &CancelToken::new(), |download, file| {
            assert_eq!(download.sha256, HASH);
            std::fs::write(file, b"MZ").map_err(io_error(file))
        })
        .unwrap();
        let found = installed_in(&root);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].listing, listing);
        assert_eq!(found[0].exe, root.join("hello").join("hello.exe"));
        assert!(!root.join("hello.download").exists());

        uninstall_in(&root, "hello").unwrap();
        assert!(installed_in(&root).is_empty());
        uninstall_in(&root, "hello").unwrap();
        assert!(uninstall_in(&root, "..").is_err());
    }

    #[test]
    fn a_failed_download_keeps_the_installed_version() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("extensions");
        let listing = hello();
        install_in(&root, &listing, &CancelToken::new(), |_, file| std::fs::write(file, b"old").map_err(io_error(file))).unwrap();
        let mut newer = hello();
        newer.version = "2.0.0".to_owned();
        let result = install_in(&root, &newer, &CancelToken::new(), |_, _| Err(EngineError::Network("down".to_owned())));
        assert!(result.is_err());
        let found = installed_in(&root);
        assert_eq!(found[0].listing.version, "1.0.0");
        assert_eq!(std::fs::read(&found[0].exe).unwrap(), b"old");
        assert!(!root.join("hello.download").exists());
    }

    #[test]
    fn a_listing_without_a_build_for_this_computer_cannot_be_installed() {
        let mut listing = hello();
        listing.downloads.remove(ARCH);
        let temp = tempfile::tempdir().unwrap();
        let result = install_in(temp.path(), &listing, &CancelToken::new(), |_, _| Ok(()));
        assert!(matches!(result, Err(EngineError::Unsupported(_))));
    }

    #[test]
    fn installed_skips_folders_that_do_not_match_their_entry() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let write = |folder: &str, manifest: &str, exe: bool| {
            let dir = root.join(folder);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join(MANIFEST), manifest).unwrap();
            if exe {
                std::fs::write(dir.join("hello.exe"), b"MZ").unwrap();
            }
        };
        let good = entry("hello").to_string();
        write("hello", &good, true);
        write("renamed", &good, true);
        write("noexe", &entry("noexe").to_string(), false);
        write("broken", "{", true);
        assert_eq!(installed_in(root).iter().map(|i| i.listing.id.as_str()).collect::<Vec<_>>(), ["hello"]);
    }

    #[test]
    fn output_lines_are_read_and_cleaned() {
        assert_eq!(parse_line(r#"{"progress": 0.5, "step": "1 / 2: a.jpg"}"#), Line::Progress(0.5, "1 / 2: a.jpg".to_owned()));
        assert_eq!(parse_line(r#"{"progress": 7}"#), Line::Progress(1.0, String::new()));
        assert_eq!(parse_line(r#"{"summary": "Done\u0007 "}"#), Line::Summary("Done".to_owned()));
        assert_eq!(parse_line(r#"{"error": "No"}"#), Line::Error("No".to_owned()));
        assert_eq!(parse_line("plain text"), Line::Other);
        assert_eq!(parse_line(r#"{"progress": "x"}"#), Line::Other);
        let long = format!(r#"{{"summary": "{}"}}"#, "a".repeat(2000));
        assert!(matches!(parse_line(&long), Line::Summary(text) if text.len() == MAX_OUTPUT_TEXT));
    }

    #[test]
    fn asset_urls_come_only_from_the_api() {
        let release = r#"{"assets": [
            {"name": "hello-x64.exe", "url": "https://api.github.com/repos/windowcraftdev/WindowCraftExtensions/releases/assets/1"},
            {"name": "evil-x64.exe", "url": "https://example.com/evil"}
        ]}"#;
        assert_eq!(asset_url(release, "hello-x64.exe").as_deref(), Some("https://api.github.com/repos/windowcraftdev/WindowCraftExtensions/releases/assets/1"));
        assert_eq!(asset_url(release, "evil-x64.exe"), None);
        assert_eq!(asset_url(release, "missing.exe"), None);
    }

    /// Installs a locally built extension exe and runs it on a test JPEG:
    /// `WINDOWCRAFT_EXTENSION_EXE=<path to metadata-stripper.exe>
    /// cargo test -p windowcraft-engine live_extension_run -- --ignored --nocapture`.
    #[test]
    #[ignore = "needs a built metadata-stripper.exe"]
    fn live_extension_run() {
        let built = PathBuf::from(std::env::var_os("WINDOWCRAFT_EXTENSION_EXE").expect("WINDOWCRAFT_EXTENSION_EXE"));
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("extensions");
        let mut listing = entry("metadata-stripper");
        listing["entry"] = serde_json::json!("metadata-stripper.exe");
        let listing = listing_from(listing).unwrap();
        install_in(&root, &listing, &CancelToken::new(), |_, file| std::fs::copy(&built, file).map(|_| ()).map_err(io_error(file))).unwrap();
        let installed = installed_in(&root).remove(0);

        let photo = temp.path().join("photo.jpg");
        std::fs::write(&photo, [0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x0A, b'E', b'x', b'i', b'f', 0, 0, 1, 2, 0xFF, 0xD9]).unwrap();
        let mut steps = Vec::new();
        let summary = run(&installed, "pl", std::slice::from_ref(&photo), &CancelToken::new(), |share, step| steps.push((share, step))).unwrap();
        println!("steps {steps:?}\nsummary {summary}");
        assert_eq!(steps, [(0.0, "1 / 1: photo.jpg".to_owned())]);
        assert!(summary.contains("photo-clean.jpg"), "{summary}");
        assert!(temp.path().join("photo-clean.jpg").is_file());

        let error = run(&installed, "en", &[temp.path().join("clip.flv")], &CancelToken::new(), |_, _| {}).unwrap_err();
        println!("error {error}");
        assert!(matches!(error, EngineError::Message(_)), "{error:?}");
    }

    /// Installs every listed extension from GitHub into a temporary folder, the
    /// way the app does, and runs the metadata stripper on a small JPEG.
    #[test]
    #[ignore = "needs the internet"]
    fn live_install_from_github() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("extensions");
        let cancel = CancelToken::new();
        for listing in fetch_index().unwrap() {
            let mut stages = 0;
            install_in(&root, &listing, &cancel, |download, file| {
                with_retries(&cancel, &mut |_| stages += 1, FIRST_RETRY_WAIT, |progress| fetch(download, file, &cancel, progress))
            })
            .unwrap();
            println!("{} {} installed, {stages} progress reports", listing.id, listing.version);
        }
        let stripper = installed_in(&root).into_iter().find(|installed| installed.listing.id == "metadata-stripper").unwrap();
        let photo = temp.path().join("photo.jpg");
        std::fs::write(&photo, [0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x0A, b'E', b'x', b'i', b'f', 0, 0, 1, 2, 0xFF, 0xD9]).unwrap();
        let summary = run(&stripper, "pl", std::slice::from_ref(&photo), &cancel, |_, _| {}).unwrap();
        println!("{summary}");
        assert!(temp.path().join("photo-clean.jpg").is_file());

        if crate::sysinfo::is_elevated() {
            let root_sddl = crate::security::owner_and_dacl_sddl(&root).unwrap();
            println!("extensions folder: {root_sddl}");
            assert!(crate::security::admin_only(&root_sddl, true));
            // Users may now change the exe, so it must not run any more.
            let grant = std::process::Command::new(crate::win::system32("icacls.exe"))
                .arg(&stripper.exe)
                .args(["/grant", "*S-1-5-32-545:(M)"])
                .output()
                .unwrap();
            assert!(grant.status.success());
            let refused = run(&stripper, "en", std::slice::from_ref(&photo), &cancel, |_, _| {});
            println!("after icacls: {refused:?}");
            assert!(matches!(refused, Err(EngineError::Invalid(kind)) if kind == UNTRUSTED));
            println!("elevated: folder locked, changed exe refused");
        }
    }

    #[test]
    #[ignore = "needs the internet"]
    fn live_index_downloads_and_parses() {
        fetch_index().unwrap();
    }
}
