/**
 * Asks for one subject's rule count, one way or the other, and prints it.
 *
 * Run as a process of its own by `subject-process.test.ts`, because what the
 * case is about is the stderr of the process that asks. The compiler writes to
 * that file descriptor itself, so nothing inside the test process can see it.
 *
 * Usage: node <this file> <packageDir> <child|in-process> <code>
 * `code` is the module the subject is asked to compile.
 */

import path from 'node:path';
import { pathToFileURL } from 'node:url';

const [, , packageDir, mode, code] = process.argv;

// A URL and not a path: an absolute path is not a module specifier on Windows.
const lib = name => pathToFileURL(path.join(packageDir, 'benchmark', 'lib', name)).href;

const { countRulesInChild } = await import(lib('subject-process.ts'));
const { countRulesInProcess } = await import(lib('runner.ts'));
const { loadSubject } = await import(lib('subjects.ts'));

const fixture = {
  name: 'probe',
  filePath: path.join(packageDir, 'benchmark', 'perf_fixtures', 'create-basic.js'),
  code,
  weight: 'standard',
  category: 'perf',
  batchSize: 1,
};

const stylexOptions = {
  dev: false,
  treeshakeCompensation: true,
  unstable_moduleResolution: { type: 'haste', rootDir: packageDir },
};

if (mode === 'child') {
  const counts = countRulesInChild({
    subject: { label: 'subject', packageDir },
    fixtures: [fixture],
    stylexOptions,
    timeBudgetMs: 10,
  });

  console.log(JSON.stringify(counts.probe));
} else {
  const subject = await loadSubject({ label: 'subject', packageDir });

  try {
    console.log(JSON.stringify(countRulesInProcess(stylexOptions)(subject, fixture)));
  } catch (error) {
    // The same shape a refusal has when a child reports it, so the case can
    // hold both answers to one assertion.
    console.log(JSON.stringify({ refusal: error.message.split('\n')[0] }));
  }
}
