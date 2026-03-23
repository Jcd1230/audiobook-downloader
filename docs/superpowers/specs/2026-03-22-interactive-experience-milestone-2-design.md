# Design Spec: Milestone 2 - Interactive Experience & Library Management

## Goal
Enhance the user experience through interactive TUI elements, persistent configuration, and an import mechanism to detect existing audiobook files on disk.

## User Requirements
- **Persistent Config:** Save settings like `library_path` and `filename_template` to a `config.json`.
- **Library Import:** Scan a directory for existing `.m4b` files and mark them as `Decrypted` in the local state based on ASIN patterns in filenames.
- **Interactive TUI:** Use selectable lists for search results and multi-match downloads.
- **Improved Feedback:** Show multi-bar progress for concurrent downloads and check for CLI updates.
- **Scriptability:** Provide a way to bypass interactive prompts for scripting.

## Proposed Design

### 1. Configuration Management
- **File:** `config.json` in the standard app config directory.
- **Structure:**
  ```json
  {
    "library_path": "/home/user/Audiobooks",
    "filename_template": "{author} - {title}"
  }
  ```
- **Commands:**
  - `config set <KEY> <VALUE>`: Update a setting.
  - `config show`: Display current settings.
- **Integration:** The `download` command will now prioritize `library_path` from config if not overridden by a CLI flag.

### 2. Library Import Logic (`import` command)
- **Regex:** `\[([A-Z0-9]{10})\]` (Audible ASIN) and `\[(\d{10})\]` (ISBN-10/Legacy ASIN).
- **Scanning:**
  1. Recursively walk `library_path`.
  2. For every `.m4b` file, extract the ASIN from the path/filename.
  3. If the ASIN exists in `library.json`, update its status to `Decrypted`.
- **Output:** A summary report using "Modern Developer" styling.

### 3. Interactive TUI (`inquire` integration)
- **Search:** After results are shown, allow selecting one to view info or download.
- **Download:** If a query matches multiple books and `--all` is not set, show a multi-select list.
- **Non-Interactive Mode:** Add a global `--yes` / `-y` flag to `Cli` to auto-confirm or skip prompts.

### 4. Progress & Feedback
- **Multi-Progress:** Use `indicatif::MultiProgress`.
  - Concurrency: Process downloads sequentially for now (1 at a time), but use the multi-progress layout for better separation of metadata fetching and actual bytes downloading.
- **Update Checker:** 
  - Asynchronous check against GitHub API.
  - Cache result for 24 hours in a small `.update_cache` file.
  - Display hint at end of execution.

## Implementation Details

### Dependencies to Add
- `inquire = "0.7.5"`
- `regex = "1.11.1"`
- `walkdir = "2.5.0"`
- `semver = "1.0.25"`

### File Structure Changes
- `cli/src/config.rs`: Configuration handling.
- `cli/src/commands/import.rs`: Logic for scanning and database updates.
- `cli/src/update.rs`: Background update checker logic.

## Testing Plan
1. **Import Test:** Create mock directory with `.m4b` files and verify status updates in `library.json`.
2. **Config Test:** Verify persistence and command-line overrides.
3. **Interactive Test:** Manual verification of TUI prompts and `--yes` bypass.
