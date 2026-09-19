# 65 — Cover the transform's walk

**What to build:** The tests that exercise the 123 regions in the `VisitMut`
walk — `crates/stylex-transform/src/transform/visit_mut/**`,
`transform/visit_mut.rs` and `transform/mod.rs` — that no test in the workspace
reaches. This is the cycle driver and the pre-scan: what the transform does with
a module before and around any single `stylex.*` call.

**Where the gap is:**

| File                                         | Uncovered regions | Regions | Lines  |
| -------------------------------------------- | ----------------- | ------- | ------ |
| `visit_mut/visit_mut_module.rs`              | 37                | 89.74%  | 91.65% |
| `transform/mod.rs`                           | 29                | 87.92%  | 88.93% |
| `visit_mut/visit_mut_var_declarator.rs`      | 29                | 85.78%  | 87.44% |
| `visit_mut/visit_mut_jsx_opening_element.rs` | 14                | 93.24%  | 93.72% |
| `visit_mut/visit_mut_import_decl.rs`         | 10                | 86.84%  | 85.42% |
| `transform/visit_mut.rs`                     | 3                 | 94.44%  | 94.74% |
| `visit_mut/visit_mut_module_items.rs`        | 1                 | 97.87%  | 98.08% |

`visit_mut_module.rs` also holds two per-instantiation gaps, at the functions on
lines 542 and 619 — each is one region the best instantiation misses, and the
one at 619 is a `Visit` implementation the compiler generates for more than one
node type.

**The three big files are three different subjects:**

- `visit_mut_module.rs` is the [pre-scan](../../../crates/stylex-transform/CONTEXT.md)
  and the cycle order. Its uncovered arms cluster in the `for_sx()` mode, which
  runs only when `sxPropName` is set, and in the `writes_only()` mode, which
  runs only for a module that reaches evaluation. A case must set the option to
  reach the first at all.
- `transform/mod.rs` is the transform's own construction and options: the
  combinations of `dev`, runtime injection, treeshake compensation and module
  resolution that no fixture builds.
- `visit_mut_var_declarator.rs` is what the walk does with a declarator whose
  initialiser is a producer call — including the shapes it declines to rewrite.

**The `sx` prop path is the thread through all three**, and it is the path the
existing fixtures use least. Reading the pre-scan entry in the crate's
`CONTEXT.md` before starting will save re-deriving which mode records what.

**Blocked by:** [62](./62-record-the-transform-coverage-baseline.md) — the
baseline and the repeatable suite.

**Status:** in-review

- [x] `scripts/coverage-missing.sh -p stylex_transform` reports no unexercised
      region under `src/transform/visit_mut/`, in `src/transform/visit_mut.rs`
      or in `src/transform/mod.rs`. Every one of the ten files reads 100.00% of
      regions, functions and lines.
- [x] The two per-instantiation groups in `visit_mut_module.rs` are closed. Both
      are gone from the gate list; the 13 that remain are in
      `shared/transformers/`, which is [66](./66-cover-the-transformers-and-structures.md).
- [x] Tests cover regular and irregular inputs, and the edge cases each function
      states, rather than only the path that makes the number go up. The list is
      in the comment below.
- [x] A case that needs an option set says which option and why. Three do: the
      `sx` prop name for the attribute shapes, `import_sources` for the alias a
      `require` is read under, and `inject_stylex_side_effects` for the scan
      that reads every item of the module.
- [x] The full workspace suite stays green. `cargo test --workspace
  --all-features`, `cargo clippy --workspace --all-features --all-targets`
      and `cargo fmt --all` are clean.

## Comments

### What the walk reads, measured

`scripts/coverage-missing.sh -p stylex_transform` on this branch, 3217 tests:

| Figure                            | Baseline (62) | Now    |
| --------------------------------- | ------------- | ------ |
| Regions                           | 89.03%        | 97.46% |
| Functions                         | 92.78%        | 98.41% |
| Lines                             | 90.09%        | 97.51% |
| Regions unexercised by every test | 741           | 166    |
| Of those, in this ticket's group  | 123           | 0      |
| Regions the gate counts           | 31            | 13     |

The 166 that remain are `shared/transformers/**`, `shared/structures`,
`shared/utils/**` and `transform/stylex/**` -- tickets
[63](./63-cover-the-transform-shared-utils.md),
[64](./64-cover-the-transform-call-handlers.md),
[66](./66-cover-the-transformers-and-structures.md) and
[71](./71-close-the-guards-over-a-compiler-built-object.md).

### Three things were code, not tests

**A destructured member target was read bare.** `[o.x] = xs` was recorded as a
mutation of `o` and `[(o.x)] = xs` was not, because the pattern arm matched a
bare member where every other write shape in the file looks through the
parentheses and the TypeScript wrappers. The write went unrecorded, so a
declaration the module had already changed could still be folded at a use site.
The one walk that serves the other shapes serves this one now.

