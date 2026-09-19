# 85 — Read the env inside a rule call

**What to fix:** `stylex.env.<name>` folds inside `stylex.create` and refuses
inside the three rule calls -- `keyframes`, `positionTry` and
`viewTransitionClass`. The reference folds it in all four.

Measured with the probe against `@stylexjs/babel-plugin` 0.19.0, with
`env: { brandPrimary: '#123456' }`:

| Source                                                        | Here                                                             | Upstream                        |
| ------------------------------------------------------------- | ---------------------------------------------------------------- | ------------------------------- |
| `stylex.create({ a: { color: stylex.env.brandPrimary } })`      | `{ a: { kMwMTN: "x1tfn4g9", $$css: true } }`                     | the same                        |
| `stylex.keyframes({ from: { color: stylex.env.brandPrimary } })`| REFUSED: `Only static values are allowed inside of a keyframes() call.` | `"xvc6sa4-B"`            |
| `stylex.positionTry({ top: stylex.env.brandPrimary, ... })`     | REFUSED: `Only static values are allowed inside of a positionTry() call.` | `"--xl9eqvh"`           |

A build that writes one theme value in a `create` and the same value in a
`keyframes` stops on the second, and the author has no way to tell from the
sentence that the first one worked.

**Where it starts.** `build_rule_call_eval_config` in
`transform/stylex/visitor_utils.rs` folds `StateManager::apply_stylex_env` into
the map, which registers `env` as a *member* of the namespace. That is not what
makes `stylex.env.<name>` resolve; `register_env_in_namespace_fold` is, and it
is called only where a `create` call sets its evaluation up.

**What makes it hard.** `register_env_in_namespace_fold` names the reason it is
not called for the rule calls: registering the namespace there turned four
refusals into silent drops, because a bare `stylex` written where a static
value belongs then materializes into an object. So the answer is not to call it
in `build_rule_call_eval_config`; it is to make the two-level member read
resolve without registering the bare namespace, and to keep a bare `stylex`
refusing.

**How old it is.** Not introduced by the crate split. `register_env_in_namespace_fold`
carries the trade on `develop` as well.

**Blocked by:** None.

**Status:** ready-for-human

- [ ] `stylex.env.<name>` folds inside all three rule calls
- [ ] A bare `stylex` written where a static value belongs still refuses, in all
      four calls
- [ ] A parity corpus row covers one rule call reading the env

## Comments

Found while recording the preconditions the cached rule-call function map rests
on ([83](./83-build-the-keyframes-function-map-once.md)). The case that found it
was meant to measure that the second rule call of a module still reads the env;
it cannot, because the first one does not read it either. The cache precondition
is measured against the builder instead, by
`the_env_survives_into_every_map_the_module_asks_for`.
