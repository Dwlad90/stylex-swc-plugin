/**
 * Field separator for the composite keys the harness builds — a declaration's
 * identity, and the joined class-name/rule lists a verdict compares.
 *
 * NUL is the one character a CSS property name, a CSS value, a class name and
 * a rule body cannot contain, so no two distinct inputs can collide onto one
 * key. Spelled as an escape: a literal NUL in a source file makes git treat
 * that file as binary and drop it out of every diff.
 */
export const SEPARATOR = '\u0000';
