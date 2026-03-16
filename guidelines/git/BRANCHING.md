# Git Branching

The default branch is `develop`. All feature branches are created from and
merged back into `develop`.

- Create a branch from `develop` for each new feature or bugfix, and open a pull
  request when ready for review.
- Branch names follow the convention: `<type>_<short-description>`, where `type`
  is one of `feat`, `fix`, `chore`, or `refactor` (e.g., `feat_parse_css`).
- Keep branches focused on a single task or feature to make reviews easier and
  reduce the risk of conflicts.
- Use git worktrees to allow working on multiple features simultaneously.
- Spin an agent for each task and use the agent's branch for all commits related
  to that task.
- Rebase your branch on the latest `develop` before merging to ensure a clean
  commit history and to resolve any conflicts.
- Merge your branch into `develop` using the `merge` strategy with the `--ff-only` flag to ensure a fast-forward merge.
