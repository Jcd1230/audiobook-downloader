# Design Spec: Milestone 1 - The Developer Foundation

## Goal
Establish a robust architectural foundation for `audiobook-downloader` by modularizing command logic, implementing structured logging, improving error handling, and adding a baseline testing suite.

## User Requirements
- **Command Modularization:** Split the large `commands/mod.rs` into focused, per-command files.
- **Logging:** Implement `tracing` for logging, controlled by a global `--verbose` flag.
- **Error Handling:** Use `miette` for user-facing CLI errors and `thiserror` for the library.
- **Testing:** Add unit and integration tests for core library and CLI features.

## Proposed Design

### 1. Command Modularization
Refactor `cli/src/commands/` into a directory structure:
- `mod.rs`: Dispatches to submodules and contains the `handle` function.
- `{auth, sync, list, search, download, config, info}.rs`: Each command gets its own module.
- `utils.rs`: Shared utilities (config directory resolution, line formatting).

### 2. Error Handling Architecture
- **Library (`audible_api`):** Define a custom `Error` enum with `thiserror`. All library functions will return `Result<T, audible_api::Error>`.
- **CLI:** Define a `CLIError` enum with `miette`.
- **Formatting:** Errors will follow a minimalist, two-line format:
  ```text
  Error: [Colored error message]
  Advice: [Dimmed help hint]
  ```

### 3. Logging & Verbosity
- **Global Flag:** Add `#[arg(long, short, global = true)] verbose: bool` to the `Cli` struct.
- **`tracing`:** 
  - Add `tracing` to both `audible_api` and the CLI.
  - Initialize a `tracing-subscriber` in `main.rs` by default with a `WARN` filter.
  - If `--verbose` is present, set the filter to `INFO`.
- **Log Format:** Compact style (e.g., `[INFO] ...`) for a clean CLI experience.

### 4. Testing Strategy
- **Unit Tests:** Move `search_library` into `cli/src/state/mod.rs` and add tests for filtering and sorting.
- **Integration Tests:** Create `cli/tests/integration_tests.rs` using `assert_cmd` and `predicates` to verify CLI behavior against mock state files.
- **Code Quality:** Ensure `cargo clippy` and `cargo fmt` pass as part of the implementation.

## Implementation Details

### Dependencies to Add
- **`audible_api/Cargo.toml`**: `thiserror`, `tracing`
- **`cli/Cargo.toml`**: `miette`, `thiserror`, `tracing`, `tracing-subscriber`
- **`cli/Cargo.toml` (dev-dependencies)**: `assert_cmd`, `predicates`

### File Migrations
- Move `search_library` from `commands/mod.rs` to `state/mod.rs`.
- Move `get_config_dir` and `format_book_line` to `commands/utils.rs`.

## Error Handling Mockup
```rust
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CLIError {
    #[error("Audible authentication token has expired")]
    #[diagnostic(help("Run 'audiobook-downloader auth' to refresh your credentials."))]
    AuthExpired,
    
    #[error("No books found matching '{query}'")]
    #[diagnostic(help("Try a broader search or run 'sync' to update your local library."))]
    NoMatches { query: String },
}
```

## Testing Plan
1.  **State Tests:** Verify `LibraryState` load/save and `search` filtering.
2.  **Command Tests:** Verify `list` and `search` CLI output using mock `library.json`.
