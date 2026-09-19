# 76 — Read a declaration before copying it to expand it

**What to build:** The shape test that lets the null-declaration guard skip the
copy it makes for every property it reads.

**Where the cost is:**
`crates/stylex-transform/src/transform/visit_mut/visit_mut_var_declarator.rs`,
around line 351. The retain guard copies each property so that it can expand a
legacy shorthand on the copy, and then reads one boolean off the result:

```rust
let mut prop = prop.clone();
expand_shorthand_prop(&mut prop);
```

The copy is dropped at the end of the guard. Every property of every style
object in the module pays for it, including the great majority that are not
null declarations and cannot become one.

**What to do:** Read the key and the value first, and make the copy only where
the property could answer yes -- a name whose value is a null literal, or a
shorthand that can expand into one. The guard must keep answering what it
answers now, so the cases that cover it are the check.

**Where this came from:** The performance review of
[65](./65-cover-the-transform-walk.md). The line is older than that ticket,
which is why it is filed rather than fixed there.

**Status:** resolved

- [x] The copy is made only where the property could answer yes: a name whose
      value is a null literal, or a shorthand that can expand into one.
- [x] The guard answers what it answered before, which the cases that cover it
      are the check for.
- [x] No figure is claimed. The work removed is a copy per property of every
      style object, which is a shape change rather than a measured delta, and
      `guidelines/PERFORMANCE.md` records that this machine cannot resolve a
      delta of that size.

## Comments

The copy and the expansion are both gone, which is more than the ticket asked
for. Expanding a shorthand writes the name as the value, and a name is not a
literal, so an expanded shorthand could never be a declaration of `null`. The
step could not change the answer for any shape a caller can hand the guard.

The list of names the guard reads against is lent rather than copied, so the
sweep above it no longer copies one list per namespace either.

A case covers the shape the copy used to hide: a property written as a bare
name stays, because it declares nothing.
