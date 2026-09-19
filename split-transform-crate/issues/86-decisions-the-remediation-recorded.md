# 86 — Decisions the remediation recorded

**What this holds:** the items of the branch-wide review that were answered with
a decision rather than a change, each with the evidence behind it. Filed so a
later reader finds the reasoning where the review is, rather than in a commit
message or a plan file that does not travel with the repository.

**Blocked by:** None.

**Status:** resolved

## A finding that was false

**A namespace key the printer has to quote was never dropped.** The review read
`member_expression.rs` asking `key_value.key.as_ident()` and concluded that a
namespace spelled `'--my-color'` answered nothing and was swept away. It is not:
the evaluator files every key of every object it rebuilds through
`create_ident_key_value_prop`, which keeps a name no identifier would take
rather than quoting it, so `'--my-color'`, `0`, `1e21` and `['--x']` all reach
this reader as idents.

Measured by making the change, then putting the old reader back and running the
cases again: all twelve pass either way. Three cases were added from the
reader's side -- a quoted key, the three key shapes that are not written as a
name, and a quoted key that declares nothing -- so the invariant is now pinned
where it is read as well as where it is written. The reader itself is unchanged
and its comment names `create_ident_key_value_prop` as what makes the invariant
hold.

## Declaring the constant pair, which no ticket asked for

Item 20 asked for the addon output to be verified and the answer recorded. The
answer is recorded in two places, and one of them is the generated `.d.ts`: the
metadata type named `ltr` and `rtl` only, so a reader of `constKey` or
`constVal` had to reach past the declaration to find a pair every `defineConsts`
rule writes.

It is not a type this repository invented. The declaration is now byte-identical
to the reference's own `index.d.ts`:

```ts
{ ltr: string; rtl?: null | string; constKey?: string; constVal?: string | number }
```

So the change removes a divergence rather than adding one, and a consumer that
reads both compilers' rules through one type compiles against either. Both
runtimes are wider than that type -- `null` and a boolean cross as well -- which
is why `index.spec.ts` reads the field as `unknown` to assert what actually
crosses, and why the note beside the declaration says the type follows the
reference rather than the truth.

## A sentence that can be read two ways

`f3f41df84`'s message ends "null, 1, \"red\", true and 0 cross identically on
both carriers." It means that this compiler and the reference answer the same on
each of the two carriers. It can be read as saying the two carriers answer the
same as each other, which is false and is the opposite of what the commit is
about: a boolean is `true` in the metadata and `"true"` in the injected call,
and a null keeps its pair in one and loses it in the other.

The message is left as it is. It is the second commit of the branch, and every
later commit and this file cite SHAs that a rewrite would invalidate, which
trades one ambiguous sentence for a dozen dead references. The precise statement
lives at the reader instead, in the note above `metadata.rs`'s constant
reader, and both halves are pinned by cases.

## A second finding that was false

**An unread fallback slot was never the risk.** The review of the remediation
read `ArgTexts::take_text` as resting on "a chain names only positions the plan
asked about", and a test was written to hold that. It holds, and it is not the
property the code needs: `filled` reads a slot on demand, so a position the plan
never asked about simply costs one read. The only way the move can answer an
empty text is a position read *twice*.

The test was removed and the code answers the question instead. `take_text`
empties the slot rather than leaving the empty text the move makes, so a second
read reads the argument again. That also keeps the type's own account of its
slots true: a filled slot means one that was read.

## The `coverage(off)` sites

Fourteen were audited. The review's premise that none of them is asserted
anywhere does not hold for two of the three it named: `reason_under_key`'s live
arm is asserted by `a > flexGrow > unknown error` in `logical_operators.rs`, and
`number_from_json`'s by the round trips in
`flat_compiled_styles_value_test.rs`. The defect was the justification wording --
three said the step "computes nothing" where it does something -- and all three
now say what is excluded and what is measured.

Dropping the attributes was considered and refused. A two-arm choice cannot be
split across the exclusion boundary, so the only way to measure the live arm is
to make the dead one reachable. For `reason_under_key` that means taking a
`String` instead of an `Option<String>`, which turns a deopt -- a value left for
the runtime -- into a stopped build. `RUST.md` ranks "make the shape
unrepresentable" above the attribute, but not at that price.

### The audit, site by site

