# Milestone 2: Interactive Experience & Library Management Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement persistent configuration, library importing via filesystem scanning, and interactive TUI elements for book selection.

**Architecture:** Introduce a `Config` struct for persistence, an `import` command using `walkdir` and regex for ASIN detection, and `inquire` for interactive prompts. Use `indicatif::MultiProgress` for concurrent feedback.

**Tech Stack:** Rust, `clap`, `inquire`, `walkdir`, `regex`, `semver`.

---

### Task 1: Persistent Configuration

**Files:**
- Create: `cli/src/config.rs`
- Modify: `cli/src/main.rs` (Register module)
- Modify: `cli/src/commands/config.rs` (Implement logic)
- Modify: `cli/src/commands/mod.rs` (Propagate config)

- [ ] **Step 1: Create `cli/src/config.rs`**

```rust
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::commands::utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub library_path: Option<String>,
    pub filename_template: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let path = get_config_dir().join("config.json");
        if let Ok(data) = std::fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = get_config_dir().join("config.json");
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
```

- [ ] **Step 2: Implement `config` command logic**

Update `commands/config.rs` to support `set <KEY> <VALUE>` and `show`.

- [ ] **Step 3: Update `handle` and command signatures**

Pass `Config` from `main.rs` through `handle` to all commands that need it.

- [ ] **Step 4: Verify build and persistence**

Run: `cargo run -- config set library_path /tmp/books`
Run: `cargo run -- config show`
Expected: Value is saved and displayed correctly.

- [ ] **Step 5: Commit**

```bash
jj commit -m "feat: implement persistent configuration"
```

---

### Task 2: Library Import Command

**Files:**
- Create: `cli/src/commands/import.rs`
- Modify: `cli/src/commands/mod.rs` (Register command)
- Modify: `cli/src/cli.rs` (Add command to enum)

- [ ] **Step 1: Create `cli/src/commands/import.rs`**

Implement recursive scanning using `walkdir`. Use regex `\[([A-Z0-9]{10}|[0-9]{10})\]` to extract ASINs. Update matching books in `library.json` to `Decrypted`.

- [ ] **Step 2: Add `Import` to `Cli` and `handle`**

- [ ] **Step 3: Verify with mock files**

```bash
mkdir -p /tmp/test_import/book1
touch "/tmp/test_import/book1/Title [B012345678].m4b"
cargo run -- config set library_path /tmp/test_import
cargo run -- import
```
Expected: Summary report showing books found.

- [ ] **Step 4: Commit**

```bash
jj commit -m "feat: implement library import via filesystem scanning"
```

---

### Task 3: Interactive TUI Selection

**Files:**
- Modify: `cli/Cargo.toml` (Add dependencies)
- Modify: `cli/src/cli.rs` (Add `--yes` flag)
- Modify: `cli/src/commands/search.rs`
- Modify: `cli/src/commands/download.rs`

- [ ] **Step 1: Add `inquire`, `walkdir`, `regex`, `semver` to dependencies**

- [ ] **Step 2: Add global `--yes` / `-y` flag to `Cli` and propagate it**

- [ ] **Step 3: Implement search interactivity**

If results > 1 and `--yes` is false, use `inquire::Select` to let the user pick a book. Ask if they want to view info or download.

- [ ] **Step 4: Implement download interactivity**

If query matches multiple books, `--all` is false, and `--yes` is false, use `inquire::MultiSelect` to choose titles.

- [ ] **Step 5: Integrate Config into `download`**

Use `library_path` from `Config` if available and not overridden.

- [ ] **Step 6: Verify build**

Run: `cargo build`
Expected: Success

- [ ] **Step 7: Commit**

```bash
jj commit -m "feat: add interactive TUI selection for search and download"
```

---

### Task 4: Multi-bar Progress and Update Checker

**Files:**
- Modify: `cli/src/commands/download.rs` (Use `MultiProgress`)
- Create: `cli/src/update.rs`
- Modify: `cli/src/main.rs` (Trigger update check)

- [ ] **Step 1: Update download command to use `MultiProgress`**

Separate the overall batch progress from the individual file download progress.

- [ ] **Step 2: Create `cli/src/update.rs`**

Implement background check against GitHub API. Cache result for 24h.

- [ ] **Step 3: Hook update check in `main.rs`**

Print the update hint (if any) before the program exits.

- [ ] **Step 4: Final verification**

Run: `cargo run -- -v search "Mary"`
Expected: No interactive prompt if only one match, or TUI if multiple.

- [ ] **Step 5: Commit**

```bash
jj commit -m "feat: add multi-bar progress and background update checker"
```
