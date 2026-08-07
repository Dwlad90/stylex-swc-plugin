#!/usr/bin/env bash

# Shared helpers for the GitHub-facing scripts in scripts/git. Source it, do not
# execute it.

error() {
  echo "Error: $1" >&2
  exit 1
}

require_commands() {
  local command_name

  for command_name in "$@"; do
    command -v "$command_name" >/dev/null 2>&1 ||
      error "$command_name is required but was not found on PATH."
  done
}

# Resolves owner/repo from a git remote, so gh and git can never act on two
# different repositories.
resolve_github_repository() {
  local remote="$1"
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

# Prompts unless $1 is true. Aborting is a deliberate choice, not a failure, so
# it exits 0.
confirm_or_exit() {
  local skip="$1"
  local answer

  if [ "$skip" = true ]; then
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
