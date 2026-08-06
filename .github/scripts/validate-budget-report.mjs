/**
 * Assert that the canonical Linux x64 GNU leg produced a budget report and
 * that the report did not fail.
 *
 * The budget step itself already fails its own job on a breach, but a
 * skipped step, a lost artifact, or a report written by the error path
 * would otherwise be invisible to the release. This runs in the single
 * aggregation job that publication depends on, so any of those blocks the
 * release instead of passing quietly.
 */

import fs from 'node:fs';

import { fail, failWithErrors, requireEnv } from './lib/ci.mjs';

const reportPath = requireEnv('BUDGET_REPORT');

if (!fs.existsSync(reportPath)) {
  fail(`Absolute budget report is missing: ${reportPath}`);
}

let report;
try {
  report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
} catch (error) {
  fail(`Absolute budget report is not valid JSON: ${error.message}`);
}

if (report === null || typeof report !== 'object' || Array.isArray(report)) {
  fail('Absolute budget report must be an object');
}

if (report.schemaVersion !== 1) {
  fail(`Unsupported budget report schemaVersion: ${String(report.schemaVersion)}`);
}

if (report.status === 'error') {
  fail(`Budget check errored: ${report.error?.message ?? 'unknown error'}`);
}

if (report.status === 'failed') {
  failWithErrors(
    'Absolute p95 budget failed:',
    (report.problems ?? []).map(problem => `${problem.kind}: ${problem.message}`)
  );
}

if (report.status !== 'pass' && report.status !== 'unseeded') {
  fail(`Unexpected budget report status: ${String(report.status)}`);
}

if (report.reportOnly === true) {
  fail('Budget report was produced in report-only mode; the release gate requires an enforced run');
}

const measured = Array.isArray(report.fixtures) ? report.fixtures.length : 0;
if (measured === 0) {
  fail('Budget report contains no measured benchmarks');
}

console.log(
  `Absolute p95 budget: ${report.status} (${measured} benchmarks, budget state ${String(report.budgetState)})`
);
