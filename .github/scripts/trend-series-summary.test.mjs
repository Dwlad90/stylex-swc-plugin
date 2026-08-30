/**
 * Guards the note that qualifies the trend-series table.
 *
 * `github-action-benchmark` writes a Current / Previous / Ratio table to the
 * job summary. The two columns come from two runners, so the ratio holds
 * machine-to-machine noise together with any code change. Without a note the
 * reader takes the ratio for a verdict and reports a regression that the gate
 * does not see. These checks keep the note, and keep it beside the table.
 */

import assert from 'node:assert/strict';
import fs from 'node:fs';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const WORKFLOW = fileURLToPath(new URL('../workflows/pr-validation.yml', import.meta.url));
const STEP_NAME = 'Explain the trend-series comparison';
const RESTORE_OUTPUT = "steps.benchmark-cache-restore.outputs.cache-matched-key != ''";

function workflow() {
  return fs.readFileSync(WORKFLOW, 'utf8');
}

/** Returns the lines of one step, from its name to the next step. */
function stepBody(source, name) {
  const lines = source.split('\n');
  const start = lines.findIndex(line => line.includes(`- name: ${name}`));
  assert.notEqual(start, -1, `no step named "${name}" in pr-validation.yml`);

  const rest = lines.slice(start + 1);
  const end = rest.findIndex(line => /^\s{6}- name: /.test(line));
  return (end === -1 ? rest : rest.slice(0, end)).join('\n');
}

void test('the workflow explains the trend-series comparison', () => {
  assert.ok(workflow().includes(`- name: ${STEP_NAME}`));
});

void test('the note runs only when a baseline was restored', () => {
  // With no baseline the action writes no comparison, so a note about "the
  // table above" would qualify a table that is not there.
  assert.match(
    stepBody(workflow(), STEP_NAME),
    new RegExp(RESTORE_OUTPUT.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'))
  );
});

void test('the note reaches the job summary', () => {
  assert.match(stepBody(workflow(), STEP_NAME), /GITHUB_STEP_SUMMARY/);
});

void test('the note says that the two columns come from two machines', () => {
  assert.match(stepBody(workflow(), STEP_NAME), /compares two machines/);
});

void test('the note gives the size of the runner noise', () => {
  const body = stepBody(workflow(), STEP_NAME);

  assert.match(body, /16%/);
  assert.match(body, /34%/);
});

void test('the note names the check that does gate', () => {
  assert.match(stepBody(workflow(), STEP_NAME), /paired revision benchmark/);
});

void test('the note says that the table never gates', () => {
  assert.match(stepBody(workflow(), STEP_NAME), /advisory and never gates/);
});

// The note qualifies the table that `summary-always` writes. If that option
// goes away, the table goes away and the note describes nothing.
void test('the action still writes the table that the note qualifies', () => {
  assert.match(workflow(), /summary-always:\s*true/);
});

// A note above the table would qualify the wrong thing.
void test('the note comes after the step that writes the table', () => {
  const source = workflow();

  assert.ok(
    source.indexOf('- name: Store benchmark result') < source.indexOf(`- name: ${STEP_NAME}`)
  );
});
