import { describe, expect, test } from 'vitest';

import { enclosingCallees, phfSetMembers } from '../lib/rust-source.js';

/** The calls enclosing `needle`, which each case marks its offset with. */
function calleesAt(source: string, needle: string): string[] {
  return enclosingCallees(source, source.indexOf(needle));
}

/**
 * Reading a `phf_set!` out of Rust source, which is how a list the compiler
 * owns becomes something a harness beside it can be asserted against.
 *
 * The cases are the ways a text scan can quietly answer the wrong set: the set
 * next door, a longer name that carries the wanted one, a member somebody
 * commented out, a mention that is an import rather than a declaration. Each of
 * those returns a plausible list, so a wrong answer here would read as a
 * passing assertion rather than as a failure.
 */

describe('reading a declared set', () => {
  test('the members of a set are its string literals, in source order', () => {
    const source = `pub static CALLEES: phf::Set<&'static str> = phf_set! {
  "String", "Number",
  "Math",
};`;

    expect(phfSetMembers(source, 'CALLEES')).toStrictEqual(['String', 'Number', 'Math']);
  });

  test('a set written on one line reads the same as one spread over many', () => {
    const source = `static A: phf::Set<&'static str> = phf_set! { "one", "two" };`;

    expect(phfSetMembers(source, 'A')).toStrictEqual(['one', 'two']);
  });

  test('an empty set is a set with no members, not a missing one', () => {
    expect(phfSetMembers(`static A: phf::Set<&'static str> = phf_set! {};`, 'A')).toStrictEqual([]);
  });

  test('a name the source never declares answers nothing', () => {
    expect(phfSetMembers(`static A: phf::Set<&'static str> = phf_set! { "one" };`, 'B')).toBe(
      undefined
    );
  });

  test('a name declared as something other than a set answers nothing', () => {
    // The gate this closes: a constant renamed into a plain string would
    // otherwise answer with whatever set happened to follow it.
    const source = `pub static A: &str = "one";
pub static B: phf::Set<&'static str> = phf_set! { "two" };`;

    expect(phfSetMembers(source, 'A')).toBe(undefined);
  });
});

describe('answering for the right set', () => {
  test('a set stops at its own closing brace rather than running into the next', () => {
    const source = `pub static FIRST: phf::Set<&'static str> = phf_set! {
  "one",
};

pub static SECOND: phf::Set<&'static str> = phf_set! {
  "two",
};`;

    expect(phfSetMembers(source, 'FIRST')).toStrictEqual(['one']);
    expect(phfSetMembers(source, 'SECOND')).toStrictEqual(['two']);
  });

  test('a longer name carrying the wanted one does not answer for it', () => {
    const source = `pub static INVALID_METHODS: phf::Set<&'static str> = phf_set! { "random" };
pub static METHODS: phf::Set<&'static str> = phf_set! { "trim" };`;

    expect(phfSetMembers(source, 'METHODS')).toStrictEqual(['trim']);
  });

  test('an import of the name is not the declaration of it', () => {
    const source = `use crate::constants::common::{CALLEES, OTHER};

pub static CALLEES: phf::Set<&'static str> = phf_set! { "String" };`;

    expect(phfSetMembers(source, 'CALLEES')).toStrictEqual(['String']);
  });

  test('a comment naming the set is not the declaration of it', () => {
    // The mention that reads most like a declaration and is not one. Without
    // the keyword in front, this hands back the members of somebody else's set
    // — an assertion that loads, compares and passes over the wrong list.
    const source = `// CALLEES is the list this mirrors.
pub static OTHER: phf::Set<&'static str> = phf_set! { "String" };`;

    expect(phfSetMembers(source, 'CALLEES')).toBe(undefined);
  });

  test('a call site is not the declaration either', () => {
    const source = `fn folds(name: &str) -> bool { CALLEES.contains(name) }

pub static CALLEES: phf::Set<&'static str> = phf_set! { "Math" };`;

    expect(phfSetMembers(source, 'CALLEES')).toStrictEqual(['Math']);
  });

  test('a name whose own text starts with the keyword is still a mention', () => {
    const source = `let mystatic CALLEES = 1;
pub static OTHER: phf::Set<&'static str> = phf_set! { "String" };`;

    expect(phfSetMembers(source, 'CALLEES')).toBe(undefined);
  });

  test('a set declared as a const reads the same as one declared static', () => {
    expect(
      phfSetMembers(`const A: phf::Set<&'static str> = phf_set! { "one" };`, 'A')
    ).toStrictEqual(['one']);
  });

  test('a use that never reaches a declaration answers nothing', () => {
    // The set below belongs to another name, and a scan that took the first
    // `phf_set!` after the mention would hand it over under this one.
    const source = `use crate::constants::common::CALLEES;

pub static OTHER: phf::Set<&'static str> = phf_set! { "String" };`;

    expect(phfSetMembers(source, 'CALLEES')).toBe(undefined);
  });
});

