# 13 — Three media queries the reference implementation refuses and we accept

**What to build:** Agreement on which media queries are valid, or a recorded
decision not to have it. Ticket 09's sweep closed the unbalanced-parenthesis
family and left three inputs where this compiler's rule grammar is more
permissive than `@stylexjs/babel-plugin` 0.19.0's:

| Input                                                         | We emit                              |
| ------------------------------------------------------------- | ------------------------------------ |
| `@media (min-width: 100px) and (max-width: 200px) or (color)` | a comma-joined `or` of the two sides |
| `@media ((min-width: 100px))`                                 | `@media (min-width: 100px)`          |
| `@media (((( min-width: 100px ))))`                           | `@media (min-width: 100px)`          |

Our rule parser accepts a parenthesized single condition and an unparenthesized
mix of `and` and `or`; the reference's `oneOf` chain accepts neither, and
refuses the whole declaration. So an author writing any of these gets a
stylesheet from us and a build failure from the official compiler — which is a
divergence in the safe direction, but a divergence.

These predate the media-query-order wrapping work and are not consequences of
it. They are separated because closing them means changing the media query
grammar, whose regression surface is every query the repository already accepts,
and because the acceptance question is worth deciding on its own terms: refusing
input that is valid CSS in order to match a stricter reference is a choice, not
an obvious correction.

One thing not to reach for while doing it: the reference implementation's parser
backtracks exponentially in parenthesis nesting depth — 8 levels take 1.18 s, 12
take 20.5 s, 16 do not finish in 30 s, where this compiler takes 2 ms at 200
levels. Matching its answers must not mean matching that.

**Blocked by:** 09.

**Status:** done — see `../evidence/grammar.md`

- [x] Each of the three is decided: matched, or recorded as a deliberate
      difference with the reason. **Twelve, not three** — widening the sweep
      from the three inputs to forty-seven combinator shapes found nine more,
      including two that both compilers accepted and rendered differently,
      which is a silent class-name divergence the three-input list could not
      have found. Twelve matched; two are kept as deliberate differences —
      nested parentheses around one condition, and one bare `not` after a
      media type's `and`, both of which the language defines and the reference
      implementation has no grammar alternative for.
- [x] Any grammar change is measured against the whole existing corpus of
      accepted queries, not only against the three inputs. Fifty combinator
      shapes, the full Rust suite, the JS suites and 1103 parity subjects. The
      sweep runs over features the range merge cannot fold, so a shape's text
      keeps the shape the parse gave it — running it over width bounds instead
      gives the same verdict, which is what says the choice is not doing the
      work.
- [x] Nesting depth stays linear — evidence: `../evidence/nesting-depth.cjs`,
      one depth per process, recorded in `../evidence/grammar.md`. Measuring it
      also found that five thousand levels aborted the process, which is now
      bounded at sixty-four rather than left to the stack.
- [x] The parity harness carries whichever answer is chosen, so it moves if the
      reference implementation does. Two module subjects: the comma binding and
      the parenthesized disjunction.
