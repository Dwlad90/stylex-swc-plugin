/**
 * Times one subject, in a process that holds nothing else.
 *
 * Started by the paired benchmark on a platform that cannot hold two native
 * bindings at once. It is not run by hand: it reads a request file the parent
 * wrote and writes a report file the parent reads. `lib/subject-process.ts`
 * carries both shapes and the reason for them.
 *
 * Imports nothing that reads the fixture manifest. That module reads a value
 * off `dist/index.js`, and importing it would load this package's binding --
 * the very second binding this process exists to avoid.
 */

import fs from 'node:fs';

import {
  runWorkerRequest,
  WORKER_PROTOCOL_VERSION,
  type WorkerRequest,
} from './lib/subject-process.js';

async function main(): Promise<void> {
  const [requestPath, reportPath] = process.argv.slice(2);
  if (requestPath === undefined || reportPath === undefined) {
    throw new Error('bench-worker takes a request file and a report file');
  }

  const request = JSON.parse(fs.readFileSync(requestPath, 'utf8')) as WorkerRequest;
  if (request.protocol !== WORKER_PROTOCOL_VERSION) {
    throw new Error(
      `bench-worker reads protocol ${String(WORKER_PROTOCOL_VERSION)}, ` +
        `and the request names ${String(request.protocol)}`
    );
  }

  const report = await runWorkerRequest(request);
  fs.writeFileSync(reportPath, JSON.stringify(report), 'utf8');
}

main().catch((error: unknown) => {
  // Written to the report rather than thrown, so the parent reads one sentence
  // instead of guessing from an exit code. A parent that finds no report at all
  // says so itself, which is the case this cannot cover.
  const [, reportPath] = process.argv.slice(2);
  const failure = error instanceof Error ? error.message : String(error);
  if (reportPath !== undefined) {
    try {
      fs.writeFileSync(
        reportPath,
        JSON.stringify({ protocol: WORKER_PROTOCOL_VERSION, failure }),
        'utf8'
      );
    } catch {
      // The report cannot be written. The parent reports a child that wrote none.
    }
  }
  console.error(failure);
  process.exit(1);
});
