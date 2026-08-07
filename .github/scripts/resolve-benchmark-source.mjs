/**
 * Decide whether a completed benchmark run still describes the current PR.
 *
 * A benchmark measures the PR head against `merge-base(develop, head)`. That
 * comparison stays valid for as long as the head is unchanged: it is a
 * statement about what the branch does to the code it forked from, not about
 * the current tip of the base branch.
 *
 * So staleness is decided by the head SHA alone. The previous rule also
 * compared `pull_requests[0].base.sha` -- the base *branch tip* -- between the
 * source run and now, which meant any unrelated push to `develop` in the
 * minutes between measuring and reporting silently discarded a perfectly valid
 * report and left the PR with no comment at all.
 */

import path from 'node:path';
import { fileURLToPath } from 'node:url';

export function resolveBenchmarkSource(input) {
  const sourceHeadSha = sha(input.sourceHeadSha, 'sourceHeadSha');
  const currentHeadSha = sha(input.currentHeadSha, 'currentHeadSha');

  return { stale: sourceHeadSha !== currentHeadSha, headSha: currentHeadSha };
}

function sha(value, name) {
  if (typeof value !== 'string' || !/^[a-f\d]{40}$/.test(value)) {
    throw new Error(`${name} must be a full lowercase commit SHA`);
  }
  return value;
}

function parseCli(argv) {
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const name = argv[index];
    const value = argv[index + 1];
    if (!name?.startsWith('--') || value === undefined) {
      throw new Error('Expected --name value arguments');
    }
    values.set(name, value);
  }
  for (const required of ['--source-head-sha', '--current-head-sha']) {
    if (!values.has(required)) throw new Error(`${required} is required`);
  }
  return values;
}

function isMainModule() {
  return (
    process.argv[1] !== undefined &&
    path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
  );
}

if (isMainModule()) {
  try {
    const options = parseCli(process.argv.slice(2));
    const result = resolveBenchmarkSource({
      sourceHeadSha: options.get('--source-head-sha'),
      currentHeadSha: options.get('--current-head-sha'),
    });
    process.stdout.write(`${JSON.stringify(result)}\n`);
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  }
}
