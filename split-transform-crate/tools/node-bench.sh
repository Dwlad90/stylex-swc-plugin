#!/bin/bash
# Price the addon's crate-type in production terms: build the `.node` each way
# and benchmark it through the published entry point.
#
# The two configurations alternate, because a NAPI addon cannot be swapped
# inside one process on macOS and separate processes drift over a long run.
set -euo pipefail

pkg=/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix/crates/stylex-rs-compiler
out="$1"
orig="$2"
mkdir -p "$out"

set_crate_type() {
  python3 - "$pkg/Cargo.toml" "$1" <<'PY'
import sys
path, value = sys.argv[1], sys.argv[2]
text = open(path).read()
for old in ('crate-type = ["cdylib", "rlib"]', 'crate-type = ["cdylib"]'):
    if old in text:
        open(path, "w").write(text.replace(old, value))
        break
else:
    raise SystemExit("crate-type line not found")
PY
}

for round in 1 2; do
  for config in 'crate-type = ["cdylib", "rlib"]:both' 'crate-type = ["cdylib"]:only'; do
    value="${config%:*}"
    tag="${config##*:}"
    echo "=== round $round / $tag"
    set_crate_type "$value"
    (cd "$pkg" && pnpm run build >/dev/null 2>&1)
    ls -l "$pkg/dist"/*.node | awk '{print "node bytes:", $5}'
    (cd "$pkg" && pnpm run bench > "$out/bench-$tag-$round.log" 2>&1) || echo "bench failed"
    cp "$pkg/benchmark/results/raw-stats.v1.json" "$out/raw-$tag-$round.json"
  done
done

# Leave the tree exactly as it was found.
cp "$orig" "$pkg/Cargo.toml"
echo "=== restored"
grep -n 'crate-type' "$pkg/Cargo.toml"
