# 43 — Read a nested `env` value as the top level does

**What to build:** `stylex.env` accepts an object, and two readers in
`crates/stylex-rs-compiler/src/utils/fn_parser.rs` read it. `parse_env_value`
reads a value at the top. `napi_value_to_expr` reads the values inside an
object or an array below it.

The two disagreed. The top reader asks `env_value_kind` what a value is, and it
makes a null expression for each kind that carries no expression of its own.
The reader below it matched the raw type and called `panic!` for everything
else.

**Why it is worse than a panic.** That `panic!` ran in the option parser, which
runs **before** the compiler installs its panic guard. It therefore ended the
Node process with SIGABRT and gave JavaScript no error to catch. One nested
value was enough to end a build. A null, an undefined value, a symbol, a bigint
and a function all compile at the top of `env`, and all five killed the process
one level down.

The fix landed in `280cf3a68` without a ticket. This file records it.

**Blocked by:** None.

**Status:** resolved

- [x] `napi_value_to_expr` asks `env_value_kind`, so one rule answers for both
      levels of `env`
- [x] No path out of the option parser calls `panic!`.
      `guidelines/stack/RUST.md` rules out `.unwrap()` and `.expect()` for the
      same reason: a crash there cannot be caught
- [x] Each of the five shapes is covered nested one level down, and nested
      inside an array
- [x] The tests run each shape in a **child process**. A test cannot report an
      abort of the process it runs in, so an in-process test would have taken
      the whole suite down rather than failing
- [x] The addon is rebuilt before the JavaScript suite runs. The suite
      exercises `dist/*.node` and not the Rust sources, so a green Rust run is
      not evidence on its own
- [x] The workspace gate is green in **debug** -- never `--release`

## Comments

**Why the top level was already right.** `env_value_kind` exists because the top
reader had to answer this question first. The second reader was written later
and repeated the decision in its own words rather than calling the function that
holds it. The two then diverged, which is the ordinary end of a rule that is
written twice. The fix is not new behaviour; it is one reader deleting its copy
of a rule and asking for the original.

**The blast radius was a whole build.** This is not a wrong output but a dead
process, and the caller could not tell the two apart: no stack, no message, no
catchable error. That is why it is filed as its own ticket rather than folded
into a batch of small findings.
