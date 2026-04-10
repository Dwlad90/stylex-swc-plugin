#!/usr/bin/env python3
"""
Normalize indentation inside r#"..."# raw string literals in Rust test files.

Rules applied:
  - Base code indentation is exactly 2 spaces deeper than the `r#"` tag.
  - Features a built-in JS formatter: parses brackets and JSX tags to enforce
    strict 2-space internal relative indentation.
  - Automatically fixes misaligned properties, keyframes, and React nodes.
  - The closing `"#` delimiter is aligned perfectly with the opening `r#"` tag.
"""

import argparse
import os
import re
import textwrap
from pathlib import Path


def format_js_block(inner: str) -> str:
  dedented = textwrap.dedent(inner)
  lines = dedented.split('\n')
  formatted = []
  depth = 0

  for line in lines:
    s_line = line.strip()
    if not s_line:
      formatted.append('')
      continue

    # Remove strings and regex literals to avoid false bracket counting
    s_line_clean = re.sub(r"'[^']*'|\"[^\"]*\"|`[^`]*`", "", s_line)

    # Collapse common JS patterns to treat them as a single block indent level
    s_line_clean = s_line_clean.replace('({', '{').replace('})', '}')
    s_line_clean = s_line_clean.replace('([', '[').replace('])', ']')

    # 1. Check for leading closing brackets at the start of the line
    leading_brackets = re.match(r'^([\}\]\)]+)', s_line_clean)
    num_leading_brackets = len(leading_brackets.group(1)) if leading_brackets else 0

    # 2. Check for leading closing JSX tags (e.g. </>, </div>)
    leading_tag = re.match(r'^</[a-zA-Z0-9_]*>', s_line_clean)
    num_leading_tags = 1 if leading_tag else 0

    # Reduce depth BEFORE applying indent for the current line
    depth = max(0, depth - num_leading_brackets - num_leading_tags)

    # Apply standard 2-space indent
    formatted.append(('  ' * depth) + s_line)

    # 3. Calculate depth change for the NEXT line
    opens = len(re.findall(r'[\{\[\(]', s_line_clean))
    closes = len(re.findall(r'[\}\]\)]', s_line_clean))
    closes -= num_leading_brackets # Exclude already processed leading brackets

    # Match opening JSX tags like <div> or <>, ensuring they don't self-close />
    opens_tags = len(re.findall(r'<[a-zA-Z0-9_]+[^>]*(?<!/)>|<>\s*', s_line_clean))
    closes_tags = len(re.findall(r'</[a-zA-Z0-9_]*>', s_line_clean))
    closes_tags -= num_leading_tags # Exclude already processed leading tags

    depth += (opens - closes) + (opens_tags - closes_tags)
    depth = max(0, depth)

  return '\n'.join(formatted)


def normalize_raw_string_indent(content: str) -> str:
  pattern = re.compile(r'^([ \t]*)(r#"[^\S\n]*\n)(.*?)(\n[ \t]*"#)', re.DOTALL | re.MULTILINE)

  def fix_block(m: re.Match) -> str:
    leading_ws = m.group(1)
    open_delim = m.group(2)
    inner = m.group(3)

    if not inner.strip():
      return m.group(0)

    # Auto-format the JS block with proper bracket/tag indentation
    formatted_inner = format_js_block(inner)

    # Base indent for the JS code is the r#" indent + 2 spaces
    base_indent_str = leading_ws + '  '
    final_lines = []

    for line in formatted_inner.split('\n'):
      if line.strip():
        final_lines.append(base_indent_str + line)
      else:
        final_lines.append('')

    new_inner = '\n'.join(final_lines)

    # Rebuild block, aligning the closing delimiter with the opening r#"
    return f'{leading_ws}{open_delim}{new_inner}\n{leading_ws}"#'

  return pattern.sub(fix_block, content)


def process_directory(test_dir_str: str) -> None:
  test_dir = Path(test_dir_str)
  changed: list[Path] = []

  for root, dirs, files in os.walk(test_dir):
    # Never descend into snapshot directories
    if '__swc_snapshots__' in dirs:
      dirs.remove('__swc_snapshots__')

    root_path = Path(root)
    for fname in files:
      if not fname.endswith('.rs') or fname == 'macros.rs':
        continue

      fpath = root_path / fname
      original = fpath.read_text(encoding='utf-8')
      normalized = normalize_raw_string_indent(original)

      if normalized != original:
        fpath.write_text(normalized, encoding='utf-8')
        changed.append(fpath)

  if changed:
    print(f"Fixed {len(changed)} file(s):")
    for p in sorted(changed):
      print(f"  {p}")
  else:
    print("All files already have normalized indentation.")


def main() -> None:
  parser = argparse.ArgumentParser(
    description=__doc__,
    formatter_class=argparse.RawDescriptionHelpFormatter
  )
  parser.add_argument(
    'test_dir',
    nargs='?',
    default='tests',
    help='Root directory of Rust test files',
  )
  args = parser.parse_args()

  test_dir = args.test_dir.replace('test_dir=', '')

  print(f"Processing test directory: {test_dir}")
  process_directory(test_dir)


if __name__ == '__main__':
  main()
