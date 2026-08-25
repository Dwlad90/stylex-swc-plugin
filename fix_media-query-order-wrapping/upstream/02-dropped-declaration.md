# Two media queries that canonicalize to the same text lose one declaration

**Version:** `@stylexjs/babel-plugin` 0.19.0, `@babel/core` 8.0.1, Node
v24.11.0.

## What happens

When two entries of one conditional value map canonicalize to the same query
text, the second silently replaces the first. One of the author's declarations
is absent from the output, with no warning and no error.

## Minimal reproduction

```js
import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  root: {
    color: {
      default: 'black',
      '@media (min-width: 200px)': 'red',
      '@media (min-height: 100px)': 'green',
      '@media (min-width: 300px)': 'blue',
      '@media (min-width: 100px)': 'purple',
    },
  },
});
```

## Observed output

Four rules where the author wrote five:

```css
.x1mqxbix { color: black }
@media not all { .x12vud9h.x12vud9h { color: blue } }
@media (max-width: 99.99px) and (min-height: 100px) { .xsllcrx.xsllcrx { color: green } }
@media (min-width: 100px) { .xr6za1w.xr6za1w { color: purple } }
```

`red` is gone. Both `(min-width: 200px)` and `(min-width: 300px)` canonicalize
to `not all` once the trailing `(min-width: 100px)` is negated out of them, and
the second assignment takes the first one's key — keeping its position, which is
why the surviving rule sits where `red`'s would have.

## Expected

Either both declarations survive, or the author is told one was dropped. Losing
a style silently is the part worth fixing: redundant CSS is ugly, a missing
style is lost work, and nothing in the build output points at it.

## Where it comes from

`dfsProcessQueries` holds the level in a plain object and rewrites each media
key with `delete result[currentKey]` followed by `result[newMediaKey] =
currentValue`. When `newMediaKey` is already present, the assignment overwrites
that entry rather than adding one, so the collision is invisible.

## Note

This is more likely to bite once the redundant-wrapper defect reported
separately is fixed, or less — it depends which way that one goes. The two are
reported apart so either can be resolved without waiting on the other.
