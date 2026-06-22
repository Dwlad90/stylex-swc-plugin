#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

ROOT_README="README.md"
RS_COMPILER_README="crates/stylex-rs-compiler/README.md"
STYLEX_BLOG_URL="https://stylexjs.com/blog"
MARKER_START="<!-- stylex-compatibility:start -->"
MARKER_END="<!-- stylex-compatibility:end -->"

stylex_repo=""
stylex_version=""
dry_run=false
yes=false
allow_dirty=false
temp_dir=""

cd "$REPO_ROOT"

cleanup_temp_dir() {
  if [ -n "$temp_dir" ] && [ -d "$temp_dir" ]; then
    rm -rf "$temp_dir"
  fi
}

trap cleanup_temp_dir EXIT
trap 'cleanup_temp_dir; exit 130' HUP INT TERM

usage() {
  cat <<EOF
Usage: $0 [--version <version>] [--stylex-repo <path>] [--dry-run] [--yes|--no-confirm] [--allow-dirty]

Updates README StyleX compatibility indicators.

Options:
  --version <version>       Official StyleX version. Accepts 0.18.3 or v0.18.3.
  --stylex-repo <path>      Local checkout of facebook/stylex to derive the version from.
  --dry-run                 Print the proposed README changes without writing files.
  --yes, --no-confirm       Skip interactive confirmation.
  --allow-dirty             Allow deriving from a dirty local StyleX checkout.
  -h, --help                Show this help message.
EOF
}

error() {
  echo "Error: $1" >&2
  exit 1
}

check_dependencies() {
  command -v perl >/dev/null 2>&1 ||
    error "perl is required to update README compatibility markers."
}

create_temp_dir() {
  mktemp -d 2>/dev/null || mktemp -d -t stylex-compatibility
}

