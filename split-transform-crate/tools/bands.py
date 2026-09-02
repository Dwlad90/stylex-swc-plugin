"""Compare two configurations, each measured by several independent builds.

Every run reads against the same stored `parent-clean` baseline, so the drift
in that baseline is common to all of them and cancels when the two bands are
set side by side. A group says something only when the bands do not overlap.
"""

import statistics
import sys

from summarize import parse

CONTROLS = ["leg1c.log", "leg1c2.log", "leg1c3.log", "leg1c4.log"]
DROPPED = ["leg2.log", "leg2b.log", "leg2c.log"]


def group_median(run, group):
    values = [v for k, v in run.items() if k.split("/")[0] == group]
    return statistics.median(values)


controls = [dict(parse(p)) for p in CONTROLS]
dropped = [dict(parse(p)) for p in DROPPED]
branch = dict(parse("leg3.log"))

print("Whole-suite median against the stored parent-clean baseline\n")
for label, runs in (("cdylib kept  ", controls), ("cdylib dropped", dropped)):
    medians = [statistics.median(r.values()) for r in runs]
    print(f"  {label} n={len(runs)}  "
          + "  ".join(f"{m:+.2f}%" for m in medians)
          + f"   band {min(medians):+.2f}% to {max(medians):+.2f}%")
print(f"  branch (leg 3) n=1  {statistics.median(branch.values()):+.2f}%")

print("\n\nPer group\n")
print(f"  {'group':<24} {'cdylib kept (n=4)':>21}   {'cdylib dropped (n=3)':>21}   verdict")
for group in sorted({k.split("/")[0] for k in controls[0]}):
    kept = [group_median(r, group) for r in controls]
    drop = [group_median(r, group) for r in dropped]
    overlap = not (min(drop) > max(kept) or max(drop) < min(kept))
    if overlap:
        verdict = "no effect"
    else:
        gap = min(drop) - max(kept) if min(drop) > max(kept) else max(drop) - min(kept)
        verdict = f"{'slower' if gap > 0 else 'faster'} by >= {abs(gap):.1f} pts"
    print(f"  {group:<24} {min(kept):+7.2f} to {max(kept):+7.2f}   "
          f"{min(drop):+7.2f} to {max(drop):+7.2f}   {verdict}")
