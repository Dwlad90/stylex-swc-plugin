#!/usr/bin/env node
/**
 * Write `paired-benchmark-identity.v1.json` next to the PR benchmark results.
 *
 * The trusted reporter in `benchmark-report.yml` re-validates this file before
 * it will comment, so every field is validated on the way out too -- a
 * malformed identity should fail the run that produced it, not silently
 * degrade into an "unavailable" comment an hour later.
 */

import { fail, requireEnv } from './lib/ci.mjs';
import { buildPairedBenchmarkIdentity, writeJsonDocument } from './lib/paired-benchmark.mjs';

try {
  const identity = buildPairedBenchmarkIdentity({
    runId: requireEnv('GITHUB_RUN_ID'),
    prNumber: Number(requireEnv('PR_NUMBER')),
    headSha: requireEnv('HEAD_SHA'),
    candidateSha: requireEnv('CANDIDATE_SHA'),
    baseSha: requireEnv('BASE_SHA'),
    target: requireEnv('BENCHMARK_TARGET'),
    nodeAbi: requireEnv('NODE_ABI'),
    subjectSchemaVersion: Number(requireEnv('BENCHMARK_SUBJECT_SCHEMA_VERSION')),
  });
  const written = writeJsonDocument(requireEnv('IDENTITY_OUTPUT'), identity);
  console.log(`Recorded paired benchmark identity: ${written}`);
} catch (error) {
  fail(error instanceof Error ? error.message : String(error));
}
