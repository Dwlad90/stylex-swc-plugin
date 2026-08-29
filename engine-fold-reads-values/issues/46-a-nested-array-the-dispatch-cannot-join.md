# 46 — A nested array the dispatch below the fold cannot join

**What to build:** A join over an array holding an array folds on the hand-back
path, as it does when the whole expression crosses the bridge.

Found while closing 31, and unrelated to it: an array element that is itself an
array refuses once the expression is handed back rather than folded.

```js
import { colors } from 'colors.stylex.js';
content: [[colors.primary, 'a'], 'b'].join('|')
// upstream: `.xhpny3o{content:var(--xa513j),a|b}`
// here: `Cannot fold 'join' at compile time.`
```

The theme member read hands the whole expression back — the fold declines a
property read on a carried reference — so `join` is answered by the dispatch
below, and that path has no reading for a nested array element. Nothing about
`Array` is involved: the same shape written as `Array([colors.primary, 'a'],
'b').join('|')` refuses identically, and the same array with no theme member
folds, because then the expression crosses the bridge and the engine joins it.

Measured on `@stylexjs/babel-plugin` 0.19.0. `.flat()` in the same position
diverges the same way — `.xxpjdwt{content:var(--xa513j)|a|b}` upstream.

An acceptance divergence rather than wrong output: this build refuses where the
other compiles, so no class name is invented.

**Blocked by:** none.

**Status:** resolved

- [~] A join over an array holding an array folds on the hand-back path —
      obsolete: it folds on the crossing path instead, and there is no longer a
      hand-back path for it to fold on
- [~] The same is measured for the other methods that read elements through the
      dispatch rather than through the engine — obsolete for the same reason: no
      method reads an element through the dispatch, so the set is empty. Every
      such read is measured through the engine instead
- [x] The shape is pinned in the module corpus, with the note saying which path
      answers it — `modules-46-a-nested-element-beside-a-theme-member` in
      `parity/corpus/modules.json`, sixteen reads over a nested element,
      measured `identical` against the reference compiler. The full
      twenty-four, and the two shapes both compilers refuse, stay in
      `transform_stylex_create_test/theme_members_in_a_fold.rs`, which the row
      names

**Resolution:** the divergence was closed by 44, and this ticket ends as the
corpus that proves it.

The reading of the bug was right about the fold declining and wrong about what
happened next. A theme member read did hand the whole expression back, and the
dispatch below did refuse `join` — but not for want of a reading of a nested
element. That arm refuses **every** method on an array receiver, without
condition, with `Cannot fold 'join' at compile time.`, because 06 gave the whole
of `Array.prototype` to the fold and left nothing below it. Nesting was never
the reason; being handed back at all was.

So the fix was to stop handing it back, and 44 did that for its own reasons: a
`defineVars` group now crosses the value bridge as a group stand-in rather than
as the string its `toString` answers, so nothing in `[[vars.primary, 'a'], 'b']`
declines and the engine reads the nesting the way the language does. Both of the
first two boxes describe work on a path that no longer exists, which is why they
are struck rather than ticked.

What was left to do was measure it, and the measurement is the deliverable.
Twenty-four shapes over a nested element — `join`, `flat` with no argument and
with `2` and with `Infinity`, `toString`, the `String` global, `concat`, an
index read, `map`, `filter`, `reverse`, `slice`, `sort`, `indexOf`, `length`,
the `Array` receiver spelling, an empty array and both nullish values as
elements, a plain object as one, sixty nested elements side by side and eight
levels of nesting — answer what `@stylexjs/babel-plugin` 0.19.0 answers, class
name and rule text alike. Two more agree by refusing: a hole inside a nested
element, and a nested read whose method answers a boolean.

The pinned note says which path answers the shape, because the next person to
read the ticket's own reasoning would otherwise look for the hand-back path this
resolution is about.
