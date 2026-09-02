"""Compare the addon built with and without its unused rlib.

A NAPI addon cannot be swapped inside one process on macOS, so each artifact is
benchmarked in its own process and the two configurations alternate. Each
configuration therefore has two runs, and a fixture only says something when
its two bands do not overlap.
"""

import re
import statistics
import sys

MEDIAN = re.compile(r"^(?P<name>.+?): median (?P<value>[\d.]+) (?P<unit>µs|ms|ns),")
SCALE = {"ns": 1e-3, "µs": 1.0, "ms": 1e3}


def parse(path):
    out = {}
    for line in open(path, encoding="utf-8"):
        found = MEDIAN.match(line.strip())
        if found:
            out[found.group("name")] = float(found.group("value")) * SCALE[found.group("unit")]
    return out


root = sys.argv[1]
both = [parse(f"{root}/bench-both-{n}.log") for n in (1, 2)]
only = [parse(f"{root}/bench-only-{n}.log") for n in (1, 2)]
names = [n for n in both[0] if all(n in run for run in both + only)]

print(f"{len(names)} fixtures, two runs per configuration\n")
print(f"  {'fixture':<44} {'rlib kept (us)':>18} {'rlib dropped (us)':>18}   change")

deltas, decided = [], []
for name in names:
    kept = [run[name] for run in both]
    drop = [run[name] for run in only]
    change = (statistics.mean(drop) / statistics.mean(kept) - 1) * 100
    deltas.append(change)
    overlap = not (min(drop) > max(kept) or max(drop) < min(kept))
    if not overlap:
        decided.append((name, change))
    flag = "" if overlap else ("  faster" if change < 0 else "  SLOWER")
    print(f"  {name:<44} {min(kept):8.1f}-{max(kept):8.1f} "
          f"{min(drop):8.1f}-{max(drop):8.1f}   {change:+6.2f}%{flag}")

print(f"\nmedian change {statistics.median(deltas):+.2f}%   "
      f"mean {statistics.mean(deltas):+.2f}%   "
      f"faster on {sum(1 for d in deltas if d < 0)}/{len(deltas)}")
print(f"decided by non-overlapping runs: {len(decided)} of {len(names)}")
if decided:
    values = [c for _, c in decided]
    print(f"  of those, median {statistics.median(values):+.2f}%, "
          f"{sum(1 for v in values if v < 0)} faster / {sum(1 for v in values if v > 0)} slower")