**The module copy read a condition the measured profile cannot enter.**
`cfg!(debug_assertions) || !use_real_file_for_source` is always true in a debug
build, which is the profile coverage measures, so the branch had a side no test
could take. Two arms under `#[cfg]` say the same thing.

**Two readers had no caller.** The optional-config import filler duplicated the
one beside it with an arm nothing passes, and a TypeScript strip factory had
never been called at all. Both are gone. `matching_style_var` now answers the
name and the initializer it matched on, rather than the declarator, because the
sweep asked those two questions again where a missing part could only be
skipped.

### The cases, by what they answer

- **The pre-scan** -- the five TypeScript wrappers around a write target and
  around a deleted member, bare and parenthesised; a write into a call result,
  reached through an optional chain and as a `delete` operand; a private method
  call; a `super` property; every binding form a loop head can carry; an
  invalid node, which the parser writes for `[o?.x] = []`; and a callee that is
  no expression, which `super(…)` and a dynamic `import(…)` are.
- **The construction** -- the constructor the compiler uses and the order it
  seeds the import sources in, the `Module` entry, a member call on another
  object, and the two builder settings no fixture asks for.
- **The `sx` runtime binding** -- a call that carries no position, which a build
  step generates, and a transform configured with no import source at all.
- **The `sx` prop** -- an element named through a member or a namespace, an
  attribute whose value is text or absent, a compiled call whose element name is
  no host element or that has no props object, the three Solid.js shapes that
  name no prop, and a dynamic import beside a prop that does compile.
- **The imports** -- the atoms source as a namespace import and as a named one,
  plain, renamed and spelled as a string; a name from the StyleX source that is
  no API; the `require` calls that name no module and the binding forms that
  name no local, for both sources; and a side-effect import scan over a module
  that holds statements.
- **The finalize sweep** -- namespaces kept by name and by the whole-namespace
  record, a namespace nothing reads, a namespace that holds no object, the null
  declarations kept and dropped, and the two props the producer never writes.

### Two guards are over an object this compiler built

`retain_object_props` and `retain_style_props` walk the object
`convert_object_to_ast` wrote a few phases earlier, which writes key-value props
under names it chose. So a spread, a method and a computed key are shapes no
module can put in front of them -- the same reading
[71](./71-close-the-guards-over-a-compiler-built-object.md) records for the
calls under `transform/stylex/`.

`guidelines/stack/RUST.md` ranks the answers, and the first one fits: the guards
are reachable from a caller, so the cases hand them the objects they answer for
rather than excluding the step from the gate. They sit in the crate's own test
binary, which is also what closes them for the gate -- the compiled suite
reaches the function in a different instantiation, and llvm-cov scores a
function on the best-covered one.

### The review round

Three passes ran over the commits above -- standards, spec and performance.
What they found, and what came of it.

**Two suites were registered from the wrong file.** `TESTING.md` decides a
suite's directory by which file registers it, so that one source module has one
suite and a reader has one place to look. The `sx` runtime binding is read in
the file that resolves it now, and the `Module` entry in the file that declares
it. The transform builder every case shares moved into the prelude beside the
parse, which is what the three suites had been repeating.

**The `sx` attribute search copied a value it goes on to leave alone.** The
rewrite took the attribute value by copy, so `sx="red"` paid for a deep copy of
the subtree under it that the shape before the review did not. It borrows now,
and copies the expression alone, which is the copy the original made.

**The invariant the sweep reads is asserted where the object is built.**
`guidelines/stack/RUST.md` asks that a deleted guard name the producer that
holds the invariant instead, and that the producer assert it.
`writes_every_prop_as_a_key_value_under_a_name` is that assertion, and both
sweeps point at it.

**Two readings were left as they are, and here is why.**

The `#[cfg]` split of the module copy leaves the release arm compiled in no
profile the coverage gate measures. That is true, and the alternative is a
branch the measured profile cannot enter, which is the thing the gate is for.
The arm is one `if` around a call the debug arm makes as well, and every
release build type-checks it.

The sweep's guards are answered by cases that build the objects rather than
compile them. `RUST.md` ranks proving the claim against a source first, and no
module reaches these guards -- which is the same reading ticket
[71](./71-close-the-guards-over-a-compiler-built-object.md) records for the
readers under `transform/stylex/`. The cases here say what the sweep answers at
its own boundary, for the shapes a caller can hand it, and the producer test
above says no caller does. The answer that removes the question rather than
answering it is 71's reshape, and these two belong with it.

### Every finding, and what came of it

Four review passes ran in all: standards, spec and performance over the batch,
then one more over the parse the suites share. The table is the whole list, so
a reader can see which findings were acted on and which were read and left.

