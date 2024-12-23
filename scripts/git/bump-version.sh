#!/usr/bin/env bash

set -e

# Parse arguments
NEW_VERSION=$1

# Update Cargo.toml files
find . -name 'node_modules' -prune -o -name 'Cargo.toml' -print | while read -r cargo_file; do
    echo "Updating $cargo_file"
    sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$cargo_file"
    rm "${cargo_file}.bak"
done

# Update package.json files
find . -name 'node_modules' -prune -o -name 'package.json' -print | while read -r package_file; do
    echo "Updating $package_file"
    jq --arg version "$NEW_VERSION" '
    def update_deps:
        with_entries(
            if (.value | type == "string") and (.value | test("^file:|^link:|^workspace:") | not) and (.key | startswith("@stylexswc/")) then
                .value = $version
            else
                .
            end
        );

    .version = $version |
    if has("dependencies") then .dependencies |= update_deps else . end |
    if has("devDependencies") then .devDependencies |= update_deps else . end |
    if has("peerDependencies") then .peerDependencies |= update_deps else . end
    ' "$package_file" >"${package_file}.tmp" && mv "${package_file}.tmp" "$package_file"
done

# Update README.md
if echo "$NEW_VERSION" | grep -q '^[0-9]\+\.[0-9]\+\.[0-9]\+$'; then
    sed -i.bak "s|\(](.*/\)[0-9]\+\.[0-9]\+\.[0-9]\+\(.*)\)|\1${NEW_VERSION}\2|" README.md
    rm "README.md.bak"
fi
