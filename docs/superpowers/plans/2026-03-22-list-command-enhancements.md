# List Command Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve the `list` command with "Modern Developer" styling and a machine-readable `--json` output.

**Architecture:** Add a `--json` flag to the `List` subcommand, extract terminal formatting into a reusable helper function, and implement deterministic, case-insensitive sorting by title and ASIN.

**Tech Stack:** Rust, `clap`, `serde_json`, `colored`.

---

### Task 1: Add Dependencies

**Files:**
- Modify: `cli/Cargo.toml`

- [ ] **Step 1: Add `colored` dependency**

Add `colored = "2.1.0"` to `[dependencies]` in `cli/Cargo.toml`.

- [ ] **Step 2: Verify build**

Run: `cargo build -p audiobook-downloader`
Expected: Success

- [ ] **Step 3: Commit**

```bash
jj commit -m "feat: add colored dependency for terminal styling"
```

---

### Task 2: Update CLI Definition

**Files:**
- Modify: `cli/src/cli.rs`

- [ ] **Step 1: Add `json` flag to `List` subcommand**

```rust
    /// List available books in the local state
    List {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
```

- [ ] **Step 2: Verify CLI help**

Run: `cargo run -- list --help`
Expected: `--json` flag is visible in output.

- [ ] **Step 3: Commit**

```bash
jj commit -m "feat: add --json flag to list command"
```

---

### Task 3: Implement Helper and Update Commands

**Files:**
- Modify: `cli/src/commands/mod.rs`

- [ ] **Step 1: Update `handle` function to pass `json` flag**

Update the match arm for `Commands::List { json } => list(json).await`.

- [ ] **Step 2: Implement `format_book_line` helper**

```rust
use colored::*;

fn format_book_line(book: &crate::state::Book) -> String {
    let status_label = match book.status {
        crate::state::BookStatus::NotDownloaded => "[NotDownloaded]".dimmed(),
        crate::state::BookStatus::Downloading => "[Downloading  ]".blue(),
        crate::state::BookStatus::Downloaded => "[Downloaded   ]".cyan(),
        crate::state::BookStatus::Decrypted => "[Decrypted    ]".green(),
    };

    let title = book.title.bold().white();
    
    let mut line = format!("{} {}", status_label, title);

    if !book.author.is_empty() {
        line.push_str(&format!(" {} {}", "·".dimmed(), book.author.italic().dimmed()));
    }

    line.push_str(&format!(" {}", format!("({})", book.id).dimmed()));

    line
}
```

- [ ] **Step 3: Update `list` function**

Implement sorting (case-insensitive title, then ASIN) and JSON/Text output logic as per spec.

- [ ] **Step 4: Update `search` and `download` functions to use helper**

Replace manual print loops with the `format_book_line` helper.

- [ ] **Step 5: Verify implementation**

Run: `cargo run -- list` and `cargo run -- list --json`
Expected: Correct styling and valid JSON output.

- [ ] **Step 6: Commit**

```bash
jj commit -m "feat: implement styled list output and json support"
```

---

### Task 4: Final Verification

- [ ] **Step 1: Verify search and download styling**

Run: `cargo run -- search "Hail Mary"` and `cargo run -- download "Hail Mary"` (when multiple matches)
Expected: Styled output matches `list`.

- [ ] **Step 2: Verify JSON sorting**

Run: `cargo run -- list --json | jq '.[0].title'`
Expected: First title in alphabetical order.

- [ ] **Step 3: Verify empty state**

Move `library.json` out of the config directory and run `list --json`.
Expected: `[]` on stdout and hint on stderr.