| Finding                                                        | Verdict |
| -------------------------------------------------------------- | ------- |
| Two suites registered from a file they do not cover            | Fixed   |
| The builder every case repeats                                 | Fixed   |
| The pre-scan suite's own copy of the parse                     | Fixed   |
| The `sx` attribute search copies a value it leaves alone       | Fixed   |
| The sweep's invariant is not asserted at the producer          | Fixed   |
| `.expect` in a case, where the repo uses `match`               | Fixed   |
| Prose left stale by the shared parse                           | Fixed   |
| A parser shape claimed in prose rather than read               | Fixed   |
| The `#[cfg]` split of the module copy serves the coverage tool | Kept    |
| `comments()` is a middle man                                   | Kept    |
| `matching_style_var` answers a pair rather than a type         | Kept    |
| The sweep reads each prop a second time                        | Kept    |
| `to_id()` moved onto a colder path                             | Kept    |
| `transform/mod.rs` reaches 100% partly by deletion             | Kept    |
| The change reaches outside the ticket's group                  | Kept    |
| `resolved_ts_module` has one consumer                          | Kept    |

**Why the kept ones are kept.**

The `#[cfg]` split leaves the release arm compiled in no profile the gate
measures. The alternative is a branch the measured profile cannot enter, which
is what the gate exists to refuse. `RUST.md` forbids reshaping a _signature_ to
move a region; this moves no signature, and every release build type-checks the
arm.

`comments()` has two callers in two files and names what it answers, so it is
no longer the one-line wrapper the pass called it.

`matching_style_var` answers two borrowed parts of one declarator, both named in
its doc and both destructured at the one call site. A type for that is a name
for a pair nothing else holds.

The sweep reads each prop again in its second pass because the first pass
borrows the object to read the names, and the second needs the value through a
mutable borrow. It is two enum matches and no allocation. Removing them means
carrying validated props across the borrow, which no safe shape expresses.

`to_id()` runs for every declarator that matches a recorded style variable,
including those that then bail. It is a refcount bump and a context copy, and it
has to be read before the mutable borrow the sweep needs.

`transform/mod.rs` reaches 100% partly by deletion, which is the honest answer
for a private reader with no caller: `RUST.md` asks for the dead branch to go,
not for a test that keeps it alive.

The change reaches `stylex-state` and `stylex-constants` because the sweep's two
extra questions were asked of a state API, and the message the deleted panic
carried had no other reader. Both are the caller cleanup, and the state's own
suite covers the refusal that moved there.

`resolved_ts_module` has one consumer, which is the group of cases an
angle-bracket cast belongs to. It sits in the prelude because that is where the
parse lives, and both parses share one reader; splitting them would put half a
parse in each place.

### The performance review of the walk, second round

One more performance pass ran over this ticket's commits, after the fixes
above. It confirmed the four readings the ticket claims and found one cost.

**The four claims hold.** The `#[cfg]` module copy is the same test the release
build folded `cfg!(debug_assertions) || !use_real_file_for_source` down to, so
release pays what it paid before and the debug copy is compiled out. The `sx`
attribute search yields a borrow and allocates nothing; the one `Expr` copy it
makes is the copy the shape before the review made. `matching_style_var`
answers two borrows, not two copies. `member_target` is a borrow-returning
match that recurses only through the wrappers, and only the assignment and
loop-head arms reach it, so no cost lands on a node that is neither.

**The null sweep ran past its own answer.** A namespace kept whole makes every
name gathered before it unread, and no entry later in the recorded list can
change that answer. The scan read the list to its end and copied a name into a
vector it then dropped. It stops at that answer now.

**Two costs are older than this ticket and are filed rather than fixed.**
[75](./75-index-the-namespace-sweep-instead-of-scanning-it.md) is the sweep's
two linear scans, which multiply where a module holds many style variables.
[76](./76-read-a-declaration-before-copying-it-to-expand-it.md) is the retain
guard that copies every property of every style object to expand a shorthand,
only to read one boolean off the copy. Neither is a line this ticket wrote, and
[the repository's rule](../../../guidelines/PERFORMANCE.md) is that a finding
is fixed where it is found, not where it is met.

### The standards and spec review

Two more passes ran over this ticket. They read every rewritten path as
equivalent to the one it replaced, and found no defect in the code.

**Three comments were written so short that a noun went missing.** One ended on
the verb it began with, one left out what a prop is not, and one said an object
is reached for. Each states its subject now. The sweep's parameter is also
named for the many namespaces it holds, which is what its caller already calls
it.

**Two steps did nothing.** A test deduplicated a list whose only reader asks
whether any entry matches, and an import named the module it is a child of
through two steps. Both are gone.

**This record said 75 and 76 were filed rather than fixed.** Both are resolved,
and both fixes are in this ticket's files. The sentence above is kept as the
reading at the time, and this is the correction.
