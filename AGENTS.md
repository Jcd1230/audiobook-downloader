# Agent Instructions

When using `jj` (Jujutsu):
- To create a new revision with a message, use `jj commit -m "message"`.
- Alternatively, use `jj describe -m "message"` followed by `jj new` (or vice-versa) to move to a new working copy commit.
- Avoid using `jj describe -m` repeatedly on the same revision as it only updates the current commit's description.

## Creating a Release

To create a new release:
1. Ensure all changes are committed and tested.
2. Run `mise run release <VERSION>` (e.g. `mise run release 0.4.0`).
   - This task automatically bumps the version in `Cargo.toml` files, creates a commit, moves the `main` bookmark, sets a tag, and pushes everything to GitHub.
3. Once the CI build finishes, use `gh release edit v<VERSION> --notes '...'` to add release notes.
