#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

DEFAULT_REMOTE="origin"

remote="$DEFAULT_REMOTE"
tag=""
dry_run=false
yes=false
repository=""
releases=""

cd "$REPO_ROOT"

usage() {
  cat <<EOF
Usage: $0 [<tag>] [--remote <name>] [--dry-run] [--yes|--no-confirm]

Deletes a draft release (for example an experimental 0.18.4-dev.1) from GitHub
together with its tag, locally and on the remote. With no <tag>, the most
recently created draft release is used.

Published releases, and tags claimed by a release that is not the draft being
deleted, are refused.

Options:
  <tag>                     Draft release tag to delete. Defaults to the latest draft.
  --remote <name>           Git remote to resolve the repository from. Default: $DEFAULT_REMOTE.
  --dry-run                 Print what would be deleted without deleting anything.
  --yes, --no-confirm       Skip interactive confirmation.
  -h, --help                Show this help message.
EOF
}

error() {
  echo "Error: $1" >&2
  exit 1
}

check_dependencies() {
  command -v gh >/dev/null 2>&1 ||
    error "gh (GitHub CLI) is required to read and delete releases."
  command -v jq >/dev/null 2>&1 ||
    error "jq is required to inspect the release list."
}

parse_args() {
  while [ "$#" -ne 0 ]; do
    case "$1" in
      --remote)
        [ "$#" -ge 2 ] || error "--remote requires a value."
        remote="$2"
        shift 2
        ;;
      --dry-run)
        dry_run=true
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
      -*)
        error "Unsupported flag $1"
        ;;
      *)
        [ -z "$tag" ] || error "Unexpected argument $1"
        tag="$1"
        shift
        ;;
    esac
  done
}

run() {
  if [ "$dry_run" = true ]; then
    echo "  [dry-run] $*"
  else
    "$@"
  fi
}

# Resolves owner/repo from the same remote the tag deletion will target, so gh
# and git can never act on two different repositories.
resolve_repository() {
  local url

  url="$(git remote get-url "$remote" 2>/dev/null)" ||
    error "Git remote '$remote' does not exist."

  url="${url%.git}"

  case "$url" in
    *github.com[:/]*)
      printf '%s\n' "${url#*github.com}" | sed 's|^[:/]||'
      ;;
    *)
      error "Remote '$remote' ($url) is not a GitHub remote."
      ;;
  esac
}

# Every release, drafts included. The published-vs-draft guard reads from this
# snapshot, so it must not be capped -- a published release paged out of the
# window would silently stop protecting its tag.
fetch_releases() {
  gh api --paginate "repos/$repository/releases?per_page=100" \
    --jq '.[] | {id, tagName: .tag_name, isDraft: .draft, createdAt: .created_at}' |
    jq -s '.'
}

releases_jq() {
  printf '%s' "$releases" | jq "$@"
}

# `id` is monotonic in creation order; `created_at` on a draft is the target
# commit's date, so sorting by it picks the wrong draft when commits land out
# of order.
latest_draft_tag() {
  releases_jq -r '[.[] | select(.isDraft)] | max_by(.id) | .tagName // ""'
}

confirm() {
  local answer

  if [ "$yes" = true ] || [ "$dry_run" = true ]; then
    return 0
  fi

  printf 'Continue? [y/N] '
  read -r answer ||
    error "Interactive input unavailable. Pass --yes to run non-interactively."

  case "$answer" in
    y | Y | yes | YES)
      return 0
      ;;
    *)
      echo "Aborted."
      exit 0
      ;;
  esac
}

# 0 -> the ref exists, 2 -> it does not. Anything else is a transport or auth
# failure and must not be reported as "no tag to delete".
remote_tag_state() {
  local status=0

  git ls-remote --exit-code --tags "$remote" "refs/tags/$tag" >/dev/null 2>&1 || status=$?

  case "$status" in
    0)
      echo "present"
      ;;
    2)
      echo "absent"
      ;;
    *)
      error "Could not query '$remote' for refs/tags/$tag (git ls-remote exited $status)."
      ;;
  esac
}

main() {
  parse_args "$@"
  check_dependencies

  repository="$(resolve_repository)"
  releases="$(fetch_releases)"

  if [ -z "$tag" ]; then
    tag="$(latest_draft_tag)"
    [ -n "$tag" ] || {
      echo "No draft release found in $repository. Nothing to do."
      exit 0
    }
  fi

  local matches total drafts release_id remote_tag local_tag

  # shellcheck disable=SC2016 # $tag is a jq variable bound by --arg, not a shell one
  matches="$(releases_jq --arg tag "$tag" '[.[] | select(.tagName == $tag)]')"
  total="$(printf '%s' "$matches" | jq 'length')"
  drafts="$(printf '%s' "$matches" | jq '[.[] | select(.isDraft)] | length')"

  [ "$total" -ne 0 ] || error "No release found for tag $tag in $repository."
  [ "$drafts" -eq "$total" ] || error "Refusing to delete $tag -- it is a published release."
  [ "$total" -eq 1 ] ||
    error "$total draft releases share the tag $tag. Delete them from the GitHub UI."

  release_id="$(printf '%s' "$matches" | jq -r '.[0].id')"
  remote_tag="$(remote_tag_state)"
  local_tag=absent

  if git rev-parse -q --verify "refs/tags/$tag" >/dev/null; then
    local_tag=present
  fi

  echo "Repository: $repository"
  echo "About to delete:"
  echo "  - draft release $tag (id $release_id)"
  if [ "$remote_tag" = present ]; then
    echo "  - remote tag $remote/$tag"
  fi
  if [ "$local_tag" = present ]; then
    echo "  - local tag $tag"
  fi
  if [ "$remote_tag" = absent ] && [ "$local_tag" = absent ]; then
    echo "  - no tag exists for $tag"
  fi

  confirm

  echo "Deleting draft release $tag..."
  run gh api -X DELETE "repos/$repository/releases/$release_id" --silent

  # Re-read after the delete: if a release still claims this tag, it is not the
  # draft we just removed and its tag is not ours to delete.
  if [ "$dry_run" != true ]; then
    releases="$(fetch_releases)"
    # shellcheck disable=SC2016 # $tag is a jq variable bound by --arg, not a shell one
    [ "$(releases_jq --arg tag "$tag" '[.[] | select(.tagName == $tag)] | length')" -eq 0 ] ||
      error "Tag $tag is still claimed by a release. Leaving the tag in place."
  fi

  if [ "$remote_tag" = present ]; then
    echo "Deleting remote tag $remote/$tag..."
    run git push "$remote" --delete "refs/tags/$tag"
  fi

  if [ "$local_tag" = present ]; then
    echo "Deleting local tag $tag..."
    run git tag --delete "$tag"
  fi

  echo "Done."
}

main "$@"
