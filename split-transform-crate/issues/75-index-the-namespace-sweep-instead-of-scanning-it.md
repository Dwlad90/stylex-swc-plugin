# 75 — Index the namespace sweep instead of scanning it

**What to build:** An index for the two linear scans the finalize sweep makes
for every property of every style variable, so the sweep costs what the module
holds rather than the square of it.

**Where the cost is:**
`crates/stylex-transform/src/transform/visit_mut/visit_mut_var_declarator.rs`.

- Line 236 asks `namespace_to_keep.contains(&namespace_name)` for each
  property. `namespace_to_keep` is a vector, so this reads it from the start
  every time. The caller at line 99 builds that list and can as easily build a
  set.
- Line 250 walks `self.state.style_vars_to_keep` from the start for each kept
  namespace, to gather the entries recorded against one declaration and one
  name. That list holds every recorded entry in the file.

Together the sweep costs properties times namespaces times recorded entries. A
module with a handful of style variables does not notice it. One with many
pays for it twice over.

**What to do:** Index `style_vars_to_keep` by its declaration and name once per
declarator, and take the kept namespaces as a set from the caller that already
builds the list. Neither changes what the sweep answers, so the cases that
cover it stand as they are.

**Where this came from:** The performance review of
[65](./65-cover-the-transform-walk.md). It is older than that ticket, which is
why it is filed rather than fixed there.

**Status:** resolved

- [x] One declaration's entries are gathered in one pass, keyed by namespace
      name, and the caller is handed a set rather than a list.
- [x] The three answers stay apart, each with a case: names recorded, the
      namespace kept whole, and nothing recorded at all.
- [x] No figure is claimed. The work removed is a scan of every recorded entry
      per namespace replaced by one pass per declaration, which is a shape
      change rather than a measured delta, and
      `guidelines/PERFORMANCE.md` records that this machine cannot resolve a
      delta of the size this would be.

## What this ticket superseded

`7a335ffac perf(stylex_transform): stop the null sweep at the answer that
settles it` added a `break` to the scan this change replaced. Two commits later
the scan was gone, so its claim describes code that no longer exists. The
behaviour it named survives as the `if let Some(nulls) = nulls` guard in the
gathering pass -- an entry that keeps a namespace whole stays that way, and a
list arriving after it adds nothing -- but the saving it claimed does not, and
nothing was left behind to delete.

## Comments

The entries of one declaration are gathered in one pass now, into a map keyed
by the namespace name, and the caller hands the sweep a set instead of a list.
The sweep costs what the module holds.

The map leaves out the entries whose recorded name is a whole style variable
rather than a namespace. The sweep asks only for namespaces, so those entries
could never answer it, and the old nested scan skipped them by the same test.

Three answers had to stay apart, and a case says each one. A namespace with
names recorded against it keeps those names. A namespace recorded as kept whole
keeps every declaration, whichever order the entries arrived in -- one reader
can name a declaration while another reads the namespace whole, and the two are
recorded independently. A namespace with nothing recorded loses every null
declaration in it. The third was the one the first draft of this change got
wrong: an absent entry is not the same answer as an entry that keeps the
namespace whole.

Each case was checked against the defect it guards, by making the defect and
watching the case fail.
