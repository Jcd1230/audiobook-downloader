# Design Spec: Milestone 3 - UX Polish, Metadata & Automation

## Goal
Improve the user experience with "Smart Import", enhanced metadata tagging, rich help text, and automated release workflows.

## User Requirements
- **Smart Import:** `import` should accept an optional path and automatically configure `library_path` if books are found.
- **Shell Completions:** Support for generating shell completion scripts.
- **Metadata Enrichment:** Embed cover art and ID3 tags (title, author, series) into `.m4b` files using `ffmpeg`.
- **Error Handling:** Explicitly catch "License Denied" errors and mark books as `Unavailable`.
- **Automation:** A `mise release` task to automate version bumping, tagging, and pushing.
- **Styling:** Colorized help text using `clap` v4 features.
- **Documentation:** Clear release instructions in `AGENTS.md`.

## Proposed Design

### 1. Smart Import
- **Signature:** `import [PATH]`
- **Logic:**
  - If `config.library_path` is empty:
    - Check provided `PATH` (or `.` if omitted).
    - If books found: Update `config.json` with the absolute path.
- **Feedback:** Clearly state if the configuration was updated.

### 2. Help Text Styling
- Implement a `styles()` helper for `clap` using ANSI colors:
  - Header: Yellow/Bold
  - Usage: Yellow/Bold
  - Literal: Green
  - Placeholder: Cyan
  - Styles will be applied to the `Cli` struct using `#[command(styles = ...)]`.

### 3. "License Denied" Handling
- **API Error:** Add `LicenseDenied` variant to `audible_api::Error`.
- **Status:** Add `Unavailable` to `BookStatus`.
- **Detection:** In `audible_api`, check for `"status_code": "Denied"` in the `licenserequest` response.
- **CLI Logic:** Catch `LicenseDenied` error during download, update book status to `Unavailable`, and inform the user.
- **Persistence:** Ensure `upsert_book` preserves `Unavailable` status during `sync`.

### 4. Metadata Enrichment
- **Book Struct:** Add `cover_url` (Option<String>).
- **Update during Sync:** Populate `cover_url` from Audible's `product_images["500"]`.
- **Decryption Step:**
  - Pass a `Metadata` struct (or `Book` ref) to `Decryptor::decrypt`.
  - Download cover image to a temp file using `tempfile`.
  - Run `ffmpeg` with `-metadata` flags for `title`, `artist` (author), `album` (series), and `comment` (ASIN).
  - Use `-disposition:v:0 attached_pic` to embed the cover.

### 5. Shell Completions
- **Command:** `audiobook-downloader completions <SHELL>`
- **Support:** Bash, Zsh, Fish, PowerShell.

### 6. Automation (`mise release`)
- **Task:** `mise release <VERSION>`
- **Workflow Script:**
  1. Bump `version` in `cli/Cargo.toml` and `audible_api/Cargo.toml`.
  2. `jj commit -m "chore: bump version to v<VERSION>"`
  3. `jj bookmark move main --to @`
  4. `jj tag set v<VERSION> -r main`
  5. `jj git push --bookmark main`
  6. `git push origin v<VERSION>`

## Implementation Details

### Dependencies
- `clap_complete = "4.5.2"`
- `tempfile = "3.10.1"` (for cover art)

### File Changes
- `cli/src/cli.rs`: Add `Completions` subcommand and update `Import`.
- `cli/src/commands/import.rs`: Update logic.
- `cli/src/commands/completions.rs`: New module.
- `cli/src/media/mod.rs`: Update `decrypt` signature and metadata logic.
- `mise.toml`: Add `release` task.
- `AGENTS.md`: Add release documentation.

## Testing Plan
1. **Import Test:** Verify path auto-detection and config update.
2. **Metadata Test:** Check `ffprobe` output of a decrypted file to verify tags and cover art.
3. **Completion Test:** Ensure generated scripts are valid for at least one shell (e.g., bash).
4. **Version Test:** Verify `audiobook-downloader --version` matches the expected value.
