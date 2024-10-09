#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# Kill all subprocesses when exiting
# shellcheck disable=2154
trap 'exit $exit_code' INT TERM
trap 'exit_code=$?; kill 0' EXIT

script_dir="$(cd "$(dirname "$0")" && pwd)"

# shellcheck disable=SC1091
. "$script_dir"/../functions.sh

BASE_BRANCH=develop
MASTER_BRANCH=master

# Check if the current branch is the base branch
if [ "$(git rev-parse --abbrev-ref HEAD)" != "$BASE_BRANCH" ]; then
  printf "Error: You must be on the %s branch to finish a release.\n\n" "$BASE_BRANCH"
  exit 1
fi

# Check if the working directory is clean
if [ -n "$(git status --porcelain)" ]; then
  printf "Error: Working directory is not clean. Please commit or stash your changes.\n\n"
  exit 1
fi

# Get the last tag sorted by time
LAST_TAG=$(git for-each-ref --sort=-creatordate --format '%(refname:short)' refs/tags | grep -E '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$' | head -n 1)
RELEASE_BRANCH="release/${LAST_TAG}"

if [ -z "$LAST_TAG" ]; then
  printf "Error: No tags found. Please create a tag before finishing a release.\n\n"
  exit 1
fi

# Merge the release tag into the base branch
git merge "${LAST_TAG}"

# Merger release tag into the master branch
git checkout "${MASTER_BRANCH}"
git merge --ff-only "${LAST_TAG}"

# Push the changes to the remote repository
git push --follow-tags origin "${MASTER_BRANCH}"

# Delete the release branch
git branch -d "${RELEASE_BRANCH}"
git push origin -d "${RELEASE_BRANCH}"

# Checkout and push the base branch
git checkout "${BASE_BRANCH}"
git push --tags origin "${BASE_BRANCH}"

# set positional arguments in their proper place
eval set -- "$PARAMS"
