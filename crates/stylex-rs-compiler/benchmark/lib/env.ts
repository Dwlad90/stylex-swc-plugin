/**
 * Environment metadata capture for raw-stats and reporting.
 *
 * `github-action-benchmark` compares numbers alone, but the budget check
 * (Phase 6) requires knowing the OS image, Node version, CPU, and target
 * that produced a measurement so an image change forces recalibration
 * instead of a silent regression comparison.
 */

import { execSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import type { RawStatsEnvironment } from './types.js';

export interface CaptureEnvironmentOptions {
  packageDir: string;
  workspaceRoot: string;
  target?: string;
}

export function captureEnvironment(options: CaptureEnvironmentOptions): RawStatsEnvironment {
  const packageVersion = readJsonField(path.join(options.packageDir, 'package.json'), 'version');
  const rust = detectRustToolchain();
  const commit = detectCommit();
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
    const raw = JSON.parse(fs.readFileSync(filePath, 'utf-8')) as Record<string, unknown>;
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

function detectCommit(): string | undefined {
  const fromEnv = process.env.GITHUB_SHA || process.env.CI_COMMIT_SHA;
  if (fromEnv) return fromEnv;
  try {
    const out = execSync('git rev-parse HEAD', {
      stdio: ['ignore', 'pipe', 'ignore'],
      encoding: 'utf-8',
    });
    return out.trim() || undefined;
  } catch {
    return undefined;
  }
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
