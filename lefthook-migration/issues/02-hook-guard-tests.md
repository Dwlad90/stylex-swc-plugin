# Automated tests for the commitizen guard matrix

Status: `resolved`
Type: `task`

The `prepare-commit-msg` guard is pure logic over `$2`/`$3` and is the part of
the migration most likely to silently regress -- a wrong branch means either no
prompt when you wanted one, or a prompt that hangs a rebase.

## Scope

Add `scripts/git/hooks.test.mjs`, picked up by the existing `test:scripts`
script (`node --test .github/scripts/*.test.mjs scripts/git/*.test.mjs`) and so
by the `ci-script-tests` matrix entry, with no CI changes needed.

Invoke `.lefthook/prepare-commit-msg/commitizen.sh` directly with a stubbed
`node_modules/.bin/cz` that records its invocation, and assert:

| `$2` (COMMIT_SOURCE) | `$3` (SHA) | cz runs? |
|---|---|---|
| *(empty)* | *(empty)* | yes |
| `template` | *(empty)* | yes |
| `message` | *(empty)* | no |
| `merge` | *(empty)* | no |
| `squash` | *(empty)* | no |
| *(empty)* | `abc1234` | no -- amend |
| `commit` | `abc1234` | no -- amend |

Only the non-interactive paths are testable here. The prompt itself needs a real
TTY and stays on the manual checklist.

## Also worth covering

`scripts/git/install-hooks.sh` unsets `core.hooksPath` **only** when it equals
`.husky/_`, and leaves a developer's custom value alone.

## Answer

Landed as `scripts/git/hooks.test.mjs`, 20 cases, no CI changes needed. Scope
grew past the original plan during the refactor pass, because two of the three
scripts changed logic and needed a net under them:

- the full commitizen guard matrix above
- `install-hooks.sh`: unset only for `.husky/_`, custom paths survive, and the
  CI skip under both `CI=true` and `CI=1`
- `manifests.sh`: `--source` interleaving, the subshell not leaking its rewritten
  argument list, and a manifest path containing a space surviving as one
  argument