normalize_path() {
  local path="$1"

  case "$path" in
    \~)
      printf '%s\n' "$HOME"
      ;;
    \~/*)
      printf '%s\n' "$HOME/${path:2}"
      ;;
    *)
      printf '%s\n' "$path"
      ;;
  esac
}

normalize_version() {
  local version="$1"

  version="${version#v}"

  if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    error "StyleX version must be semver in the form 0.18.3 or v0.18.3."
  fi

  printf '%s\n' "$version"
}

read_package_version() {
  local package_file="$1"

  if [ ! -f "$package_file" ]; then
    error "Package file not found: $package_file"
  fi

  sed -n 's/^[[:space:]]*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' \
    "$package_file" | head -n 1
}

ensure_clean_stylex_repo() {
  local repo="$1"

  git -C "$repo" rev-parse --is-inside-work-tree >/dev/null 2>&1 ||
    error "Not a git repository: $repo"

  if [ "$allow_dirty" = false ] && [ -n "$(git -C "$repo" status --porcelain)" ]; then
    error "StyleX repository is dirty. Commit/stash changes or pass --allow-dirty."
  fi
}

derive_stylex_version() {
  local repo="$1"
  local stylex_package_version
  local babel_plugin_version

  ensure_clean_stylex_repo "$repo"

  stylex_package_version=$(
    read_package_version "$repo/packages/@stylexjs/stylex/package.json"
  )
  babel_plugin_version=$(
    read_package_version "$repo/packages/@stylexjs/babel-plugin/package.json"
  )

  if [ -z "$stylex_package_version" ]; then
    error "Could not read @stylexjs/stylex version from $repo."
  fi

  if [ "$stylex_package_version" != "$babel_plugin_version" ]; then
    error "$(
      printf '@stylexjs/stylex (%s) and @stylexjs/babel-plugin (%s) versions differ.' \
        "$stylex_package_version" \
        "$babel_plugin_version"
    )"
  fi

  normalize_version "$stylex_package_version"
}

write_root_badge() {
  local version="$1"
  local output_file="$2"
  local badge_url

  badge_url="https://img.shields.io/badge/StyleX%20compatibility-v${version}-blue"
  printf '[![StyleX compatibility](%s)](%s)' "$badge_url" "$STYLEX_BLOG_URL" \
    >"$output_file"
}

write_rs_compiler_callout() {
  local version="$1"
  local output_file="$2"

  cat >"$output_file" <<EOF

> [!NOTE]
> Compatibility target: this package has been updated through official
> StyleX v${version}. This is not an official Meta support guarantee.
EOF
}

replace_marker_block() {
  local file="$1"
  local replacement_file="$2"
  local output_file="$3"

  perl -0e '
    use strict;
    use warnings;

    my ($file, $replacement_file, $start, $end) = @ARGV;

    open my $input, "<", $file or die "Could not read $file: $!";
    my $content = do { local $/; <$input> };
    close $input;

    open my $replacement_input, "<", $replacement_file
      or die "Could not read $replacement_file: $!";
    my $replacement = do { local $/; <$replacement_input> };
    close $replacement_input;

    my $start_pos = index($content, $start);
    exit 42 if $start_pos == -1;
    exit 42 if index($content, $start, $start_pos + length($start)) != -1;

    my $replacement_start = $start_pos + length($start);
    my $end_pos = index($content, $end, $replacement_start);
    exit 42 if $end_pos == -1;
    exit 42 if index($content, $end, $end_pos + length($end)) != -1;

    substr($content, $replacement_start, $end_pos - $replacement_start) =
      $replacement;
    print $content;
  ' "$file" "$replacement_file" "$MARKER_START" "$MARKER_END" >"$output_file" || {
    rm -f "$output_file"
    error "Expected exactly one compatibility marker block in $file."
  }
}

confirm_version() {
  local entered_version

  if [ "$dry_run" = true ] || [ "$yes" = true ]; then
    return
  fi

  printf 'StyleX compatibility version [%s]: ' "$stylex_version"
  read -r entered_version ||
    error "Interactive input unavailable. Pass --yes/--no-confirm for non-interactive runs."

  if [ -n "$entered_version" ]; then
    stylex_version=$(normalize_version "$entered_version")
  fi

  printf 'Update README compatibility indicators to official StyleX v%s? [y/N] ' \
    "$stylex_version"
  read -r reply ||
    error "Interactive input unavailable. Pass --yes/--no-confirm for non-interactive runs."

  case "$reply" in
    y | Y | yes | YES)
      ;;
    *)
      echo "No files updated."
      exit 0
      ;;
  esac
}

update_readmes() {
  local version="$1"
  local root_badge
  local rs_compiler_callout
  local root_output
  local rs_compiler_output
  local root_backup
  local rs_compiler_backup

  temp_dir=$(create_temp_dir) || error "Failed to create temporary directory."
  root_badge="$temp_dir/root-badge.md"
  rs_compiler_callout="$temp_dir/rs-compiler-callout.md"
  root_output="$temp_dir/root-readme.md"
  rs_compiler_output="$temp_dir/rs-compiler-readme.md"
  root_backup="$temp_dir/root-readme.backup.md"
  rs_compiler_backup="$temp_dir/rs-compiler-readme.backup.md"

  write_root_badge "$version" "$root_badge"
  write_rs_compiler_callout "$version" "$rs_compiler_callout"
  replace_marker_block "$ROOT_README" "$root_badge" "$root_output"
  replace_marker_block "$RS_COMPILER_README" "$rs_compiler_callout" "$rs_compiler_output"

  if [ "$dry_run" = true ]; then
    echo "Dry run: proposed StyleX compatibility update to v${version}"
    diff -u "$ROOT_README" "$root_output" || true
    diff -u "$RS_COMPILER_README" "$rs_compiler_output" || true
  else
    cp "$ROOT_README" "$root_backup"
    cp "$RS_COMPILER_README" "$rs_compiler_backup"

    mv "$root_output" "$ROOT_README" ||
      error "Failed to update $ROOT_README."

    if ! mv "$rs_compiler_output" "$RS_COMPILER_README"; then
      cp "$root_backup" "$ROOT_README" ||
        echo "Warning: failed to restore $ROOT_README after update failure." >&2
      error "Failed to update $RS_COMPILER_README."
    fi

    echo "Updated StyleX compatibility indicators to v${version}."
  fi

  rm -rf "$temp_dir"
  temp_dir=""
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version)
      [ "$#" -ge 2 ] || error "--version requires a value."
      stylex_version=$(normalize_version "$2")
      shift 2
      ;;
    --version=*)
      stylex_version=$(normalize_version "${1#*=}")
      shift
      ;;
    --stylex-repo)
      [ "$#" -ge 2 ] || error "--stylex-repo requires a value."
      stylex_repo=$(normalize_path "$2")
      shift 2
      ;;
    --stylex-repo=*)
      stylex_repo=$(normalize_path "${1#*=}")
      shift
      ;;
    --dry-run)
      dry_run=true
      shift
      ;;
    --yes | --no-confirm)
      yes=true
      shift
      ;;
    --allow-dirty)
      allow_dirty=true
      shift
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      error "Unsupported argument: $1"
      ;;
  esac
done

check_dependencies

if [ -n "$stylex_repo" ]; then
  derived_version=$(derive_stylex_version "$stylex_repo")

  if [ -z "$stylex_version" ]; then
    stylex_version="$derived_version"
  elif [ "$stylex_version" != "$derived_version" ]; then
    printf 'Using --version %s instead of derived StyleX repo version %s.\n' \
      "$stylex_version" \
      "$derived_version"
  fi
fi

if [ -z "$stylex_version" ]; then
  error "Provide --version or --stylex-repo."
fi

confirm_version
update_readmes "$stylex_version"
