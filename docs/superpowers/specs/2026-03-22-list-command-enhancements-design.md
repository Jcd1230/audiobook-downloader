# Design Spec: List Command Enhancements

## Goal
Improve the visual presentation of the `list` command in the `audiobook-downloader` CLI to make it more readable and provide a machine-readable `--json` output.

## User Requirements
- **Coloring/Formatting:** Adopt a "Modern Developer" style for the default terminal output.
- **Data Density:** Maintain a compact, one-book-per-line format to handle large libraries (e.g., 100+ titles).
- **Machine Readable:** Provide a `--json` flag that outputs a sorted array of book objects.

## Proposed Design

### 1. Terminal Output ("Modern Developer" Style)
The command will use ANSI escape codes to format each line:
- **Status Badge:** A bracketed prefix, color-coded by status, padded to the width of the longest status (`NotDownloaded`) to ensure vertical alignment of titles:
  - `[Decrypted    ]` (Green)
  - `[Downloading  ]` (Blue)
  - `[Downloaded   ]` (Cyan)
  - `[NotDownloaded]` (Dim/Grey)
- **Title:** Bold white text.
- **Separator:** A dimmed dot (` · `), only shown if both title and author are present.
- **Author:** Dimmed or italicized text.
- **ASIN:** Dimmed grey text in parentheses.

**Mockup:**
```text
[Decrypted    ] **Project Hail Mary** · *Andy Weir* (B08G99999W)
[NotDownloaded] **The Way of Kings** · *Brandon Sanderson* (B0041K95UE)
```

### 2. JSON Output
- **Trigger:** Added as a flag to the `list` subcommand: `audiobook-downloader list --json`.
- **Format:** A pretty-printed JSON array of objects.
- **Ordering:** Sorted by `title` (primary) and `id` (secondary) to ensure a deterministic, predictable output.
- **Stdout Integrity:** When `--json` is enabled, **only** the JSON array will be printed to stdout (no "Found X books" headers) to ensure it can be piped directly to tools like `jq`.

### 3. Implementation Details

#### 3.1 CLI Structure
Modify `cli/src/cli.rs` to add the `--json` flag to the `List` command.

#### 3.2 Command Logic
Modify `cli/src/commands/mod.rs`:
- Update `list(json: bool)` to handle the new logic.
- Extract the "Modern Developer" line formatting into a helper function so it can be reused by the `search` command.
- Load `LibraryState` from `library.json`.
- Collect books into a `Vec<Book>`.
- Sort the vector by `title` then `id`.
- **If `json` is true:** Print **only** the pretty-printed JSON to stdout.
- **If `json` is false:** Print the summary header ("Found X books") followed by the formatted lines.

#### 3.3 Dependencies
- **Colored:** Add `colored = "2.1.0"` to `cli/Cargo.toml` for cross-platform terminal coloring.

## Error Handling
- If the `library.json` file is missing, output a helpful error message to **stderr** if `--json` is used, to keep stdout clean.
- The `--json` output should still be a valid, empty JSON array `[]` if no books are found, to maintain machine-readability.

## Testing Strategy
- **Manual Verification:** 
  - Run `cargo run -- list` to verify colors, padding, and formatting.
  - Run `cargo run -- list --json` and pipe to `jq` to verify structure and sorting.
  - Run `cargo run -- search <query>` to verify reused formatting.
- **Edge Cases:** 
  - Very long titles should not break the one-line-per-book constraint.
  - Missing metadata (e.g., no author) should be handled gracefully (no separator shown).
