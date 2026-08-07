#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

WORKFLOW="release.yml"
DEFAULT_REMOTE="origin"
DEFAULT_TYPE="patch"
BASE_BRANCH="develop"
RELEASE_BRANCH="master"

remote="$DEFAULT_REMOTE"
release_type="$DEFAULT_TYPE"
prerelease_type=""
stylex_version=""
ref=""
npm_dry_run=false
preview=false
no_fetch=false
yes=false
repository=""
next_version=""

cd "$REPO_ROOT"

# shellcheck source=scripts/git/lib/github.sh
. "$SCRIPT_DIR/lib/github.sh"

# increment_version, the same calculation the workflow runs. It leaks its
# working variables, so nothing below may reuse those names.
# shellcheck source=scripts/functions.sh
. "$REPO_ROOT/scripts/functions.sh"

usage() {
  cat <<EOF
Usage: $0 [--minor|--patch] [--pre <tag>] [options]

Starts a new release by dispatching the $WORKFLOW workflow, after checking
locally everything the workflow would only reject minutes in.

The workflow bumps the version, tags it, drafts a release, then waits on the
release-approval and changlog-approval environments before publishing to npm.

Options:
  --patch                   Patch release. This is the default.
  --minor                   Minor release.
  --pre <tag>               Prerelease with this tag, for example rc or dev.
  --stylex-version <ver>    Official StyleX compatibility target, e.g. 0.18.3.
  --ref <branch>            Branch to release from. Default: the current branch.
  --remote <name>           Remote to resolve the repository from. Default: $DEFAULT_REMOTE.
  --npm-dry-run             Run the full release gate without publishing to npm.
  --preview                 Print the plan and exit without dispatching.
  --no-fetch                Skip fetching tags before computing the next version.
  --yes, --no-confirm       Skip interactive confirmation.
  -h, --help                Show this help message.
EOF
}

parse_args() {
  while [ "$#" -ne 0 ]; do
    case "$1" in
      --patch | --minor)
        release_type="${1#--}"
        shift
        ;;
      --pre)
        [ "$#" -ge 2 ] || error "--pre requires a value."
        prerelease_type="$2"
        shift 2
        ;;
      --stylex-version)
        [ "$#" -ge 2 ] || error "--stylex-version requires a value."
        stylex_version="$2"
        shift 2
        ;;
      --ref)
        [ "$#" -ge 2 ] || error "--ref requires a value."
        ref="$2"
        shift 2
        ;;
      --remote)
        [ "$#" -ge 2 ] || error "--remote requires a value."
        remote="$2"
        shift 2
        ;;
      --npm-dry-run)
        npm_dry_run=true
        shift
        ;;
      --preview)
        preview=true
        shift
        ;;
      --no-fetch)
        no_fetch=true
        shift
        ;;
      --yes | --no-confirm)
        yes=true
        shift
        ;;
      -h | --help)
        usage
        exit 0
        ;;
      *)
        error "Unsupported argument $1"
        ;;
    esac
  done
}

# Mirrors the workflow's own input validation so a bad combination fails here
# instead of one job into the run.
validate_inputs() {
  if [ -n "$stylex_version" ]; then
    echo "${stylex_version#v}" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$' ||
      error "--stylex-version must be semver in the form 0.18.3 or v0.18.3."
  fi

  if [ -n "$prerelease_type" ]; then
    echo "$prerelease_type" | grep -Eq '^[0-9A-Za-z-]+$' ||
      error "--pre must be alphanumeric, for example rc or dev."
  fi
}

# Both the next-version calculation and the behind-develop check read local
# refs, so a stale checkout silently answers the wrong question.
sync_remote_refs() {
  if [ "$no_fetch" = true ]; then
    return 0
  fi

  git fetch --quiet --tags "$remote" ||
    error "Could not fetch from '$remote'. Pass --no-fetch to skip."
}

remote_commit() {
  git rev-parse --verify --quiet "refs/remotes/$remote/$1"
}

