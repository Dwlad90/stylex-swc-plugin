# Two reports drafted for facebook/stylex, not filed

Ticket 11 asks for one report per defect. Filing is outward-facing and
irreversible, so it does not happen without a go-ahead at the time. The
maintainer's instruction for this session was to draft them locally and file
neither.

| File                        | Defect                                     |
| --------------------------- | ------------------------------------------ |
| `01-redundant-wrapper.md`   | A ladder of exclusive breakpoints is wrapped in contradictory branches |
| `02-dropped-declaration.md` | Two entries canonicalizing to one query text lose one declaration |

Both carry a minimal input, the observed output, and the resolved version. They
are deliberately separate so either can be resolved without the other.

Once filed, put the numbers in
`crates/stylex-css-parser/docs/adr/0001-the-official-compilers-output-wins.md`,
in the paragraph that says they belong there, and in ticket 11.
