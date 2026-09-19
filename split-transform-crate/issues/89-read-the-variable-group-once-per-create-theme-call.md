# 89 — Read the variable group once per createTheme call

**What to fix:** `validate_theme_variables` ran twice for one `createTheme`
call. `transform_stylex_create_theme_call.rs` asked it and threw the answer
away; `stylex_create_theme` then asked it again.

The read is not free. It runs `get_key_values_from_object`, which
(`stylex-ast/src/ast/convertors.rs:458-470`) copies every property of the theme
object and then copies each pair again -- two deep copies of the whole variable
group per call -- plus a copy of the theme reference on the `ThemeVars::Group`
path.

**How it was answered.** The first call keeps its answer and hands it on. The
producer is split in two: `stylex_create_theme` reads the group and delegates,
and `stylex_create_theme_from_group` takes the group already read. The transform
calls the second.

Splitting rather than changing the signature, because the first spelling has
twenty-five callers in the test suites and one in `stylex_create_theme_nested`,
and none of them holds a group to hand over.

The refusal ordering the first call exists for is unchanged: it still runs
before the producer, so a first argument that is no variable group is refused
with that sentence rather than with the producer's.

**Blocked by:** None.

**Status:** resolved

- [x] One read per call
- [x] The refusal ordering is unchanged
- [x] Both spellings of the producer keep their tests
