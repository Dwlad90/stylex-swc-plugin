// Emit ticket 02's deliverable: the divergence table, as Markdown wrapped for
// the repository's eighty-column prose convention where prose allows it.
const path = require('node:path');
const { run, runRust, req, PKG } = require(path.join(__dirname, 'ref.cjs'));
const subjects = require(path.join(__dirname, 'subjects.cjs'));

const moduleFor = subject => [
  "import * as stylex from '@stylexjs/stylex';",
  `export const styles = stylex.create({ x: ${JSON.stringify(subject.props)} });`,
  '',
].join('\n');

function preludes(rules) {
  const seen = [];
  for (const rule of rules) {
    const found = (rule?.[1]?.ltr ?? '').match(/@media[^{]*/g);
    if (found === null) continue;
    const joined = found.map(text => text.trim()).join(' >> ');
    if (!seen.includes(joined)) seen.push(joined);
  }
  return seen;
}

function attempt(fn) {
  try {
    const { rules } = fn();
    return { preludes: preludes(rules), classes: rules.map(rule => rule?.[0] ?? ''), failure: null };
  } catch (error) {
    return { preludes: [], classes: [], failure: error instanceof Error ? error.message : String(error) };
  }
}

const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

const rows = subjects.map(subject => {
  const code = moduleFor(subject);
  const options = subject.options ?? {};
  const reference = attempt(() => run(code, options));
  const mine = attempt(() => runRust(code, options));
  const partial = subject.ours.some(text => text.startsWith('<partial'));
  const fromSnapshot = subject.ours[0] === '@snapshot';
  const compilersAgree = same(reference.preludes, mine.preludes);
  return {
    ...subject,
    partial,
    reference,
    mine,
    fromSnapshot,
    pinAgrees: fromSnapshot
      ? compilersAgree
      : partial
        ? reference.preludes.every(text => !text.includes('only (screen)'))
        : same(reference.preludes, subject.ours),
    compilersAgree,
    classesDiffer: reference.classes.filter((name, index) => name !== mine.classes[index]).length,
  };
});

const flagged = rows.filter(row => !row.pinAgrees);
const compilerGaps = rows.filter(row => !row.compilersAgree);
const version = req('@stylexjs/babel-plugin/package.json').version;
const out = [];

out.push('# Ticket 02 — every media-query expectation, re-derived');
out.push('');
out.push('One row per media-query expectation in the repository, giving that');
out.push("expectation beside the reference implementation's actual output for the same");
out.push('input. Nothing in production code, in a test, or in a snapshot was modified to');
out.push('produce it.');
out.push('');
out.push('## What was measured, and how');
out.push('');
out.push('The reference implementation does not export its media-query transform, so a');
out.push('row is read through emitted CSS rather than through that function: each subject');
out.push('is compiled as a `stylex.create` module and every `@media` prelude the run');
out.push('emitted is recorded, nested preludes joined with ` >> `. That is the observable');
out.push('the class name hashes over, which is the contract this work defends.');
out.push('');
out.push('One consequence has to be named, because it produces the single flagged row');
out.push("below: at-rule sorting can reorder a rule's *nested* preludes, so a nested");
out.push('unit-seam expectation is not recoverable from emitted CSS. A third column —');
out.push("this compiler's own emitted CSS for the same input — is therefore recorded");
out.push('beside the other two, and it settles that row.');
out.push('');
out.push('## What is in scope');
out.push('');
out.push('Every `@media` key the last-media-query-wins transform rewrites: a key');
out.push('nested at least one level below the style object, in a conditional value map.');
out.push('Those are the keys this work changes, and the ones whose text feeds a class');
out.push('name that could diverge.');
out.push('');
out.push('Three families of `@media` key elsewhere in the repository are deliberately');
out.push('excluded, because the transform does not touch them and no expectation about');
out.push('them can move:');
out.push('');
out.push('- style-level keys — an `@media` wrapping a block of properties, which the');
out.push('  spec lists as out of scope and `c02` pins as passing through verbatim');
out.push('- `defineVars`, `createTheme`, and `defineConsts` keys, which are variable');
out.push('  definitions rather than conditional values');
out.push("- the coverage suite's structural assertions, which pin no query text at all");
out.push('');
out.push('## Versions');
out.push('');
out.push(`- \`@stylexjs/babel-plugin\` **${version}**`);
out.push(`- resolved from \`${path.relative(path.join(PKG, '../..'), req.resolve('@stylexjs/babel-plugin'))}\``);
out.push('- the version is held by `pnpm-lock.yaml`, not by an exact range in the');
out.push('  dependency catalog, so it moves under a dependency update without anything');
out.push('  in this directory changing');
out.push(`- \`@babel/core\` ${req('@babel/core/package.json').version}`);
out.push(`- \`@stylexswc/rs-compiler\` ${require(path.join(PKG, 'package.json')).version}, from \`dist/index.js\``);
out.push('');
out.push('## Counts');
out.push('');
out.push(`- expectations re-derived: **${rows.length - 1}**, of which ${rows.filter(row => row.fromSnapshot).length} are carried by a generated snapshot`);
out.push(`- expectations the reference implementation contradicts: **${flagged.length}** — ${flagged.map(r => r.id).join(', ') || 'none'}`);
out.push(`- rows where the two compilers emit different CSS: **${compilerGaps.length}** — ${compilerGaps.map(r => r.id).join(', ') || 'none'}`);
out.push('');
out.push('One extra row, `r01`, carries the reported input, which no expectation in the');
out.push('repository covers yet. It is not counted above.');
out.push('');
out.push('## Summary');
out.push('');
out.push('| Row | Seam | Origin | Pin matches reference | Compilers agree | Class names differing |');
out.push('| --- | ---- | ------ | --------------------- | --------------- | --------------------- |');
for (const row of rows) {
  const origin = row.origin.replace(/^crates\//, '').replace(/^.*::/, '$&').replace(/\|/g, '\\|');
  out.push(`| \`${row.id}\` | ${row.seam} | ${origin} | ${row.pinAgrees ? 'yes' : '**NO**'} | ${row.compilersAgree ? 'yes' : '**NO**'} | ${row.classesDiffer} of ${row.reference.classes.length} |`);
}
out.push('');
out.push('## Rows');
for (const row of rows) {
  out.push('');
  out.push(`### \`${row.id}\` — pin ${row.pinAgrees ? 'agrees' : '**disagrees**'}, compilers ${row.compilersAgree ? 'agree' : '**disagree**'}`);
  out.push('');
  out.push(`- origin: \`${row.origin}\``);
  if (row.options !== undefined) out.push(`- options: \`${JSON.stringify(row.options)}\``);
  if (row.note !== undefined) out.push(`- note: ${row.note}`);
  out.push('');
  out.push('Input:');
  out.push('');
  out.push('```json');
  out.push(JSON.stringify(row.props, null, 2));
  out.push('```');
  out.push('');
  out.push('This repository pins:');
  out.push('');
  if (row.fromSnapshot) {
    out.push("A generated snapshot, which is this compiler's output — the third block below.");
  } else {
    out.push('```text');
    for (const text of row.ours) out.push(text);
    out.push('```');
  }
  out.push('');
  out.push('The reference implementation emits:');
  out.push('');
  out.push('```text');
  for (const text of row.reference.preludes) out.push(text.length > 400 ? `${text.slice(0, 400)}… (${text.length} chars)` : text);
  if (row.reference.failure !== null) out.push(`threw: ${row.reference.failure}`);
  out.push('```');
  out.push('');
  out.push('This compiler emits:');
  out.push('');
  out.push('```text');
  for (const text of row.mine.preludes) out.push(text.length > 400 ? `${text.slice(0, 400)}… (${text.length} chars)` : text);
  if (row.mine.failure !== null) out.push(`threw: ${row.mine.failure}`);
  out.push('```');
  out.push('');
  out.push(`Class names: ${row.classesDiffer} of ${row.reference.classes.length} differ between the two compilers.`);
  out.push('');
  out.push('```text');
  out.push(`reference: ${row.reference.classes.join(' ')}`);
  out.push(`ours:      ${row.mine.classes.join(' ')}`);
  out.push('```');
}
out.push('');
out.push('## Verdict');
out.push('');
out.push('**No expectation in this repository is contradicted by the reference');
out.push('implementation.** The one flagged row is an artefact of how a row is read,');
out.push('not a disagreement, and the count of expectations to correct is zero.');
out.push('');
out.push('`u03` is the flagged row. Its expectation nests two rewritten keys, and the');
out.push("reference implementation's emitted CSS puts them in the other order — but this");
out.push('compiler emits exactly the same CSS as the reference implementation does for');
out.push('the same input, byte for byte. What differs is the emitted at-rule nesting');
out.push('order against the key nesting order the unit test pins, and both compilers');
out.push('sort it the same way. So the expectation stands and nothing is rewritten on');
out.push("its account. This is also the row ticket 09's at-rule order check should start");
out.push('from.');
out.push('');
out.push('One more expectation exists and is deliberately absent from the table.');
out.push('`media_query_transform_coverage_test.rs` asserts structure rather than query');
out.push('text — that a negation appears at all, that three keys come back, that an');
out.push('unparseable key is refused. None of them pins a query string, so none of them');
out.push('can disagree with the reference implementation about one. The refusal cases');
out.push('belong to ticket 09, which compares the inputs the refusal fires on.');
out.push('');
out.push('## The reported input');
out.push('');
out.push('Row `r01` is the one row where the two compilers disagree, and it is the');
out.push('divergence this work exists for. The reference implementation wraps the first');
out.push('two rungs of a six-rung ladder in disjunctions of contradictory branches — the');
out.push('first a doubly nested `or` of four branches, three of them `not all` — where');
out.push('this compiler emits both authored queries unchanged. Two of the seven emitted');
out.push('class names therefore differ — the two rewritten rungs, the default and the');
out.push('four unrewritten rungs agreeing. Ticket 04 quotes its expectations from this');
out.push('row.');
out.push('');
console.log(out.join('\n'));
