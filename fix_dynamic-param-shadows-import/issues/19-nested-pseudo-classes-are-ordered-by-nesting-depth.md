# 19 — Three nested pseudo-classes hash a different class name

Status: `needs-triage`
Blocked by: None

**What was found:** A style value nesting three pseudo-classes in an order that
is not already alphabetical compiles to a different class name than the
reference implementation produces for the same input. A class name is a hash of
the selector plus the declaration, so this is a compatibility divergence, not a
cosmetic one.

```js
export const styles = stylex.create({
  w: { zIndex: { default: '1', ':hover': { default: '1', ':focus': { default: '1', ':active': '1' } } } },
});
```

| | leaf selector | class |
| --- | --- | --- |
| `@stylexjs/babel-plugin` 0.19.0 | `:active:focus:hover` | `.x12rlomf` |
| rs-compiler HEAD | `:focus:hover:active` | `.x1dv1xrr` |

Both emit the same declaration (`z-index:1`) and the same number of rules, so
the verdict is `divergent`: same properties, different class names.

Measured on both compilers as
`modules-nested-pseudo-classes-are-ordered-by-nesting-depth`, which records
`divergent` so the divergence reports as a changed verdict the day it moves.

## The mechanism, as far as it is measured

Upstream sorts the whole accumulated pseudo-class list; this compiler appears to
sort each pair as it nests and append the next one. That is what the four probes
say, and it is a description of the outputs, not of the code — the producing
call site is not pinned yet:

| nesting order | this compiler | upstream | verdict |
| --- | --- | --- | --- |
| `:hover` > `:focus` | `:focus:hover` | `:focus:hover` | identical |
| `:focus` > `:hover` | `:focus:hover` | `:focus:hover` | identical |
| `:hover` > `:active` | `:active:hover` | `:active:hover` | identical |
| `:active` > `:hover` | `:active:hover` | `:active:hover` | identical |
| `:hover` > `:focus` > `:active` | `:focus:hover:active` | `:active:focus:hover` | **divergent** |
| `:active` > `:focus` > `:hover` | `:active:focus:hover` | `:active:focus:hover` | identical |

Two pseudo-classes agree in either nesting order, which is why nothing before
this saw it: a pair sorted from either direction lands in the same place. Three
agree only where the nesting order is already alphabetical. At-rules are not
affected — they nest in the same order on both sides at every depth probed,
including eight levels.

## How it was found

Written as a depth guard for the identifier chain
(`modules-1266-a-shadowed-param-at-extreme-condition-depth`, ticket 09): a
shadowed dynamic parameter read at eight levels of condition nesting. The
resolution half agreed — identical declarations, one inline variable per leaf —
and the selectors did not. That guard now nests its pseudo-classes
alphabetically so it measures resolution, and this ticket owns the ordering.

Nothing to do with reference resolution, and nothing this effort's commits
introduced: the ordering is the same before and after the chain reorder.

- [ ] The producing call site is named — where the pseudo-class list is
      assembled and where it is sorted
- [ ] The order matches the reference implementation for three or more nested
      pseudo-classes, in any nesting order
- [ ] The corpus row's `expected` becomes `identical`, which is how the fix
      reports
- [ ] Snapshots that carry a three-deep pseudo-class nesting are re-recorded,
      and any that do not exist are written — the divergence survived because no
      test nested three
