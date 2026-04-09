#!/usr/bin/env python3
"""
Normalize indentation inside r#"..."# raw string literals in Rust test files.

Rules applied to each raw-string block:
  - The first line (immediately after r#") is kept as-is (typically empty).
  - Non-empty content lines are re-indented to a 4-space base, preserving
    their relative indentation from the original minimum indent level.
  - The last line (before the closing "#) is set to exactly 2 spaces.
  - Blocks whose minimum indent is already ≤ 4 spaces are skipped (already
    normalized).

Usage:
    python3 scripts/fix-test-indent.py [TEST_DIR]

    TEST_DIR defaults to crates/stylex-transform/tests
"""

import argparse
import os
import re


def normalize_raw_string_indent(content: str) -> str:
    pattern = re.compile(r'(r#")(.*?)("# *)', re.DOTALL)

    def fix_block(m: re.Match) -> str:
        open_delim = m.group(1)   # r#"
        inner = m.group(2)        # content between r#" and "#
        close_delim = m.group(3)  # "#  (may have trailing spaces)

        lines = inner.split('\n')
        non_empty = [l for l in lines if l.strip()]
        if not non_empty:
            return m.group(0)

        min_indent = min(len(l) - len(l.lstrip()) for l in non_empty)

        # Skip blocks that are already at ≤ 4-space base indent.
        if min_indent <= 4:
            return m.group(0)

        new_lines = []
        for i, line in enumerate(lines):
            if i == 0:
                # Keep the opening newline as-is.
                new_lines.append(line)
            elif i == len(lines) - 1:
                # Closing "#  line — always 2 spaces.
                new_lines.append('  ')
            elif line.strip() == '':
                new_lines.append('')
            else:
                stripped = line[min_indent:] if len(line) >= min_indent else line.lstrip()
                new_lines.append('    ' + stripped)

        return open_delim + '\n'.join(new_lines) + close_delim.strip()

    return pattern.sub(fix_block, content)


def process_directory(test_dir: str) -> None:
    changed: list[str] = []

    for root, dirs, files in os.walk(test_dir):
        # Never descend into snapshot directories.
        dirs[:] = [d for d in dirs if d != '__swc_snapshots__']

        for fname in files:
            if not fname.endswith('.rs') or fname == 'macros.rs':
                continue

            fpath = os.path.join(root, fname)
            with open(fpath, 'r', encoding='utf-8') as fh:
                original = fh.read()

            normalized = normalize_raw_string_indent(original)

            if normalized != original:
                with open(fpath, 'w', encoding='utf-8') as fh:
                    fh.write(normalized)
                changed.append(fpath)

    if changed:
        print(f"Fixed {len(changed)} file(s):")
        for p in sorted(changed):
            print(f"  {p}")
    else:
        print("All files already have normalized indentation.")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        'test_dir',
        nargs='?',
        default='crates/stylex-transform/tests',
        help='Root directory of Rust test files (default: crates/stylex-transform/tests)',
    )
    args = parser.parse_args()
    process_directory(args.test_dir)


if __name__ == '__main__':
    main()
