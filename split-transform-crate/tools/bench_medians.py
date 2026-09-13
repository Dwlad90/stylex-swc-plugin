"""Turn a criterion bench log into one median per benchmark id.

Criterion prints the benchmark id, then a `time:` line holding a low, a median
and a high estimate. A short id shares its line with `time:`; a long one sits on
a line of its own. Both spellings appear in one log, so both are matched.

Two log shapes are read. `rebaseline.sh` separates targets with `==> pkg bench`
and writes a header block. The pre-split log, `baseline/benches.log`, separates
them with `===== bench =====` and has no header. Reading both is what lets the
summariser be checked against a known answer.

Usage: bench_medians.py <log-file>
"""

import re
import sys

TIME = re.compile(r"time:\s*\[[\d.]+ \S+\s+([\d.]+ \S+)\s+[\d.]+ \S+\]")
OLD_MARKER = re.compile(r"^=+\s*(\S+)\s*=+$")

# Lines criterion and cargo start at column 0 that are not benchmark ids.
NOISE = (
  "==>", "=====", "Benchmarking", "Found", "Warning", "warning", "Compiling",
  "Running", "Finished", "error", "Gnuplot", "Performance",
)


def parse(path):
  """Yield (bench target, benchmark id, median) in the order measured."""
  target, pending = None, None

  with open(path, encoding="utf-8", errors="replace") as handle:
    for line in handle:
      line = line.rstrip()

      if line.startswith("==> "):
        target = line.split()[2]
        continue

      marker = OLD_MARKER.match(line)
      if marker:
        target = marker.group(1)
        continue

      found = TIME.search(line)
      if found:
        # An id on the same line sits before `time:`; otherwise use the last
        # id seen on a line of its own.
        head = line[: found.start()].strip()
        yield target, head or pending, found.group(1)
        pending = None
        continue

      if line and not line[0].isspace() and not line.startswith(NOISE):
        pending = line


def read_header(path):
  """Read the `rebaseline.sh` header block. A log without one gives {}."""
  header = {}

  with open(path, encoding="utf-8", errors="replace") as handle:
    first = handle.readline()
    if not first.startswith("criterion baseline"):
      return header

    found = re.search(r"'([^']+)'", first)
    header["baseline"] = found.group(1) if found else "unknown"

    for line in handle:
      if not line.strip():
        break
      key, _, value = line.partition(":")
      header[key.strip()] = value.strip()

  return header


def main():
  if len(sys.argv) != 2:
    sys.exit("usage: bench_medians.py <log-file>")

  path = sys.argv[1]
  rows = list(parse(path))
  header = read_header(path)

  commit = header.get("commit", "unknown")[:9]
  name = header.get("baseline", "unknown")
  print(f"Criterion medians at commit {commit}, criterion baseline '{name}'.")

  target = None
  for row_target, bench_id, median in rows:
    if row_target != target:
      target = row_target
      print(f"\n{target}")
    # Pad to a column, but always leave a separator: seven ids are longer than
    # the column, and a value written against the id cannot be read back.
    print(f"  {bench_id.ljust(58)}  {median}")

  print(f"\n{len(rows)} measurements")


if __name__ == "__main__":
  main()
