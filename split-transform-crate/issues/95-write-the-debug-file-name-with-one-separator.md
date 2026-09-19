# 95 — Write the debug file name with one separator

**What to fix:** two paths through the same `data-style-src` debug name
disagreed on Windows.

`get_short_path` normalises deliberately: it splits on
`std::path::MAIN_SEPARATOR` and rejoins with `"/"`. Two other branches did not
-- the package-relative branch of `create_short_filename` and the `root_dir`
branch of `get_short_path` itself both returned `to_string_lossy()` straight off
`strip_prefix`, so the separator was whatever the platform uses.

A file inside a resolved package therefore emitted
`data-style-src="pkg\components\Button.js"` on the Windows CI legs and
`pkg/components/Button.js` everywhere else, while a file outside one emitted
forward slashes on both. Any byte comparison of generated output -- a snapshot,
a fixture, a parity row -- passed on POSIX and failed on Windows alone, which is
this repository's known Windows-only failure shape.

**How it was answered.** One helper, `to_posix_path`, at all three sites. The
name is compared byte for byte, so it cannot be spelled two ways.

It rewrites the text rather than rejoining the parts. The first spelling
rejoined them, and two reviews found the same defect in it: rejoining answers a
root its own separator a second time, so `/a/b` came back `//a/b`, and on
Windows `C:\a\b` came back `C:/\/a/b`. The input is reachable -- a project
that declares an empty root directory strips nothing, because a path with no
parts is a prefix of every path, so the whole absolute path is handed over.
A source at `/repo/node_modules/pkg/src/a.js` under `rootDir: ""` was named
`pkg://repo/node_modules/pkg/src/a.js`.

Rejoining also normalised: it dropped a `.` part, collapsed a repeated
separator and lost a trailing one, so the name stopped reading back against the
file it names. Rewriting the text changes the separator and nothing else, which
makes it byte-identical to the old spelling everywhere the separator is already
`/`. Both shapes are pinned by the test.

Tested by `writes_a_name_with_one_separator_on_every_platform`, which builds the
path from its parts rather than from a spelling -- no literal reads the same on
both platforms, because a backslash separates two names on Windows and belongs
to a single name on POSIX. Assembling the parts puts the platform's own
separator in, and the check takes it back out. The case therefore fails on
Windows if the helper is removed, which is what no test could do before.

**Blocked by:** None.

**Status:** resolved

- [x] All three sites write the same separator
- [x] A case fails on Windows if the normalisation is removed
- [x] A path that names a root keeps one separator, not two
- [x] Nothing but the separator is rewritten
