// Some suites must run their assertion in a child process: a test cannot
// report an abort of the process that runs it, and it cannot reload the native
// module in a clean heap.
//
// A child process that runs a generated script starts from a file on disk,
// never from `node -e`. Two limits make `-e` unsafe for a generated script,
// and both bind on Windows alone:
//
//   * Windows limits a command line to 32767 characters, and a generated
//     script passes that length long before the shape it builds is large.
//     A longer command line does not start the child at all: `spawnSync`
//     answers a null status and no signal, which reads like a crash.
//   * Node gives `-e` source to its TypeScript parser before it runs. That
//     parser descends much deeper than V8 does for the same source, and the
//     1 MB stack that Windows gives the main thread is too small for it.
//     macOS and Linux give the main thread 8 MB, which is why the same test
//     passes there.
//
// A file has no length limit, and a `.js` file keeps the TypeScript parser
// out of the path.
//
// A script that only holds fixed text, such as the module surface checks in
// `index.spec.ts`, is small enough for `-e` and does not need this helper.
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import * as path from 'node:path';

/**
 * How much output the helper accepts from one child.
 *
 * The default of `spawnSync` is 1 MB. A test that prints a large result sits
 * too near that default, and output above the limit is cut without an error
 * that names the cause, so the limit is raised and stated here.
 */
const MAX_CHILD_OUTPUT_BYTES = 64 * 1024 * 1024;

/** What a child process answered. */
export interface NodeScriptOutcome {
  /** The signal that ended the child, or null. */
  readonly signal: NodeJS.Signals | null;
  /** The exit code, or null when the child never started. */
  readonly status: number | null;
  /** The reason the child never started, or undefined. */
  readonly error?: Error;
  readonly stdout: string;
  readonly stderr: string;
}

/** How to run one child process. */
export interface NodeScriptOptions {
  /** Environment variables for the child. Defaults to those of this process. */
  readonly env?: NodeJS.ProcessEnv;
}

/**
 * Runs one JavaScript source in a child Node process and reports the outcome.
 *
 * The source is written to a temporary file and removed afterwards. The caller
 * gets the outcome even when the child fails, so a test can assert on an exit
 * code, a signal, or a start failure.
 */
export function runNodeScript(source: string, options: NodeScriptOptions = {}): NodeScriptOutcome {
  const directory = mkdtempSync(path.join(tmpdir(), 'stylex-node-script-'));
  const scriptPath = path.join(directory, 'script.js');

  try {
    writeFileSync(scriptPath, source, 'utf8');

    const child = spawnSync(process.execPath, [scriptPath], {
      encoding: 'utf8',
      env: options.env ?? process.env,
      maxBuffer: MAX_CHILD_OUTPUT_BYTES,
    });

    return {
      signal: child.signal,
      status: child.status,
      error: child.error,
      stdout: child.stdout ?? '',
      stderr: child.stderr ?? '',
    };
  } finally {
    rmSync(directory, { force: true, recursive: true });
  }
}
