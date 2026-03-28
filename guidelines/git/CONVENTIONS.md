# Git Conventions

- Follow the commit message convention: `<type>(<scope>): <description>`, where
  `type` is one of `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`,
  and `scope` is the area of the codebase affected (e.g. `stylex_transform`,
  `stylexswc/nextjs-plugin`, `apps`, etc.).
- Write a descriptive commit messages, with a title conforming to the convention
  above, and a description of what you did and why. For example, "Setup initial
  repo infrastructure for linting, formatting and testing". Don't include
  implementation details if not required for understanding the change, e.g.,
  when replacing one dependency with another
- Use atomic commits for small, focused changes that can be easily reviewed and
  reverted if necessary. Commit only the files you touched and list each path
  explicitly. For tracked files run git commit -m "<scoped message>" --
  path/to/file1 path/to/file2. For brand-new files, use the one-liner git
  restore --staged :/ && git add "path/to/file1" "path/to/file2" && git commit
  -m "<scoped message>" -- path/to/file1 path/to/file2
- Always double-check git status before any commit
- Rebase your branch on the latest `develop` before merging to ensure a clean
  commit history and to resolve any conflicts.
- Never run destructive git operations (e.g., git reset --hard, rm, git
  checkout/restore to an older commit) unless the user gives an explicit,
  written instruction in this conversation.
- Never use git restore (or similar commands) to revert files you didn't
  author—coordinate with other agents instead so their in-progress work stays
  intact.
- Quote any git paths containing brackets or parentheses (e.g.,
  src/app/[candidate]/\*\*) when staging or committing so the shell does not
  treat them as globs or subshells.
- When running git rebase, avoid opening editors—export GIT_EDITOR=: and
  GIT_SEQUENCE_EDITOR=: (or pass --no-edit) so the default messages are used
  automatically.
- Never amend commits unless you have explicit written approval in the task
  thread.
- Never use git merge --no-ff to merge branches. Always perform
  fast-forward-only merges (for example, use `git merge --ff-only` or the
  equivalent setting in your Git hosting platform).
