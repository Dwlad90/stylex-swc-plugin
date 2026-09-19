#!/usr/bin/env python3
"""Reads the hit rate a longer-lived styleq merger would have.

Written for ticket 60. The probe it reads is two `eprintln!` lines and a
thread-local merger, described in `styleq_scope.md`; this script turns the
probe's log into one hit rate per merger lifetime.

A lookup is the key of one compiled style, and the cache is a chain, so what
names an entry is the whole path from the start of the merge, not the key
alone. That is why this replays the paths against a trie rather than counting
repeated keys: two merges that read the same style behind different styles ask
different questions of the cache.
"""

import re
import sys

# The three lifetimes compared. A merge is what the compiler builds today; the
# process is the most a merger could ever be given, so it bounds the other two.
SCOPES = ("merge", "file", "process")

# The suite prints its own lines into the same stream, so a marker is matched
# anywhere in a line rather than at its start.
PROBE = re.compile(r"STYLEQPROBE (FILE|MERGE|K \S+)")

# A key is a 128-bit hash, so at most 32 hex digits. Rust prints it without
# leading zeros, which is why the width is a bound and not an equality.
KEY_DIGITS = re.compile(r"^[0-9a-f]{1,32}$")


def read_merges(path):
  """The key path of every merge the log attributes to a file.

  A merge that stands before the first file marker was asked by a unit test
  that calls the merger directly. It belongs to no file, so no file-scoped
  merger would ever hold it, and pooling such merges into one bucket makes them
  look like one very long file. They are counted and left out.
  """
  try:
    log = open(path, encoding="utf-8").read()
  except OSError as error:
    sys.exit(f"cannot read the probe log {path}: {error}")

  # A log that was cut short ends inside a line. Its last key would then be
  # shorter than the one the walk printed, and every lookup behind it would
  # count as a miss that never happened.
  if log and not log.endswith("\n"):
    sys.exit("the log ends inside a line: it was cut short")

  merges = []
  file_index = -1
  current = None
  orphans = 0
  fileless = 0
  started = False

  previous = None

  for token in PROBE.findall(log):
    if token == "FILE":
      # A file is visited as a program and again as a module, so two markers in
      # a row still name one file.
      if previous != "FILE":
        file_index += 1
    elif token == "MERGE":
      started = True

      if file_index < 0:
        fileless += 1
        current = None
        previous = token
        continue

      current = (file_index, [])
      merges.append(current)
    elif current is None:
      # A key with no merge before it at all means the log lost its head, and
      # the merge that key belongs to cannot be rebuilt. A key after a merge
      # that was left out belongs to that merge and is left out with it.
      if not started:
        orphans += 1
    else:
      key = token.split()[-1]

      if not KEY_DIGITS.match(key):
        sys.exit(f"{key} is not a key: the log is not a probe log")

      current[1].append(key)

    previous = token

  if fileless:
    merge = "merge" if fileless == 1 else "merges"
    print(f"{fileless} {merge} belong to no file and are left out\n")

  if orphans:
    stand = "key stands" if orphans == 1 else "keys stand"
    sys.exit(f"{orphans} {stand} before the first merge: the log lost its head")

  return merges


def rate(merges, scope):
  """Lookups, hits and stored entries for one merger lifetime."""
  seen = set()
  lookups = hits = entries = 0

  for file_index, keys in merges:
    # A merger built per merge starts every merge with an empty chain, so the
    # entries of the merge before it are counted and then thrown away.
    if scope == "merge":
      seen = set()

    owner = file_index if scope == "file" else 0
    path = ()

    for key in keys:
      path += (key,)
      lookups += 1

      if (owner, path) in seen:
        hits += 1
      else:
        seen.add((owner, path))
        entries += 1

  return lookups, hits, entries


def main():
  if len(sys.argv) != 2:
    sys.exit("usage: styleq_scope.py <probe-log>")

  merges = read_merges(sys.argv[1])

  if not merges:
    sys.exit("the log holds no merge: is the probe still in the tree?")

  # A merge with no key means the walk computed none, which is what the identity
  # key does. The note says to make the key a hash for the run.
  if not any(keys for _, keys in merges):
    sys.exit("the log holds no key: is the walk still making an identity key?")

  print(f"{'merger lives for':<18}{'lookups':>9}{'hits':>7}{'rate':>8}{'entries':>9}")

  for scope in SCOPES:
    lookups, hits, entries = rate(merges, scope)
    print(f"{scope:<18}{lookups:>9}{hits:>7}{hits / lookups:>7.1%}{entries:>9}")

  files = len({file_index for file_index, _ in merges})
  file_word = "file" if files == 1 else "files"
  print(f"\n{len(merges)} merges over {files} {file_word} that merge")


if __name__ == "__main__":
  main()
