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

/**
 * The StyleX option keys a fixture may override, each carrying a boolean.
 *
 * An allowlist rather than `Partial<StyleXOptions>`, because a manifest is data
 * from a file: a key nobody validated would be a silently ignored measurement
 * condition, and two fixtures could then differ in a way no reader can see. Add
 * a key here when a fixture needs it, and the loader will start accepting it.
 *
 * Everything here is a *development or compatibility* feature — the work a
 * production build does not do. That is what these exist to price.
 */
export const BOOLEAN_OPTION_KEYS = [
  'dev',
  'debug',
  'test',
  'enableDebugClassNames',
  'enableDebugDataProp',
  'enableDevClassNames',
  'enableMinifiedKeys',
  'enableFontSizePxToRem',
  'enableInlinedConditionalMerge',
  'enableLogicalStylesPolyfill',
  'enableLegacyValueFlipping',
  'enableLTRRTLComments',
  'enableMediaQueryOrder',
  'legacyDisableLayers',
  'useRealFileForSource',
  'treeshakeCompensation',
  'inlineSourcesContent',
  'emitSourceMapColumns',
] as const;

export type BooleanOptionKey = (typeof BOOLEAN_OPTION_KEYS)[number];

export const STYLE_RESOLUTIONS = [
  'application-order',
  'property-specificity',
  'legacy-expand-shorthands',
] as const;

export const PROPERTY_VALIDATION_MODES = ['throw', 'warn', 'silent'] as const;

/**
 * A fixture's own measurement conditions, as the manifest declares them.
 *
 * `sourceMap` is deliberately absent: the generated typings spell it as a
 * `const enum`, whose members cannot be constructed from a JSON string without
 * asserting a type nobody checked. A fixture that needs source-map emission
 * priced wants that enum plumbed properly first.
 */
export type FixtureOptionOverrides = Partial<
  Record<BooleanOptionKey, boolean> & {
    styleResolution: (typeof STYLE_RESOLUTIONS)[number];
    propertyValidationMode: (typeof PROPERTY_VALIDATION_MODES)[number];
  }
>;

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
   * Number of `transform` invocations per timed operation. Raised per
   * fixture to lift sub-millisecond work above timer noise; 1 by default.
   */
  batchSize: number;
  /**
   * Overrides `dev` for this fixture alone. Absent means the shared
   * production shape from `createStylexOptions`, which is what all but one
   * fixture wants; `guidelines/PERFORMANCE.md` says why the two shapes are
   * watched separately rather than switched between.
   *
   * Kept beside [`FixtureDescriptor.options`] rather than folded into it: the
   * fixtures that predate the option map spell `dev` this way, and their trend
   * series are named after those entries.
   */
  dev?: boolean;
  /**
   * Every other measurement condition this fixture asks for, applied over the
   * shared options and after `dev`. A fixture measuring a development feature
   * names it here, so the option shape and the trend series it feeds are one
   * entry in one file.
   */
  options?: FixtureOptionOverrides;
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
  /**
   * Bootstrap statistics, present only when the producer was asked for
   * them. `bench-revisions.ts` records the roles above but deliberately
   * leaves the statistics to the verdict engine, so a release raw-stats
   * file carries roles without ratios. Consumers that resolve a subject
   * by role must not require these.
   */
  ratios?: readonly number[];
  confidence?: BootstrapInterval;
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
  /** Runner image family, e.g. `ubuntu24` (`ImageOS`). */
  runnerImage?: string;
  /**
   * Exact runner image build, e.g. `20260803.1.0` (`ImageVersion`). The
   * family stays stable while GitHub rebuilds the image underneath it, so
   * the budget check pins this separately.
   */
  runnerImageVersion?: string;
}

export interface RawStatsFile {
  schemaVersion: typeof RAW_STATS_SCHEMA_VERSION;
  environment: RawStatsEnvironment;
  subjects: readonly SubjectDescriptor[];
  bootstrap?: BootstrapConfig;
  fixtures: readonly FixtureRawStats[];
}
