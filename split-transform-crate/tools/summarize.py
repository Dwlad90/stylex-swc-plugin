"""Summarise a criterion `--baseline` log: per-measurement and per-group medians.

Criterion prints the benchmark id, then a `change:` line whose middle figure is
the point estimate. A short id shares its line with `time:`; a long one sits on
a line of its own. Both spellings appear in one log, so both are matched.
"""

import re
import statistics
import sys

# The minus sign criterion prints is U+2212, not an ASCII hyphen.
CHANGE = re.compile(r"change:\s*\[\s*[-+−]?[\d.]+%\s+([-+−]?)([\d.]+)%")
BENCH_ID = re.compile(r"^(?P<id>[A-Za-z][\w]*(?:/[^\s]+)+)(?:\s+time:|\s*$)")


def parse(path):
    results, current = [], None
    with open(path, encoding="utf-8", errors="replace") as handle:
        for line in handle:
            found = BENCH_ID.match(line.rstrip())
            if found:
                current = found.group("id")
                continue
            change = CHANGE.search(line)
            if change and current:
                value = float(change.group(2))
                results.append((current, -value if change.group(1) in "-−" else value))
                current = None
    return results


def report(paths):
    """Print the per-measurement and per-group summary for each log."""
    for path in paths:
        results = parse(path)
        values = [value for _, value in results]
        print(f"\n===== {path} — {len(results)} measurements =====")
        if not values:
            continue
        print(f"median {statistics.median(values):+.2f}%   "
              f"range {min(values):+.2f}% to {max(values):+.2f}%   "
              f"faster {sum(1 for v in values if v < 0)}/{len(values)}   "
              f"within ±4% {sum(1 for v in values if abs(v) <= 4)}/{len(values)}")
        groups = {}
        for name, value in results:
            groups.setdefault(name.split("/")[0], []).append(value)
        print("\n  group                      n   median")
        for name in sorted(groups, key=lambda k: -statistics.median(groups[k])):
            print(f"  {name:<24} {len(groups[name]):>3}   {statistics.median(groups[name]):+7.2f}%")
        print("\n  per measurement:")
        for name, value in results:
            print(f"    {name:<44} {value:+7.2f}%")


if __name__ == "__main__":
    report(sys.argv[1:])
