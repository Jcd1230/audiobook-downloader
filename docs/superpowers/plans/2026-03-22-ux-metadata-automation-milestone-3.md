# Milestone 3: UX Polish, Metadata & Automation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve UX with "Smart Import", metadata tagging, rich help text, and automated release tasks.

**Architecture:** Use `clap_complete` for shell completions, `ffmpeg` for ID3/cover art embedding, and a `mise` script for release automation. Update `audible_api` to detect denied licenses.

**Tech Stack:** Rust, `clap`, `clap_complete`, `ffmpeg`, `tempfile`, `mise`.

---

### Task 1: "License Denied" Handling

**Files:**
- Modify: `audible_api/src/error.rs`
- Modify: `audible_api/src/client.rs`
- Modify: `cli/src/state/mod.rs`
- Modify: `cli/src/commands/download.rs`

- [ ] **Step 1: Add `LicenseDenied` error variant**

Add `LicenseDenied(String)` to `audible_api::Error`.

- [ ] **Step 2: Update `get_aax_download_url` to detect denial**

Check `response["content_license"]["status_code"] == "Denied"`. Return `Err(Error::LicenseDenied(asin))`.

- [ ] **Step 3: Add `Unavailable` status to `BookStatus`**

Add `Unavailable` to `BookStatus` enum in `cli/src/state/mod.rs`.

- [ ] **Step 4: Catch `LicenseDenied` in `download` command**

Update book status to `Unavailable` and save state when caught. Inform the user with a clear message.

- [ ] **Step 5: Verify build**

Run: `cargo build`
Expected: Success

- [ ] **Step 6: Commit**

```bash
jj commit -m "feat: handle license denied errors and add Unavailable status"
```

---

### Task 2: Smart Import and Help Styling

**Files:**
- Modify: `cli/src/cli.rs`
- Modify: `cli/src/commands/import.rs`

- [ ] **Step 1: Add `PATH` arg to `Import` and update help styles**

Implement `styles()` function using ANSI colors and use it in `Cli`. Update `Import` variant to accept `Option<String>`.

- [ ] **Step 2: Implement Smart Import logic**

Update `import` command to use provided path/CWD if `library_path` is unset, and auto-save config if books found. Provide feedback: "Detected books in X. Saving this as your default library_path."

- [ ] **Step 3: Verify help styling**

Run: `cargo run -- --help`
Expected: Colorized output matching "Modern Developer" aesthetic.

- [ ] **Step 4: Verify smart import**

```bash
mkdir -p /tmp/smart_import/book
touch "/tmp/smart_import/book/Title [B999999999].m4b"
# Ensure library_path is unset in config first
cargo run -- import /tmp/smart_import
cargo run -- config show
```
Expected: `library_path` is now `/tmp/smart_import`.

- [ ] **Step 5: Commit**

```bash
jj commit -m "feat: implement smart import and colorized help text"
```

---

### Task 3: Shell Completions

**Files:**
- Modify: `cli/Cargo.toml`
- Modify: `cli/src/cli.rs`
- Create: `cli/src/commands/completions.rs`
- Modify: `cli/src/commands/mod.rs`

- [ ] **Step 1: Add `clap_complete` dependency**

Add `clap_complete = "4.5.2"` to `cli/Cargo.toml`.

- [ ] **Step 2: Create `completions` command**

Implement logic to generate scripts for Bash, Zsh, Fish, PowerShell using `clap_complete::generate`.

- [ ] **Step 3: Verify build**

Run: `cargo run -- completions bash`
Expected: Bash script printed to stdout.

- [ ] **Step 4: Commit**

```bash
jj commit -m "feat: add shell completions command"
```

---

### Task 4: Metadata Enrichment

**Files:**
- Modify: `cli/Cargo.toml` (Add `tempfile`)
- Modify: `cli/src/state/mod.rs` (Add `cover_url`)
- Modify: `cli/src/commands/sync.rs` (Populate `cover_url`)
- Modify: `cli/src/media/mod.rs` (Update `decrypt` logic)
- Modify: `cli/src/commands/download.rs` (Pass metadata)

- [ ] **Step 1: Update `Book` struct and `sync` command**

Add `cover_url` field and populate it from `product_images["500"]` during library sync.

- [ ] **Step 2: Update `Decryptor` trait and `ffmpeg` logic**

Download cover to temp file using `tempfile`. Add `-metadata` flags for `title`, `artist`, `album`, and `comment`. Use `-disposition:v:0 attached_pic` to embed the cover.

- [ ] **Step 3: Verify with mock check**

Ensure `ffmpeg` is called with correct metadata arguments using `--verbose`.

- [ ] **Step 4: Commit**

```bash
jj commit -m "feat: embed metadata and cover art into m4b files"
```

---

### Task 5: Automation and Docs

**Files:**
- Modify: `mise.toml`
- Modify: `AGENTS.md`

- [ ] **Step 1: Add `release` task to `mise.toml`**

Implement the version bumping, tagging, and pushing script using `jj`.

- [ ] **Step 2: Update `AGENTS.md`**

Add clear instructions for creating a release and maintaining version numbers. Include `mise release` usage.

- [ ] **Step 3: Final check**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 4: Commit**

```bash
jj commit -m "chore: add release automation and documentation"
```
