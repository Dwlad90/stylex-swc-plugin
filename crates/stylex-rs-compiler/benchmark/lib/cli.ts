/**
 * Shared plumbing for the benchmark gate entry points.
 *
 * Both gates (`compare-revisions.ts`, `check-budget.ts`) have the same
 * obligations: write complete diagnostics *before* exiting non-zero, so a
 * failed CI job still uploads an artifact explaining itself, and never
 * crash inside the failure path itself.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

/** True when `moduleUrl` is the script Node was started with. */
export function isMainModule(moduleUrl: string): boolean {
  const entry = process.argv[1];
  return entry !== undefined && path.resolve(entry) === fileURLToPath(moduleUrl);
}

/** Write a file, creating parent directories as needed. */
export function writeArtifact(filePath: string, contents: string): void {
  const resolved = path.resolve(filePath);
  fs.mkdirSync(path.dirname(resolved), { recursive: true });
  fs.writeFileSync(resolved, contents, 'utf8');
}

/** Append Markdown to the GitHub job summary when running in Actions. */
export function appendStepSummary(markdown: string): void {
  const stepSummary = process.env.GITHUB_STEP_SUMMARY;
  if (stepSummary) fs.appendFileSync(stepSummary, `${markdown}\n`, 'utf8');
}

/**
 * Recover a flag's value straight from `argv`. The failure path cannot
 * rely on the parsed options — parsing may be what threw.
 */
export function findArgument(argv: readonly string[], name: string): string | undefined {
  const assignment = argv.find(argument => argument.startsWith(`${name}=`));
  if (assignment !== undefined) return assignment.slice(name.length + 1) || undefined;
  const index = argv.indexOf(name);
  const value = index < 0 ? undefined : argv[index + 1];
  return value?.startsWith('--') === true ? undefined : value;
}

/**
 * Escape an error message for Markdown. Unlike a table cell this keeps
 * pipes — the message is rendered as a paragraph, not a row.
 */
export function escapeFailureMessage(message: string): string {
  return message
    .replace(/\\/g, '\\\\')
    .replace(/`/g, '\\`')
    .replace(/\p{Cc}/gu, ' ');
}

export function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function parsePositiveFloat(name: string, value: string): number {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`Invalid --${name} value: ${value}`);
  }
  return parsed;
}

export function parsePositiveInt(name: string, value: string): number {
  const parsed = Number(value);
  if (!Number.isSafeInteger(parsed) || parsed <= 0) {
    throw new Error(`Invalid --${name} value: ${value}`);
  }
  return parsed;
}

export function parseConfidence(name: string, value: string): number {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0 || parsed >= 1) {
    throw new Error(`Invalid --${name} value: ${value} (must be in (0, 1))`);
  }
  return parsed;
}
