# 01 — Move the binary-expression evaluation to the node named for it

**What to build:** Nothing an author can observe. This is the prefactor that
makes the following tickets easy.

The number path and the string path for binary expressions currently live in
the AST-convertor module, even though their only non-test caller is the
binary-expression evaluator node, and even though upstream keeps the whole
`BinaryExpression` switch inside its evaluator. Move them to that node.

Two things fall out of the move, and both are the point of doing it first:

- The evaluator's bridging coercion helpers are module-local to the evaluator.
  Once the two paths sit inside it, tickets 03, 04 and 05 can reach those
  helpers without widening their visibility.
- The convertor module stops being a place where a second string conversion has
  to exist.

The evaluator's node modules are private and have no inline tests today — that
is deliberate, and every node is tested from above. The pre-existing unit tests
on these two functions therefore cannot stay where they are once the functions
move: they cover operators that real StyleX source cannot reach (comparison,
bitwise, `in`), so they ride along into an inline test module beside the moved
functions rather than being deleted. This is coverage preservation only. Do not
add new cases here; the new behaviour in later tickets is pinned from above.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The number path and the string path for binary expressions live in the
      binary-expression evaluator node
- [~] The AST-convertor module no longer exports them. One caller outside the
      evaluator remains — see the first comment below
- [x] The bridging coercion helpers keep their existing module-local
      visibility — nothing is widened to make this compile
- [x] The pre-existing arithmetic, comparison, bitwise and `in` unit tests are
      preserved beside the moved functions, unchanged in what they assert
- [x] The one pre-existing logical unit test is left alone here; ticket 03
      deletes it along with the behaviour it asserts
- [x] No behaviour change: the full suite passes with zero fixture or snapshot
      edits. A snapshot that moves means this stopped being a pure move
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as a single `refactor(stylex-transform):` commit

## Comments

Landed as `refactor(stylex-transform)`. Notes for the tickets that follow:

- `binary_expr_to_num` could not be made fully private to the evaluator. The
  convertor module's own `expr_to_num` and `ident_to_number` each have an
  `Expr::Bin` arm that recurses into it, so it stays reachable through a
  `pub(crate)` re-export from `evaluate/mod.rs` — the same shape the module
  already uses for `evaluate_cached` and `deopt`. `binary_expr_to_string` and
  the transplanted `evaluate_left_and_right_expression` are now private to the
  node.
- Those two `Expr::Bin` arms were measured, not assumed: instrumenting them and
  running the whole Rust suite shows each is entered exactly once, both times
  from a convertor unit test that hand-builds the node. No fixture, and so no
  real StyleX source, reaches either. Worth knowing when ticket 04 rewrites the
  addition arm.
- Behaviour is unchanged, and so is parity with the reference implementation:
  the test-name set before and after the move is byte-identical, and no fixture
  or snapshot moved.

### Review follow-ups

Three commits after the move, from the two-axis review. The spec axis found
nothing; these are all the standards axis.

- **The test factories are gone, not consolidated.** The move copied
  `make_num_expr` / `make_str_expr` / `make_ident_expr` into a second test
  module. Rather than share the copies, all eleven across four modules were
  deleted in favour of the `create_number_expr` / `create_string_expr` /
  `create_ident_expr` the crate already ships. Equivalence was measured with a
  throwaway assertion over span, `ctxt`, `optional` and `raw` before the swap.
- **The comparison arms name their coercion.** Ten arms each spelled out
  `if condition { 1.0 } else { 0.0 }`. Behaviour is untouched, `in` and
  `instanceof` included — their arms now carry a note that what they answer is
  not what their operators mean.
- **The right-hand diagnostics named the left operand.** Both `stylex_panic!`
  calls in the addition helper's right-hand block printed `left_expr`. Fixed by
  extracting the shape both sides duplicated, so each side names itself.

Two review points deliberately left alone:

- The `ast::convertors` → `js::evaluate` import direction. The reviewer's own
  reading is that it cures Feature Envy; it is recorded in the move's commit
  body rather than undone.
- `evaluate_left_and_right_expression` reads *both* sides' confidence off the
  outer state, so its right-hand branch is unreachable. That is the copy-paste
  slip the spec already documents, and correcting it is a behaviour change that
  belongs to ticket 03/04, which delete the helper outright.
