/**
 * Regular-expression helpers.
 *
 * A leaf module with no imports of its own, so that `module-selection` can
 * use it without pulling in the plugin core and its webpack dependencies.
 */

/** Escapes `value` so that it matches itself inside a pattern. */
export function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