describe('what the text around a member does not add to it', () => {
  test('a commented-out member is not a member', () => {
    const source = `pub static A: phf::Set<&'static str> = phf_set! {
  "kept",
  // "dropped",
  /* "also dropped" */
};`;

    expect(phfSetMembers(source, 'A')).toStrictEqual(['kept']);
  });

  test('a comment above the declaration is not read as part of it', () => {
    const source = `// Populated from "MDN data", in alphabetical order.
pub static A: phf::Set<&'static str> = phf_set! { "kept" };`;

    expect(phfSetMembers(source, 'A')).toStrictEqual(['kept']);
  });

  test('an escaped quote inside a member stays inside it', () => {
    const source = `static A: phf::Set<&'static str> = phf_set! { "a\\"b", "c" };`;

    expect(phfSetMembers(source, 'A')).toStrictEqual(['a"b', 'c']);
  });

  test('a raw string member is taken verbatim', () => {
    const source = `static A: phf::Set<&'static str> = phf_set! { r#"a\\b"# };`;

    expect(phfSetMembers(source, 'A')).toStrictEqual(['a\\b']);
  });

  test('a set of thousands is read whole', () => {
    const members = Array.from({ length: 5_000 }, (_, index) => `m${index}`);
    const source = `static A: phf::Set<&'static str> = phf_set! {
${members.map(member => `  "${member}",`).join('\n')}
};`;

    expect(phfSetMembers(source, 'A')).toStrictEqual(members);
  });
});

/**
 * Reading which calls enclose an offset, which is how an extractor learns
 * where a literal was going rather than only what it spells.
 *
 * The cases are the ways the answer can be plausible and wrong: a call that
 * closed before the offset, a name that is not a callee at all, and a chain
 * that reaches past the statement -- the last of which matters because
 * comments are not masked, so an unclosed bracket written in prose would
 * otherwise be reported as the caller of everything below it.
 *
 * Offsets are taken with `indexOf`, so a case reads as the source does.
 */
describe('reading the calls that enclose an offset', () => {
  test('names the call whose argument list the offset is in', () => {
    expect(calleesAt('assert!(message.contains(NEEDLE));', 'NEEDLE')).toStrictEqual([
      'contains',
      'assert!',
    ]);
  });

  test('names a macro with the bang that spells it', () => {
    expect(calleesAt('assert_eq!(actual, MESSAGE);', 'MESSAGE')).toStrictEqual(['assert_eq!']);
  });

  test('a call that closed before the offset does not enclose it', () => {
    expect(calleesAt('assert!(first(a) && second(b) == NEEDLE);', 'NEEDLE')).toStrictEqual([
      'assert!',
    ]);
  });

  test('stops at the statement before, so an earlier call is not a caller', () => {
    expect(calleesAt('let a = join(x);\nlet b = NEEDLE;', 'NEEDLE')).toStrictEqual([]);
  });

  test('stops at a block boundary, so a closure body is not the call it sits in', () => {
    expect(calleesAt('catch_unwind(|| { NEEDLE })', 'NEEDLE')).toStrictEqual([]);
  });

  test('an unclosed bracket in prose is not reported as a caller', () => {
    // Comments are not masked, so prose that spells a call must be stepped
    // over or it reads as the caller of the statement under it.
    expect(calleesAt('// a value such as calc(1px\nlet a = NEEDLE;', 'NEEDLE')).toStrictEqual([]);
    expect(calleesAt('let a = /* like contains( */ NEEDLE;', 'NEEDLE')).toStrictEqual([]);
  });

  test('a semicolon in prose does not end the walk', () => {
    // The bound is a statement boundary in *code*. A comment that spells one
    // would otherwise hide the assertion the literal is an argument of.
    expect(
      calleesAt('assert!(\n  // the guard names both; see the report\n  x == NEEDLE\n);', 'NEEDLE')
    ).toStrictEqual(['assert!']);
  });

  test('a bracket in a character literal counts for nothing', () => {
    // `'('` survives masking, which blanks string literals only. Counting it
    // cancels the parenthesis that really does close, and the call around it
    // is then read as one the offset sits inside.
    expect(calleesAt("let a = x.matches('(').count() + NEEDLE;", 'NEEDLE')).toStrictEqual([]);
    expect(calleesAt("let a = f(x, ']', NEEDLE);", 'NEEDLE')).toStrictEqual(['f']);
  });

  test('a lifetime is an apostrophe that opens nothing', () => {
    expect(calleesAt("fn f(v: &'static str) -> T { g(NEEDLE) }", 'NEEDLE')).toStrictEqual(['g']);
  });

  test('a closure body ends the walk, so the call around it is not read', () => {
    // Deliberate: the suites wrap the call under test in
    // `catch_unwind(AssertUnwindSafe(|| { … }))`, and reading the assertion
    // outside would drop the value the compiler was actually given.
    expect(
      calleesAt('assert!(catch_unwind(|| { normalize(NEEDLE) }).is_err());', 'NEEDLE')
    ).toStrictEqual(['normalize']);
  });

  test('a bracket with no name in front of it names no callee', () => {
    expect(calleesAt('let a = (1 + NEEDLE);', 'NEEDLE')).toStrictEqual([]);
    expect(calleesAt('let a = &[NEEDLE];', 'NEEDLE')).toStrictEqual([]);
  });

  test('reads the chain of a call nested a hundred deep', () => {
    const source = `assert!(${'w('.repeat(100)}NEEDLE${')'.repeat(100)});`;

    expect(enclosingCallees(source, source.indexOf('NEEDLE'))).toHaveLength(101);
  });

  test('a call further back than the window does not enclose the offset', () => {
    // A statement has no bound, and a case table is one statement holding
    // hundreds of rows. Without the window every literal in it would walk
    // back to the head of the statement, which is the walk once per row. A
    // literal that outruns the window is kept rather than dropped, which is
    // the safe way round for a guard.
    const source = `assert!(${'a, '.repeat(400)}NEEDLE);`;

    expect(enclosingCallees(source, source.indexOf('NEEDLE'))).toStrictEqual([]);
  });
});