# `calculate-version` refuses a ref that is behind develop, and `pre-release`
# refuses one master cannot fast-forward to -- the second only after bumping
# versions, installing dependencies and updating the Cargo lock, so it is the
# expensive failure worth catching here.
check_ref() {
  if [ -z "$ref" ]; then
    ref="$(git symbolic-ref --quiet --short HEAD)" ||
      error "HEAD is detached. Pass --ref <branch>."
  fi

  local ref_commit branch branch_commit

  ref_commit="$(remote_commit "$ref")" ||
    error "Branch '$ref' does not exist on '$remote'. Push it first."

  for branch in "$BASE_BRANCH" "$RELEASE_BRANCH"; do
    if [ "$ref" = "$branch" ]; then
      continue
    fi

    branch_commit="$(remote_commit "$branch")" ||
      error "No $remote/$branch to compare against. Fetch '$remote' first."

    git merge-base --is-ancestor "$branch_commit" "$ref_commit" ||
      error "$remote/$ref is behind $remote/$branch. Rebase before releasing."
  done
}

# increment_version reads local tags to find the previous release and the
# highest existing prerelease.
compute_next_version() {
  local previous_tag

  # `|| true` because grep exits 1 on no match, and with pipefail that would
  # abort the script instead of falling through to the 0.0.0 default the
  # workflow also applies.
  previous_tag="$(git tag --list |
    grep -E '^[0-9]+\.[0-9]+\.[0-9]+$' | sort -V | tail -n 1 || true)"

  next_version="$(increment_version "${previous_tag:-0.0.0}" "$release_type" "$prerelease_type")"

  echo "Previous release: ${previous_tag:-none}"
}

# The workflow force-replaces an existing tag, but it cannot recreate a release
# that already exists for it.
check_version_is_free() {
  if gh release view "$next_version" --repo "$repository" >/dev/null 2>&1; then
    error "A release already exists for $next_version. See delete-draft-release.sh."
  fi
}

# Not fatal -- the workflow deletes and recreates the tag -- but deleting a
# remote tag is destructive enough to say out loud before confirming.
tag_warning() {
  if git ls-remote --exit-code --tags "$remote" "refs/tags/$next_version" >/dev/null 2>&1; then
    echo "Existing tag:     $remote/$next_version will be deleted and recreated"
  fi
}

print_plan() {
  echo "Repository:       $repository"
  echo "Ref:              $ref"
  echo "Release type:     $release_type"

  if [ -n "$prerelease_type" ]; then
    echo "Prerelease:       yes ($prerelease_type)"
  else
    echo "Prerelease:       no"
  fi

  if [ -n "$stylex_version" ]; then
    echo "StyleX target:    $stylex_version"
  fi

  echo "Next version:     $next_version"

  tag_warning

  if [ "$npm_dry_run" = true ]; then
    echo "npm publish:      skipped (--npm-dry-run)"
  else
    echo "npm publish:      yes, after both approval gates"
  fi
}

dispatch() {
  local prerelease=false
  local args=()

  if [ -n "$prerelease_type" ]; then
    prerelease=true
  fi

  # Every input goes as --raw-field: gh sends workflow_dispatch inputs as
  # strings either way, and --field would additionally expand a leading @.
  args+=(--repo "$repository" --ref "$ref")
  args+=(--raw-field "type=$release_type")
  args+=(--raw-field "prerelease=$prerelease")
  args+=(--raw-field "dry-run=$npm_dry_run")

  if [ -n "$prerelease_type" ]; then
    args+=(--raw-field "prerelease-type=$prerelease_type")
  fi

  if [ -n "$stylex_version" ]; then
    args+=(--raw-field "stylex-version=$stylex_version")
  fi

  gh workflow run "$WORKFLOW" "${args[@]}"
}

main() {
  parse_args "$@"
  require_commands gh git
  validate_inputs

  repository="$(resolve_github_repository "$remote")"

  sync_remote_refs
  check_ref
  compute_next_version
  check_version_is_free
  print_plan

  if [ "$preview" = true ]; then
    echo "Preview only, nothing dispatched."
    exit 0
  fi

  confirm_or_exit "$yes"

  echo "Dispatching $WORKFLOW..."
  dispatch

  echo "Runs: https://github.com/$repository/actions/workflows/$WORKFLOW"
  echo "The run pauses for the release-approval and changlog-approval gates."
}

main "$@"
