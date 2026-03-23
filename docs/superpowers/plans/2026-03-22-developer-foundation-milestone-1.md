# Milestone 1: The Developer Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish a robust architectural foundation with modular commands, `tracing` logging, and `miette` error handling.

**Architecture:** Split `commands/mod.rs` into focused modules, introduce `thiserror` for the library and `miette` for the CLI, and implement a global `--verbose` flag to control `tracing`.

**Tech Stack:** Rust, `clap`, `tracing`, `miette`, `thiserror`, `assert_cmd`.

---

### Task 1: Add Dependencies

**Files:**
- Modify: `audible_api/Cargo.toml`
- Modify: `cli/Cargo.toml`

- [ ] **Step 1: Add library dependencies**

Add `thiserror = "2.0.11"` and `tracing = "0.1.41"` to `audible_api/Cargo.toml`.

- [ ] **Step 2: Add CLI dependencies**

Add `miette = { version = "7.5.0", features = ["fancy"] }`, `thiserror = "2.0.11"`, `tracing = "0.1.41"`, and `tracing-subscriber = "0.3.19"` to `cli/Cargo.toml`.
Add `assert_cmd = "2.0.16"` and `predicates = "3.1.3"` to `[dev-dependencies]` in `cli/Cargo.toml`.

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: Success

- [ ] **Step 4: Commit**

```bash
jj commit -m "feat: add foundation dependencies (miette, tracing, thiserror)"
```

---

### Task 2: Implement Library Errors and Logging

**Files:**
- Create: `audible_api/src/error.rs`
- Modify: `audible_api/src/lib.rs`
- Modify: `audible_api/src/auth.rs`
- Modify: `audible_api/src/client.rs`

- [ ] **Step 1: Create `audible_api/src/error.rs`**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

- [ ] **Step 2: Update library to use `Result` and `tracing`**

Register the `error` module in `lib.rs`, and update function signatures in `auth.rs` and `client.rs` to return `audible_api::Result`. Use `tracing::info!` or `debug!` for logging.

- [ ] **Step 3: Verify build**

Run: `cargo build -p audible_api`
Expected: Success

- [ ] **Step 4: Commit**

```bash
jj commit -m "feat: implement library-level error handling and tracing"
```

---

### Task 3: Global Verbosity and CLI Errors

**Files:**
- Modify: `cli/src/cli.rs`
- Create: `cli/src/error.rs`
- Modify: `cli/src/main.rs`

- [ ] **Step 1: Add `--verbose` flag to `Cli` struct**

```rust
pub struct Cli {
    #[arg(long, short, global = true)]
    pub verbose: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}
```

- [ ] **Step 2: Create `cli/src/error.rs`**

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CLIError {
    #[error("Audible authentication token has expired")]
    #[diagnostic(help("Run 'audiobook-downloader auth' to refresh your credentials."))]
    AuthExpired,
    
    #[error("No books in local library")]
    #[diagnostic(help("Run 'audiobook-downloader sync' to update your local library."))]
    EmptyLibrary,
    
    #[error(transparent)]
    Library(#[from] audible_api::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
```

- [ ] **Step 3: Update `main.rs` to initialize tracing**

Implement `setup_logging(verbose: bool)` and use it at the start of `main`.

- [ ] **Step 4: Verify build**

Run: `cargo build`
Expected: Success

- [ ] **Step 5: Commit**

```bash
jj commit -m "feat: add global verbose flag and cli error handling"
```

---

### Task 4: Command Modularization (The Big Refactor)

**Files:**
- Modify: `cli/src/commands/mod.rs` (Refactor to dispatch)
- Create: `cli/src/commands/{auth, sync, list, search, download, config, info, utils}.rs`
- Modify: `cli/src/state/mod.rs` (Move `search_library` here)

- [ ] **Step 1: Move shared logic to `utils.rs` and `state/mod.rs`**

- [ ] **Step 2: Create per-command files**

- [ ] **Step 3: Update `commands/mod.rs` to dispatch**

- [ ] **Step 4: Verify build and list command**

Run: `cargo run -- list`
Expected: Same output as before refactor.

- [ ] **Step 5: Commit**

```bash
jj commit -m "refactor: modularize command implementations"
```

---

### Task 5: Testing Suite and Code Quality

**Files:**
- Modify: `cli/src/state/mod.rs` (Add unit tests)
- Create: `cli/tests/integration_tests.rs`

- [ ] **Step 1: Add unit tests for `search_library` in `state/mod.rs`**

- [ ] **Step 2: Create initial integration test for `list` command**

Use `assert_cmd` to verify the output against a mock state.

- [ ] **Step 3: Run all tests**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 4: Run clippy and fmt**

Run: `cargo clippy -- -D warnings && cargo fmt --check`
Expected: Success

- [ ] **Step 5: Commit**

```bash
jj commit -m "test: add unit and integration tests and ensure code quality"
```
