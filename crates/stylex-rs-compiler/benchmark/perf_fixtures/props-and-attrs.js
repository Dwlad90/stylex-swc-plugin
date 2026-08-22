/**
 * The consuming side: `stylex.props` and `stylex.attrs` at a JSX call site.
 *
 * This is the only shape that emits `data-style-src`, the debug data prop —
 * defining styles is not enough, because the annotation is attached where the
 * styles are *read*. It is also where conditional and array style arguments are
 * merged, which is the work `enableInlinedConditionalMerge` decides how to do.
 *
 * It is also where the source-map entries point, for a reason worth writing
 * down: this compiler emits a source map in its *production* shape already, so
 * `sourceMap: True` measures nothing. What a fixture can still change is
 * inlining the map into the module, dropping the embedded source text, dropping
 * the column mappings, and chaining onto a map an earlier tool produced — which
 * is the slowest of them, and the last one is what `.input.map.json` beside this
 * file exists for.
 *
 * Nine call sites cover what the merge branches on: one style, several, a
 * conditional, an array, a nested array, a runtime condition the fold cannot
 * resolve, a dynamic style called with an argument, and `attrs` beside `props`.
 */

import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  base: { color: 'black', padding: 4 },
  emphasis: { fontWeight: 700 },
  danger: { color: 'red' },
  spacing: { gap: 8, margin: 2 },
  hovering: { color: { default: 'black', ':hover': 'blue' } },
  sized: (width) => ({ width }),
});

export function Row({ danger, emphasised, width, items }) {
  return (
    <div {...stylex.props(styles.base)}>
      <span {...stylex.props(styles.base, styles.emphasis)} />
      <span {...stylex.props(styles.base, danger && styles.danger)} />
      <span {...stylex.props([styles.base, styles.spacing])} />
      <span {...stylex.props([styles.base, [styles.spacing, styles.emphasis]])} />
      <span {...stylex.props(emphasised ? styles.emphasis : styles.danger)} />
      <span {...stylex.props(styles.hovering, styles.sized(width))} />
      <span {...stylex.attrs(styles.base, styles.spacing)} />
      <span {...stylex.props(items.length > 0 && styles.emphasis, styles.base)} />
    </div>
  );
}
