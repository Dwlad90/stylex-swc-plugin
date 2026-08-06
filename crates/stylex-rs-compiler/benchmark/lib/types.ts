/**
 * Shared benchmark types.
 *
 * `raw-stats.v1.json` is the authoritative machine-readable artifact
 * consumed by budget checks and the verdict engine. The human-readable
 * `output.json` / `output-extended.txt` are display-only.
 *
 * Bump `RAW_STATS_SCHEMA_VERSION` on any breaking change to the shape below.
 */

export const RAW_STATS_SCHEMA_VERSION = 1 as const;

export type FixtureWeight = 'standard' | 'heavy';
export type FixtureCategory = 'transform' | 'perf' | 'rollup';

export interface FixtureDescriptor {
  /** Stable identifier used across runs (never derived from mutable paths). */
  name: string;
  /** Absolute path used at load time. Not part of raw-stats output. */
  filePath: string;
  /** Preloaded source. Kept in-memory for the duration of a run. */
  code: string;
  /** Selects tinybench iteration budget. */
  weight: FixtureWeight;
  category: FixtureCategory;
  /**
   * Number of `transform` invocations per timed operation. Phase 0 will
   * calibrate this per fixture; until then all fixtures batch by 1.
   */
  batchSize: number;
}

export interface SubjectDescriptor {
  /** Human-readable label used in diagnostics and the raw-stats output. */
  label: string;
  /** Version reported by the subject's package.json when available. */
  version: string;
  /** Absolute path from which the subject was loaded. */
  resolvedFrom: string;
}

/** Deterministic bootstrap parameters — logged so the verdict is reproducible. */
export interface BootstrapConfig {
  /** Explicit seed. Never use a wall-clock default. */
  seed: number;
  /** Number of bootstrap resamples. */
  resamples: number;
  /** One-sided confidence level, e.g. 0.95. */
  confidence: number;
}

export interface RawLatencySamples {
  /** Sorted samples in milliseconds. */
  samples: readonly number[];
  /** Median (p50) in milliseconds. */
  p50: number;
  /** 95th percentile in milliseconds. */
  p95: number;
  /** Relative margin of error reported by tinybench, percentage points. */
  rme: number;
  /** Sample count. */
  samplesCount: number;
  /** Mean throughput (ops/sec) as reported by tinybench. */
  opsPerSec: number;
}

export interface FixtureRoundStats {
  /** 0-based round index within the fixture. */
  round: number;
  /** Subject execution order for this round, seeded and logged. */
  subjectOrder: readonly string[];
  /** One entry per subject label. */
  perSubject: Record<string, RawLatencySamples>;
}

/**
 * Median-ratio bootstrap output. `point` is the median of observed round
 * ratios; `lower`/`upper` are one-sided bounds at the configured
 * confidence level. Consumed by the verdict engine and the budget check.
 */
export interface BootstrapInterval {
  point: number;
  lower: number;
  upper: number;
}

/**
 * Paired-comparison output for a fixture. Populated when a run has
 * exactly two subjects and a bootstrap config; absent otherwise.
 * `base` is the first subject in the run and `candidate` the second, so
 * `ratios[i] = candidate_p50 / base_p50` for round i.
 */
export interface FixturePairedStats {
  base: string;
  candidate: string;
  ratios: readonly number[];
  confidence: BootstrapInterval;
}

export interface FixtureRawStats {
  name: string;
  weight: FixtureWeight;
  category: FixtureCategory;
  batchSize: number;
  rounds: readonly FixtureRoundStats[];
  /** Present only for paired runs; see `FixturePairedStats`. */
  paired?: FixturePairedStats;
}

export interface RawStatsEnvironment {
  timestamp: string;
  node: string;
  os: {
    type: string;
    release: string;
    arch: string;
    platform: string;
  };
  cpu: {
    model: string;
    cores: number;
  };
  memoryGB: number;
  packageVersion: string;
  target: string;
  toolchain: {
    rust?: string;
  };
  commit?: string;
  runnerImage?: string;
}

export interface RawStatsFile {
  schemaVersion: typeof RAW_STATS_SCHEMA_VERSION;
  environment: RawStatsEnvironment;
  subjects: readonly SubjectDescriptor[];
  bootstrap?: BootstrapConfig;
  fixtures: readonly FixtureRawStats[];
}