Each site was read for two things: whether the step really computes nothing,
and whether its note names something a reader can check rather than an argument
from the call site.

| Site | Computes nothing? | Note names a check? |
| --- | --- | --- |
| `validators.rs:190` `or_refuse_missing_argument` | yes | the caller's count check |
| `validators.rs:230` `or_refuse_initializer` | yes | the caller's predicate |
| `validators.rs:1087` `or_refuse_theme` | yes | the look-up that found the declarator |
| `validators.rs:1139` `or_refuse_nameless_group` | yes | the caller's own read |
| `evaluate_stylex_create_arg.rs:68` `or_refuse_nameless_key` | yes | a folded key is a string literal |
| `evaluate_stylex_create_arg.rs:84` `or_refuse_unfolded_body` | yes | the recursion's own answer |
| `evaluate_stylex_create_arg.rs:106` `or_refuse_unfolded_key` | yes | a folded key is a string literal |
| `evaluate_stylex_create_arg.rs:140` `reason_under_key` | **no** | corrected |
| `flatten_raw_style_object.rs:95` `or_refuse_handled_template` | yes | the handler is given one |
| `flatten_raw_style_object.rs:110` `report_unmatched_key` | **no** | corrected |
| `stylex_nested_utils.rs:104` `object_under` | yes | the caller makes it an object |
| `transform_define_marker_call.rs:39` `or_refuse_unbound_marker` | yes | two named tests |
| `transform_stylex_atoms.rs:55` `or_refuse_missing_atom_namespace` | yes | one named test |
| `define_vars_call/helpers.rs:187` `arrow_body_expr` | yes | one named test |
| `flat_compiled_styles_value.rs:147` `number_from_json` | **no** | corrected |

Eleven of the fourteen were accurate. Three said the step "computes nothing"
where it does something, and each now says what is excluded and what is
measured. Three of the eleven go further than the rest and cite a test by name
rather than a precondition; those are the pattern the others should move toward
when one of them is next touched.

`guidelines/stack/RUST.md`'s option 2 was argued nowhere for the eleven
`or_refuse_*` helpers, and the answer is the same for all of them: each is the
`match read { Some(v) => v, None => <panic> }` shape, and the `Option` comes from
a library reader the caller does not own. Making the shape unrepresentable means
writing a reader that cannot answer `None`, which is the same guard moved one
call further down.

## The performance items not taken

- **A Merkle pass over the injection walk.** ADR 0002 measures that walk at 1.7%
  of the compile that pays for it. A change to it needs its own measurement, and
  this machine cannot resolve a delta of that size: two runs of one unchanged
  build against one saved criterion baseline read -2.7% and +3.5% on the same
  case.
- **Writing an inline style's own names straight into the caller's map.** It
  changes the answer. `compiled_styles` is shared across the elements of one
  argument list, so an inherited name that today overwrites an earlier
  argument's would stop doing so. The saving is one map per inline style object.
- **Short-circuiting `to_json_text`'s scalar arms.** `serde_json::to_string` is
  fallible, so the string arm gains an error arm no constant can reach, under a
  gate that permits no uncovered region. A substitution in its place is the one
  `RUST.md` refuses, because the value reaches a folded answer. The saving is one
  `String` clone per constant.
- **Rebuilding `attrs`' map.** Three `to_string()` on literals that an
  `IndexMap<String, _>` requires, and a vector that
  `inline_style_to_css_string` takes. Removing the vector means a streaming
  variant of that function, which is more code than the saving is worth.

## The two remaining smells, and why they stay

- **`crates/stylex-state/src/tests/*` are registered from `lib.rs`**, which
  `TESTING.md` names as an anti-pattern. Crate-wide debt older than this branch,
  continued by the two files it adds rather than introduced by them. Moving all
  twenty-three files is its own change, and doing it here would bury the work
  this branch is for.
- **`js_to_ast.rs`, the three delegating validators, the `(api_name, call_expr,
  state)` clump, `has_stylex_api_import_for_the_consuming_cycle`, `find_cycle`'s
  pre-joined `String`, and `functions.rs`'s `slot`.** Each is a fair reading.
  None of them is reachable from a source in a way that changes what the
  compiler emits, and every one sits in a file this branch did not otherwise
  touch. They are worth a cleanup ticket of their own, not a rider on a coverage
  branch.
