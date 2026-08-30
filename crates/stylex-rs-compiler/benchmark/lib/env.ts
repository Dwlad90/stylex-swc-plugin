/**
 * Environment metadata capture for raw-stats and reporting.
 *
 * `github-action-benchmark` compares numbers alone, but the budget check
 * requires knowing the OS image, Node version, CPU, and target that
 * produced a measurement so an image change forces recalibration instead
 * of a silent regression comparison.
 */

import { execSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { isRecord } from './json.js';
import type { RawStatsEnvironment } from './types.js';

export interface CaptureEnvironmentOptions {
  packageDir: string;
  workspaceRoot: string;
  target?: string;
  /**
   * Directory that supplies the git HEAD. Defaults to the current directory.
   * Callers give it so a test does not have to change the process directory.
   */
  cwd?: string;
}

export function captureEnvironment(options: CaptureEnvironmentOptions): RawStatsEnvironment {
  const packageVersion = readJsonField(path.join(options.packageDir, 'package.json'), 'version');
  const rust = detectRustToolchain();
  const commit = detectCommit(options.cwd);
  // GitHub exposes the image family as `ImageOS` (e.g. `ubuntu24`) and the
  // exact build as `ImageVersion` (e.g. `20260803.1.0`). `RUNNER_IMAGE` is
  // an optional override for self-hosted or containerised runs.
  const runnerImage = process.env.RUNNER_IMAGE || process.env.ImageOS || undefined;
  const runnerImageVersion = process.env.ImageVersion || undefined;

  return {
    timestamp: new Date().toISOString(),
    node: process.version,
    os: {
      type: os.type(),
      release: os.release(),
      arch: os.arch(),
      platform: os.platform(),
    },
    cpu: {
      model: os.cpus()[0]?.model ?? 'unknown',
      cores: os.cpus().length,
    },
    memoryGB: Math.round(os.totalmem() / (1024 * 1024 * 1024)),
    packageVersion,
    target: options.target ?? detectTarget(),
    toolchain: rust ? { rust } : {},
    commit,
    runnerImage,
    runnerImageVersion,
  };
}

function readJsonField(filePath: string, field: string): string {
  try {
    const raw: unknown = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
    if (!isRecord(raw)) return 'unknown';
    const value = raw[field];
    return typeof value === 'string' ? value : 'unknown';
  } catch {
    return 'unknown';
  }
}

function detectRustToolchain(): string | undefined {
  try {
    const out = execSync('rustc --version', {
      stdio: ['ignore', 'pipe', 'ignore'],
      encoding: 'utf-8',
    });
    return out.trim() || undefined;
  } catch {
    return undefined;
  }
}

/**
 * Finds the commit that names the tree these numbers came from.
 *
 * The checked-out HEAD is that commit, so read it first.
 *
 * `GITHUB_SHA` is not that commit on a `pull_request` event. It holds the
 * merge SHA from the event payload. GitHub makes the test-merge again in the
 * background, and the checkout can replace that merge commit before the job
 * starts. The workflow warns when this occurs, and it measures the ref as
 * checked out.
 *
 * A run that records the payload SHA gives its numbers to a tree that no job
 * measured. Two runs of one commit then look like two commits, and runner
 * noise looks like a regression.
 *
 * The environment variables stay as the fallback. A checkout that holds no git
 * metadata has no HEAD to read.
 */
function detectCommit(cwd?: string): string | undefined {
  try {
    const out = execSync('git rev-parse HEAD', {
      cwd,
      stdio: ['ignore', 'pipe', 'ignore'],
      encoding: 'utf-8',
    });
    const head = out.trim();
    if (head) return head;
  } catch {
    // Not a git checkout, or git is unavailable -- fall through to the env.
  }
  return process.env.GITHUB_SHA || process.env.CI_COMMIT_SHA || undefined;
}

function detectTarget(): string {
  const platform = process.platform;
  const arch = process.arch;

  const archMap: Record<string, string> = {
    x64: 'x86_64',
    arm64: 'aarch64',
  };
  const rustArch = archMap[arch] ?? arch;

  if (platform === 'darwin') return `${rustArch}-apple-darwin`;
  if (platform === 'win32') return `${rustArch}-pc-windows-msvc`;
  if (platform === 'linux') {
    const abi = isMuslLibc() ? 'musl' : 'gnu';
    return `${rustArch}-unknown-linux-${abi}`;
  }
  return `${rustArch}-${platform}`;
}

function isMuslLibc(): boolean {
  try {
    const out = execSync('ldd --version 2>&1 || true', {
      stdio: ['ignore', 'pipe', 'ignore'],
      encoding: 'utf-8',
    });
    return /musl/i.test(out);
  } catch {
    return false;
  }
}
