# Agent Instructions

When using `jj` (Jujutsu):
- To create a new revision with a message, use `jj commit -m "message"`.
- Alternatively, use `jj describe -m "message"` followed by `jj new` (or vice-versa) to move to a new working copy commit.
- Avoid using `jj describe -m` repeatedly on the same revision as it only updates the current commit's description.
