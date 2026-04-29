#!/bin/bash

set -euo pipefail

if [ ! -f Cargo.toml ]; then
  exit 0
fi

benches=()
while IFS= read -r bench; do
  benches+=("$bench")
done < <(
  awk '
    /^\[\[bench\]\]/ {
      in_bench = 1
      next
    }

    /^\[/ {
      in_bench = 0
    }

    in_bench && /^[[:space:]]*name[[:space:]]*=/ {
      name = $0
      sub(/^[^=]*=[[:space:]]*/, "", name)
      gsub(/"/, "", name)
      print name
    }
  ' Cargo.toml
)

if [ "${#benches[@]}" -eq 0 ]; then
  exit 0
fi

bench_sample_size="${BENCH_SAMPLE_SIZE:-10}"
bench_warm_up_time="${BENCH_WARM_UP_TIME:-1}"
bench_measurement_time="${BENCH_MEASUREMENT_TIME:-2}"

for bench in "${benches[@]}"; do
  cargo bench --bench "$bench" -- \
    --sample-size "$bench_sample_size" \
    --warm-up-time "$bench_warm_up_time" \
    --measurement-time "$bench_measurement_time" \
    --noplot
done
