# 100 — Refuse a namespace the reader cannot name

**What to fix:** the namespace reader in
`stylex-transform/src/shared/utils/core/member_expression.rs` passed over every
property it could not name. A spread, a method, a getter, a setter, a shorthand
name and a key it read through `as_ident` alone each fell out of a chain of
`and_then` calls as `None`, which reads the same as a property declared absent.

The two answers are not the same. A property declared absent names nothing the
runtime needs, and leaving it out is correct. A property the reader cannot name
is a namespace that never reaches `vars_to_keep`, and `retain_object_props`
(`visit_mut_var_declarator.rs`) then deletes a namespace the runtime still
reads. That is a wrong output, which is worse than a stopped build.

**Why 99 left it.** Ticket [99](./99-decisions-this-remediation-recorded.md)
recorded this as not applicable, because the refusals "could never be reached,
never be covered, and would fail the gate on every run". The object the reader
holds comes from `evaluate_with_functions` inside the same function, so no
source can put a refused shape in front of it.

That was true of the reader **while it stayed inside `member_expression`**. It
is not a property of the finding. Ticket
[97](./97-refuse-a-declaration-the-reader-cannot-name.md) closed the same shape
in `folded_value.rs`, and its refusal is covered by tests that hand the reader
the shape directly. The same route was open here once the reader became
something a test can call.

**How it was answered.** The filter moved out into `declared_namespaces`, which
takes the properties and answers the names. It reads each property through
`named_key_value` in `stylex-ast`, which is the reader this repository already
uses where a producer promised named key-values --
`transform_stylex_define_vars_call/helpers.rs` reads its variable group the same
way. So the refusals are the ones that reader already gives, and the namespace
reader speaks the same sentence as its siblings instead of a sentence of its
own.

**One deliberate step past the finding, and it matters.** The finding asked for
`OBJECT_KEY_MUST_BE_IDENT` on a key that is not an identifier. That is not what
this does, and the difference is the point. `namespace_name_from_prop_key`
**names** a quoted key, a number key and a computed literal key, because each of
them names a property at run time. Only a key that names nothing at compile time
is refused, on `KEY_HAS_NO_NAME`. The set the reader accepts is a superset of
the one it accepted before, so nothing that compiled before is refused now.

Refusing a quoted key, as the finding asked, would have been a new crash rather
than a new diagnostic. The fold rebuilds the keys of the object it writes, but
not the keys of an object it carries through as a value. Three producers hand
back an object the fold did not rebuild, and all three write their keys with
`create_key_value_prop`, which spells a key that is no identifier as a
`PropName::Str`:

| producer         | where                                        |
| ---------------- | -------------------------------------------- |
| `stylex.env`     | `stylex-evaluator/src/evaluate/rebuild.rs`    |
| `stylex.types.*` | `stylex-structures/src/base_css_type.rs`      |
| a folded fn map  | `stylex-evaluator/src/evaluate/rebuild.rs`    |

So a quoted key reaches this reader from a source. `create_ident_key_value_prop`
says as much where it is declared: use it "wherever downstream code calls
`.as_ident()`". This reader no longer does.

**Proof.** The workspace llvm-cov run reads TOTAL 100.00% with no uncovered
regions. No source reaches a refusal: the fold refuses a spread and a property
that is no key-value pair before it writes an object, and each of the three
producers above writes key-value pairs only. That is why the cases for the
refusals hand the reader its object directly.

**Known, and not changed here.** `named_key_value` refuses a method, a getter, a
setter and a shorthand name on `ILLEGAL_PROP_VALUE`, which speaks about the
value where the defect is the shape of the property. That wording is
`stylex-ast`'s and is read by every caller of that function, so it is not this
ticket's to change.

**Blocked by:** None.

**Status:** resolved

- [x] Every shape the reader cannot name is refused
- [x] One case per refusal, each handing the reader the shape directly
- [x] Every key shape that names a property is named, not refused
- [x] A property declared absent is still left out
- [x] The refusal is the sibling readers' own, not a second sentence for one shape
- [x] The workspace coverage run stays at 100% with no uncovered regions
