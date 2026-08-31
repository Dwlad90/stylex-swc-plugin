"""Rewrite `use crate::shared::...` trees after modules moved to stylex_state.

Expands every nested brace group to leaf paths, then re-groups the leaves by
which crate now owns them. A literal string rewrite cannot do this: the moved
module name is usually inside a brace group, not in the statement's prefix.
"""
import pathlib, re, sys

EXTERNAL = 'stylex_transform::'
MOVED = {
  'crate::shared::structures::state_manager': 'stylex_state::state_manager',
  'crate::shared::structures::types': 'stylex_state::types',
  'crate::shared::structures::seen_value': 'stylex_state::seen_value',
  'crate::shared::structures::state': 'stylex_state::state',
  'crate::shared::structures::functions': 'stylex_state::functions',
  'crate::shared::structures::theme_ref': 'stylex_state::theme_ref',
  'crate::shared::enums::data_structures::evaluate_result_value': 'stylex_state::evaluate_result_value',
  'crate::shared::enums::data_structures::flat_compiled_styles_value': 'stylex_state::flat_compiled_styles_value',
  'crate::shared::utils::common': 'stylex_state::common',
}

def split_top(s):
    """Split a brace group's body on commas that are not inside nested braces."""
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

def expand(prefix, tree):
    """Expand one use-tree into a list of full leaf paths."""
    tree = tree.strip()
    if tree.startswith('{'):
        assert tree.endswith('}')
        leaves = []
        for part in split_top(tree[1:-1]):
            leaves += expand(prefix, part)
        return leaves
    if '{' in tree:
        head, rest = tree.split('{', 1)
        return expand(prefix + head, '{' + rest)
    if tree == 'self':
        return [prefix.rstrip(':')]
    return [prefix + tree]

def relabel(leaf):
    """Return (crate_key, path_without_crate_prefix) for one leaf."""
    for old, new in MOVED.items():
        if leaf == old or leaf.startswith(old + '::'):
            leaf = new + leaf[len(old):]
            break
    if leaf.startswith('stylex_state::'):
        return 'stylex_state', leaf[len('stylex_state::'):]
    return 'crate', leaf[len('crate::'):] if leaf.startswith('crate::') else leaf

USE_RE = re.compile(r'^([ \t]*)use ((?:crate|stylex_transform)::[^;]*?);$', re.M | re.S)

def rewrite(text):
    def repl(m):
        indent, tree = m.group(1), m.group(2)
        external = tree.startswith(EXTERNAL)
        if external:
            tree = 'crate::' + tree[len(EXTERNAL):]
        leaves = expand('', tree)
        groups = {}
        for leaf in leaves:
            key, path = relabel(leaf)
            groups.setdefault(key, []).append(path)
        if 'stylex_state' not in groups:
            return m.group(0)
        out = []
        for key in ('crate', 'stylex_state'):
            paths = sorted(set(groups.get(key, [])))
            if not paths: continue
            root = ('stylex_transform' if external else 'crate') if key == 'crate' else 'stylex_state'
            if len(paths) == 1:
                out.append(f'{indent}use {root}::{paths[0]};')
            else:
                out.append(f'{indent}use {root}::{{{", ".join(paths)}}};')
        return '\n'.join(out)
    return USE_RE.sub(repl, text)

changed = 0
for arg in sys.argv[1:]:
    for f in pathlib.Path(arg).rglob('*.rs'):
        if not f.is_file() or 'target' in f.parts or 'node_modules' in f.parts:
            continue
        t = f.read_text()
        n = rewrite(t)
        if n != t:
            f.write_text(n); changed += 1
print(f'rewrote {changed} files')
