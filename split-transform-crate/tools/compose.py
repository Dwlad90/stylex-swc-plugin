"""Compose two legs measured against one shared criterion baseline.

Each leg reports a median ratio against `parent-clean`, so the ratio of one leg
to the other follows from the two, and no third bench run is needed:

    b_vs_a = (1 + b) / (1 + a) - 1
"""

import statistics
import sys

from summarize import parse

base, test = (dict(parse(path)) for path in sys.argv[1:3])
label = sys.argv[3] if len(sys.argv) > 3 else "test vs base"

rows = []
for name, base_change in base.items():
    if name in test:
        rows.append((name, ((1 + test[name] / 100) / (1 + base_change / 100) - 1) * 100))

values = [value for _, value in rows]
print(f"===== {label} — {len(rows)} measurements =====")
print(f"median {statistics.median(values):+.2f}%   "
      f"range {min(values):+.2f}% to {max(values):+.2f}%   "
      f"faster {sum(1 for v in values if v < 0)}/{len(values)}   "
      f"within ±4% {sum(1 for v in values if abs(v) <= 4)}/{len(values)}")

groups = {}
for name, value in rows:
    groups.setdefault(name.split("/")[0], []).append(value)
print("\n  group                      n   median")
for name in sorted(groups, key=lambda k: -statistics.median(groups[k])):
    print(f"  {name:<24} {len(groups[name]):>3}   {statistics.median(groups[name]):+7.2f}%")
print("\n  per measurement:")
for name, value in rows:
    print(f"    {name:<44} {value:+7.2f}%")
