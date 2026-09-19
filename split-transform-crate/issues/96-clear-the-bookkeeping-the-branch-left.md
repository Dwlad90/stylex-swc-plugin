# 96 — Clear the bookkeeping the branch left

**What to fix:** two items the branch left behind.

**A dead harness helper carrying an `unsafe` process-global write.**
`crates/stylex-transform/tests/utils/transform.rs` held `_parse_js`, which has
no caller anywhere in the test tree. It survived on the module-level
`#![allow(dead_code)]` and carried
`unsafe { std::env::set_var("INSTA_UPDATE", "no") }`. Under `cargo nextest` each
binary is its own process, so the write was harmless -- but libtest still runs
one binary's tests on many threads, and `setenv` concurrent with any `getenv` is
undefined behaviour in Rust 2024.

It also broke this repository's own rule: `guidelines/stack/RUST.md` says an
item kept with no caller takes `#[allow(dead_code)]` and must **not** be marked
with a leading underscore, because the underscore hides it from the lint.

Answered by deleting the function and the eleven imports it alone used. The
`INSTA_UPDATE` default was not wanted -- nothing else reads it.

**A glossary entry whose implementation was deleted.**
`crates/stylex-types/CONTEXT.md` still taught "Serialized value" through
`serialize_value_to_json_string`, which this branch deleted along with
`serialization.rs`, `remove_quotes` and `JSON_REGEX`. No live caller of any of
them remains, and `4f09c9072` ("clear the bookkeeping drift the branch left")
missed the entry. Answered by deleting it.

**Blocked by:** None.

**Status:** resolved

- [x] No `unsafe` environment write in the test harness
- [x] No glossary entry naming a deleted function
