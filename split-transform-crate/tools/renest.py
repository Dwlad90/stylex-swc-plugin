"""Re-nest the flat use lists the path rewrite produced, and merge duplicates.

Turns `use stylex_state::{common::a, common::b, types::c};` back into the
nested form the rest of the file is written in.
"""
import pathlib, re, sys, collections

ROOTS = ('crate', 'stylex_state', 'stylex_transform')

def split_top(s):
    out, depth, cur = [], 0, ''
    for ch in s:
        if ch == '{': depth += 1
        elif ch == '}': depth -= 1
        if ch == ',' and depth == 0:
            out.append(cur); cur = ''
        else:
            cur += ch
    if cur.strip(): out.append(cur)
    return [x.strip() for x in out if x.strip()]

def leaves_of(tree, prefix=''):
    tree = tree.strip()
    if tree.startswith('{'):
        return [l for part in split_top(tree[1:-1]) for l in leaves_of(part, prefix)]
    if '{' in tree:
        head, rest = tree.split('{', 1)
        return leaves_of('{' + rest, prefix + head)
    return [prefix + tree]

def build(paths):
    """Nest a set of `a::b::c` leaves into a brace tree."""
    root = {}
    for p in paths:
        head, _, rest = p.partition('::')
        root.setdefault(head, set())
        if rest:
            root[head].add(rest)
        else:
            root[head].add('')      # the segment is itself imported
    parts = []
    for head in sorted(root):
        subs = root[head]
        if subs == {''}:
            parts.append(head)
        elif '' in subs:
            inner = build(sorted(s for s in subs if s))
            parts.append(f'{head}::{{self, {inner[1:-1]}}}' if inner.startswith('{') else f'{head}::{{self, {inner}}}')
        else:
            inner = build(sorted(subs))
            parts.append(f'{head}::{inner}')
    return parts[0] if len(parts) == 1 else '{' + ', '.join(parts) + '}'

USE_RE = re.compile(rf'^use ({"|".join(ROOTS)})::([^;]*?);$', re.M | re.S)

changed = 0
for arg in sys.argv[1:]:
    for f in pathlib.Path(arg).rglob('*.rs'):
        if not f.is_file() or 'target' in f.parts or 'node_modules' in f.parts:
            continue
        text = f.read_text()
        by_root = collections.defaultdict(set)
        spans = []
        for m in USE_RE.finditer(text):
            by_root[m.group(1)] |= set(leaves_of(m.group(2)))
            spans.append(m.span())
        if not spans:
            continue
        merged = '\n'.join(
            f'use {root}::{build(sorted(by_root[root]))};' for root in ROOTS if root in by_root)
        # Replace the first statement with the merged block, drop the rest.
        out, last = [], 0
        for i, (a, b) in enumerate(spans):
            out.append(text[last:a])
            if i == 0:
                out.append(merged)
            else:
                out.append('\x00')      # marked for removal with its newline
            last = b
        out.append(text[last:])
        new = ''.join(out).replace('\x00\n', '').replace('\x00', '')
        if new != text:
            f.write_text(new); changed += 1
print(f'renested {changed} files')
