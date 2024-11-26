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

# Get the type of release from the arguments
RELEASE_TYPE=$1
PRE_RELEASE_TYPE=$2
BASE_BRANCH=develop
STYLEX_BRANCH=stylexjs

# Check if the current branch is the base branch
if [ "$(git rev-parse --abbrev-ref HEAD)" != "$BASE_BRANCH" ]; then
  printf "Error: You must be on the %s branch to prepare a release.\n\n" "$BASE_BRANCH"
  exit 1
fi

# Check if the working directory is clean
if [ -n "$(git status --porcelain)" ]; then
  printf "Error: Working directory is not clean. Please commit or stash your changes.\n\n"
  exit 1
fi

# Pull the latest changes from the remote repository
git pull --tags --force
git submodule update --init --recursive

# Check if the --with-stylex flag is passed
WITH_STYLEX=false
for arg in "$@"; do
  if [ "$arg" = "--with-stylex" ]; then
    WITH_STYLEX=true
    break
  fi
done

# Check if the stylexjs branch is rebased to the base branch
if [ "$WITH_STYLEX" = true ]; then
  # Get the latest commit of the base branch
  BASE_COMMIT=$(git rev-parse "$BASE_BRANCH")

  # Get the merge base of the base branch and the stylexjs branch
  MERGE_BASE=$(git merge-base "$BASE_BRANCH" "$STYLEX_BRANCH")

  # Check if the stylexjs branch is rebased to the base branch
  if [ "$BASE_COMMIT" != "$MERGE_BASE" ]; then
    printf "Error: The \"%s\" branch is not rebased to the base branch \"%s\".\n\nPlease rebase and try again.\n\n" "$STYLEX_BRANCH" "$BASE_BRANCH"
    exit 1
  fi

  # Print the commits that will be merged from the stylexjs branch
  commits_to_merge=$(git log --oneline "$BASE_BRANCH..$STYLEX_BRANCH")

  # Check if there are any commits to be merged
  if [ -n "$commits_to_merge" ]; then
    printf "\n\nCommits that will be merged from the stylexjs branch:\n\n"
    printf "%s\n" "$commits_to_merge"
  else
    printf "\n\nNo commits to be merged from the stylexjs branch.\n\n"
  fi
fi

# Get the last tag sorted by time
LAST_TAG=$(git for-each-ref --sort=-creatordate --format '%(refname:short)' refs/tags | grep -E '^[0-9]+\.[0-9]+\.[0-9]+$' | head -n 1)

# Increment the last tag based on the release type
NEW_TAG=$(increment_version "$LAST_TAG" "$RELEASE_TYPE" "$PRE_RELEASE_TYPE")
printf "New version tag: %s\n\n" "$NEW_TAG"

# Wait for user confirmation to continue
printf "\nDo you want to continue with the merge? (y/n): "
read -r confirmation
if [ "$confirmation" != "y" ]; then
  printf "Merge aborted by user.\n"
  exit 1
fi

# Create a new branch for the release
git checkout -b "release/$NEW_TAG"

# Merge the StyleX branch into the release branch if the flag is passed
if [ "$WITH_STYLEX" = true ]; then
  printf "Merging stylexjs branch into the release branch\n\n"
  git merge --ff-only "$STYLEX_BRANCH"
fi

# Update the version in crates
find ./crates -maxdepth 3 -name 'node_modules' -prune -o -name 'Cargo.toml' -print | while read -r cargo_file; do
  printf "\nUpdating %s to version %s" "$cargo_file" "$NEW_TAG"
  sed "s/^version = \".*\"/version = \"$NEW_TAG\"/" "$cargo_file" >tmp.$$.toml && mv tmp.$$.toml "$cargo_file"
done

# Update all local dependencies to the new version
find . -maxdepth 3 -name 'node_modules' -prune -o -name 'package.json' -print | while read -r package_file; do
  printf "\nUpdating %s to version %s" "$package_file" "$NEW_TAG"
  jq --arg new_version "$NEW_TAG" '.version = $new_version' "$package_file" >tmp.$$.json && mv tmp.$$.json "$package_file"

  # Update the version and dependencies of the stylex packages in the package.json files
  jq --arg new_version "$NEW_TAG" '
  def update_deps:
    with_entries(
      if (.value | type == "string") and (.value | test("^file:|^link:|^workspace:") | not) and (.key | startswith("@stylexswc/")) then
        .value = $new_version
      else
        .
      end
    );

  if has("dependencies") and (.dependencies | type == "object") then
    .dependencies |= update_deps
  else
    .
  end |
  if has("devDependencies") and (.devDependencies | type == "object") then
    .devDependencies |= update_deps
  else
    .
  end |
  if has("peerDependencies") and (.peerDependencies | type == "object") then
    .peerDependencies |= update_deps
  else
    .
  end
' "$package_file" >tmp.$$.json && mv tmp.$$.json "$package_file"
done

# Install the dependencies
pnpm install

# Update Cargo.lock by building the project
cargo build

# Add the changes to the staging area
git add .

# Commit the changes
git commit -m "chore: bump version to $NEW_TAG"

# Push the release branch to the remote repository
git push --set-upstream origin "release/$NEW_TAG"

# Create a tag for the release
git tag "$NEW_TAG" -m "Release $NEW_TAG"

# Push the tag to the remote repository
git push origin "$NEW_TAG"

# set positional arguments in their proper place
eval set -- "$PARAMS"
