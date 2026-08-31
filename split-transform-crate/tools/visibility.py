"""Give every item in stylex-state the narrowest visibility that still compiles.

Demotes the whole crate to `pub(crate)`, then promotes back only the items the
workspace fails to reach. The compiler decides, so nothing is exported on a
guess.
"""
import pathlib, re, subprocess, sys

SRC = pathlib.Path('crates/stylex-state/src')
FILES = [f for f in SRC.rglob('*.rs') if f.name != 'lib.rs' and 'tests' not in f.parts]

if sys.argv[1:] == ['demote']:
    for f in FILES:
        t = f.read_text()
        # `pub mod` stays: the crate root re-declares those modules publicly.
        t = re.sub(r'\bpub (?!\(|mod\b|use\b)', 'pub(crate) ', t)
        f.write_text(t)
    print('demoted')
    raise SystemExit

# "field `value` of struct `EvaluateResult` is private" names two things; the
# first backtick on the line is the one that has to be promoted.
LINE_RE = re.compile(r'^error\[E06(?:03|16|24)\]:.*is private.*$', re.M)
FIRST_NAME_RE = re.compile(r'`([A-Za-z_][A-Za-z0-9_]*)`')

for round_no in range(1, 15):
    out = subprocess.run(['cargo', 'check', '--workspace', '--all-features', '--tests'],
                         capture_output=True, text=True).stderr
    names = set()
    for line in LINE_RE.findall(out):
        m = FIRST_NAME_RE.search(line)
        if m: names.add(m.group(1))
    if not names:
        print(f'round {round_no}: clean')
        break
    promoted = 0
    for f in FILES:
        t = old = f.read_text()
        for n in sorted(names):
            t = re.sub(rf'\bpub\(crate\) (?=(?:unsafe |const |async )*(?:fn|struct|enum|type|trait|static|const|mod) {re.escape(n)}\b)', 'pub ', t)
            t = re.sub(rf'\bpub\(crate\) (?={re.escape(n)}\s*:)', 'pub ', t)   # struct fields
        if t != old:
            f.write_text(t); promoted += 1
    print(f'round {round_no}: {len(names)} private names, touched {promoted} files')
    if promoted == 0:
        print('stuck on:', sorted(names)); break
