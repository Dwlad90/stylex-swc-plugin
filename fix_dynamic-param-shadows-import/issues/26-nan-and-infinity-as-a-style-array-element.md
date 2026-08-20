# 26 — `NaN` and `Infinity` as a style array element

Status: `needs-triage`
Blocked by: None

**What was found:** A fallback array holding `NaN` or `Infinity` is refused
here and compiled upstream, in both style-value positions.

```js
export const styles = stylex.create({ s: { height: [NaN, '2px'] } });
```

| input | Babel 0.19.0 | this compiler |
| --- | --- | --- |
| `height: [NaN, '2px']` | `.x…{height:2px}` — the element is dropped | `A style array value can only contain strings or numbers.` |
| `height: [Infinity, '2px']` | `.x…{height:Infinitypx;height:2px}` | the same refusal |

The two upstream answers differ from each other, which is the interesting half:
`NaN` is dropped as an absent value and `Infinity` is spelled into the
declaration with the property's unit appended. Neither is a number to the array
check here, because both reach it as identifiers rather than as numeric
literals — the same representation fact ticket 05 settled for the value
position, one level further in.

`height: Infinitypx` is not a declaration any browser accepts, so upstream's
answer for the second row is not obviously the one to adopt. A verdict is
wanted before the check moves.

Measured against `@stylexjs/babel-plugin` 0.19.0 under the parity harness's
configuration, in a static namespace and inside a dynamic style's body — both
positions read the same, so this is about the array check and not about the
position. Found while measuring ticket 14, whose fold is what made the dynamic
half reachable.

- [ ] A verdict per row, `NaN` and `Infinity` separately
- [ ] Either the fold, or a recorded reason to keep refusing
- [ ] `modules-1266-an-array-with-a-nan-element-in-a-dynamic-style` and a static
      counterpart carry the decided verdict
