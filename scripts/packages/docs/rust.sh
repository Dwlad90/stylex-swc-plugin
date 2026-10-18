#!/usr/bin/env sh

script_dir="$(cd "$(dirname "$0")" && pwd)"

open_docs=""

for arg in "$@"; do
  if [ "$arg" = "--open" ]; then
    open_docs="--open"
    # shellcheck disable=SC3060
    set -- "${@/--open/}"
  fi
done

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

# shellcheck disable=1091
. "$script_dir"/../../parse-args.sh
: "${build_rust:=false}"


if [ "$build_rust" = true ]; then
  # Check if running in GitHub Actions CI
  if [ "$GITHUB_ACTIONS" = "true" ]; then
    verbose_flag="--verbose"
  else
    verbose_flag=""
  fi
  # These docs include private items, so a link from a public item to a
  # private item resolves on the site. Allow the lint that reports it.
  export RUSTDOCFLAGS="--enable-index-page -Zunstable-options -A rustdoc::private_intra_doc_links"

  # Rustdoc links to another crate only if the docs directory of that crate
  # exists when rustdoc starts. If the directory is missing, the link becomes
  # plain text. Rustdoc gives no warning about it. With `--no-deps`, Cargo does
  # not make the docs of a crate wait for the docs of the crates that it uses.
  # Cargo also deletes the docs directory of a crate when it documents that
  # crate again. For these reasons, one `cargo doc` for the full workspace
  # loses links at random.
  #
  # Document the workspace in levels, one `cargo doc` for each level. A level
  # holds the crates whose workspace dependencies are all in lower levels. Each
  # line of the output holds the `-p` list of one level. The lowest level comes
  # first. Dev-dependencies do not count, because the docs of a library cannot
  # link to them.
  metadata=$(cargo metadata --format-version 1) || handle_error "Failed to read the cargo metadata"
  levels=$(printf '%s' "$metadata" | jq -r '
    .workspace_members as $members
    | (.packages | map({key: .id, value: .name}) | from_entries) as $names
    | (
        .resolve.nodes
        | map(select(.id as $id | $members | index($id)))
        | map({
            key: .id,
            value: [
              .deps[]
              | select(.pkg as $pkg | $members | index($pkg))
              | select(any(.dep_kinds[]; .kind != "dev"))
              | .pkg
            ]
          })
        | from_entries
      ) as $deps
    # The level of a crate is one more than the highest level of its
    # dependencies, and 0 when it has none.
    | def level:
        if ($deps[.] | length) == 0 then 0 else ([$deps[.][] | level] | max) + 1 end;
    $members
    | group_by(level)
    | .[]
    | map("-p " + $names[.])
    | join(" ")
  ') || handle_error "Failed to find the order of the workspace crates"
  # A `cargo doc` with no `-p` documents the full workspace in one call. Do not
  # let an empty list start that call.
  [ -n "$levels" ] || handle_error "Found no workspace crates to document"

  doc_args="--no-deps --document-private-items $verbose_flag"

  while IFS= read -r packages; do
    # shellcheck disable=SC2086
    cargo doc $doc_args $packages || handle_error "Failed to build cargo docs"
  done <<EOF
$levels
EOF

  if [ -n "$open_docs" ]; then
    # This call uses the same arguments as the levels. For this reason, all
    # docs are current, and the call only opens them.
    # shellcheck disable=SC2086
    cargo doc $doc_args $open_docs || handle_error "Failed to open cargo docs"
  fi
fi
