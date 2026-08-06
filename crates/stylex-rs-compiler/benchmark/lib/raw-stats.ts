import {
  RAW_STATS_SCHEMA_VERSION,
  type BootstrapConfig,
  type BootstrapInterval,
  type FixturePairedStats,
  type FixtureRawStats,
  type FixtureRoundStats,
  type RawLatencySamples,
  type RawStatsEnvironment,
  type RawStatsFile,
  type SubjectDescriptor,
} from './types.js';

type JsonRecord = Record<string, unknown>;

export function parseRawStats(value: unknown, source: string): RawStatsFile {
  const file = requireRecord(value, source);
  if (file.schemaVersion !== RAW_STATS_SCHEMA_VERSION) {
    throw new Error(
      `${source} schemaVersion ${String(file.schemaVersion)} is not supported (expected ${String(RAW_STATS_SCHEMA_VERSION)})`
    );
  }

  const subjects = requireArray(file.subjects, `${source}.subjects`).map((subject, index) =>
    parseSubject(subject, `${source}.subjects[${index}]`)
  );
  if (subjects.length !== 2) {
    throw new Error(`${source} must expose exactly two subjects`);
  }
  if (subjects[0]?.label === subjects[1]?.label) {
    throw new Error(`${source} subject labels must be unique`);
  }

  const fixtures = requireArray(file.fixtures, `${source}.fixtures`).map((fixture, index) =>
    parseFixture(fixture, `${source}.fixtures[${index}]`, subjects)
  );
  if (fixtures.length === 0) throw new Error(`${source} has no fixtures`);
  assertUnique(
    fixtures.map(fixture => fixture.name),
    `${source} fixture names`
  );

  return {
    schemaVersion: RAW_STATS_SCHEMA_VERSION,
    environment: parseEnvironment(file.environment, `${source}.environment`),
    subjects,
    ...(file.bootstrap === undefined
      ? {}
      : { bootstrap: parseBootstrap(file.bootstrap, `${source}.bootstrap`) }),
    fixtures,
  };
}

function parseSubject(value: unknown, context: string): SubjectDescriptor {
  const subject = requireRecord(value, context);
  return {
    label: requireString(subject.label, `${context}.label`),
    version: requireString(subject.version, `${context}.version`),
    resolvedFrom: requireString(subject.resolvedFrom, `${context}.resolvedFrom`),
  };
}

function parseEnvironment(value: unknown, context: string): RawStatsEnvironment {
  const environment = requireRecord(value, context);
  const os = requireRecord(environment.os, `${context}.os`);
  const cpu = requireRecord(environment.cpu, `${context}.cpu`);
  const toolchain = requireRecord(environment.toolchain, `${context}.toolchain`);
  const rust = optionalString(toolchain.rust, `${context}.toolchain.rust`);
  const commit = optionalString(environment.commit, `${context}.commit`);
  const runnerImage = optionalString(environment.runnerImage, `${context}.runnerImage`);

  return {
    timestamp: requireString(environment.timestamp, `${context}.timestamp`),
    node: requireString(environment.node, `${context}.node`),
    os: {
      type: requireString(os.type, `${context}.os.type`),
      release: requireString(os.release, `${context}.os.release`),
      arch: requireString(os.arch, `${context}.os.arch`),
      platform: requireString(os.platform, `${context}.os.platform`),
    },
    cpu: {
      model: requireString(cpu.model, `${context}.cpu.model`),
      cores: requirePositiveInteger(cpu.cores, `${context}.cpu.cores`),
    },
    memoryGB: requirePositiveNumber(environment.memoryGB, `${context}.memoryGB`),
    packageVersion: requireString(environment.packageVersion, `${context}.packageVersion`),
    target: requireString(environment.target, `${context}.target`),
    toolchain: rust === undefined ? {} : { rust },
    ...(commit === undefined ? {} : { commit }),
    ...(runnerImage === undefined ? {} : { runnerImage }),
  };
}

function parseBootstrap(value: unknown, context: string): BootstrapConfig {
  const bootstrap = requireRecord(value, context);
  const confidence = requirePositiveNumber(bootstrap.confidence, `${context}.confidence`);
  if (confidence >= 1) throw new Error(`${context}.confidence must be less than 1`);
  return {
    seed: requireInteger(bootstrap.seed, `${context}.seed`),
    resamples: requirePositiveInteger(bootstrap.resamples, `${context}.resamples`),
    confidence,
  };
}

function parseFixture(
  value: unknown,
  context: string,
  subjects: readonly SubjectDescriptor[]
): FixtureRawStats {
  const fixture = requireRecord(value, context);
  const weight = fixture.weight;
  if (weight !== 'standard' && weight !== 'heavy') {
    throw new Error(`${context}.weight must be "standard" or "heavy"`);
  }
  const category = fixture.category;
  if (category !== 'transform' && category !== 'perf' && category !== 'rollup') {
    throw new Error(`${context}.category is not supported`);
  }

  const rounds = requireArray(fixture.rounds, `${context}.rounds`).map((round, index) =>
    parseRound(round, `${context}.rounds[${index}]`, subjects)
  );
  if (rounds.length === 0) throw new Error(`${context}.rounds must not be empty`);
  for (const [index, round] of rounds.entries()) {
    if (round.round !== index) {
      throw new Error(`${context}.rounds must use contiguous zero-based indices`);
    }
  }

  return {
    name: requireString(fixture.name, `${context}.name`),
    weight,
    category,
    batchSize: requirePositiveInteger(fixture.batchSize, `${context}.batchSize`),
    rounds,
    ...(fixture.paired === undefined
      ? {}
      : { paired: parsePaired(fixture.paired, `${context}.paired`) }),
  };
}

function parseRound(
  value: unknown,
  context: string,
  subjects: readonly SubjectDescriptor[]
): FixtureRoundStats {
  const round = requireRecord(value, context);
  const subjectOrder = requireArray(round.subjectOrder, `${context}.subjectOrder`).map(
    (label, index) => requireString(label, `${context}.subjectOrder[${index}]`)
  );
  const expectedLabels = subjects.map(subject => subject.label).toSorted();
  if (
    subjectOrder.length !== expectedLabels.length ||
    subjectOrder.toSorted().some((label, index) => label !== expectedLabels[index])
  ) {
    throw new Error(`${context}.subjectOrder must contain each subject label exactly once`);
  }

  const rawPerSubject = requireRecord(round.perSubject, `${context}.perSubject`);
  const perSubject: Record<string, RawLatencySamples> = {};
  for (const subject of subjects) {
    perSubject[subject.label] = parseSamples(
      rawPerSubject[subject.label],
      `${context}.perSubject[${JSON.stringify(subject.label)}]`
    );
  }

  return {
    round: requireNonNegativeInteger(round.round, `${context}.round`),
    subjectOrder,
    perSubject,
  };
}

function parseSamples(value: unknown, context: string): RawLatencySamples {
  const stats = requireRecord(value, context);
  const samples = requireArray(stats.samples, `${context}.samples`).map((sample, index) =>
    requirePositiveNumber(sample, `${context}.samples[${index}]`)
  );
  if (samples.length === 0) throw new Error(`${context}.samples must not be empty`);
  if (samples.some((sample, index) => index > 0 && sample < (samples[index - 1] ?? sample))) {
    throw new Error(`${context}.samples must be sorted`);
  }
  const samplesCount = requirePositiveInteger(stats.samplesCount, `${context}.samplesCount`);
  if (samplesCount !== samples.length) {
    throw new Error(`${context}.samplesCount must equal samples.length`);
  }

  return {
    samples,
    p50: requirePositiveNumber(stats.p50, `${context}.p50`),
    p95: requirePositiveNumber(stats.p95, `${context}.p95`),
    rme: requireNonNegativeNumber(stats.rme, `${context}.rme`),
    samplesCount,
    opsPerSec: requirePositiveNumber(stats.opsPerSec, `${context}.opsPerSec`),
  };
}

function parsePaired(value: unknown, context: string): FixturePairedStats {
  const paired = requireRecord(value, context);
  return {
    base: requireString(paired.base, `${context}.base`),
    candidate: requireString(paired.candidate, `${context}.candidate`),
    ratios: requireArray(paired.ratios, `${context}.ratios`).map((ratio, index) =>
      requirePositiveNumber(ratio, `${context}.ratios[${index}]`)
    ),
    confidence: parseInterval(paired.confidence, `${context}.confidence`),
  };
}

function parseInterval(value: unknown, context: string): BootstrapInterval {
  const interval = requireRecord(value, context);
  return {
    point: requirePositiveNumber(interval.point, `${context}.point`),
    lower: requirePositiveNumber(interval.lower, `${context}.lower`),
    upper: requirePositiveNumber(interval.upper, `${context}.upper`),
  };
}

function requireRecord(value: unknown, context: string): JsonRecord {
  if (!isRecord(value)) {
    throw new Error(`${context} must be an object`);
  }
  return value;
}

function isRecord(value: unknown): value is JsonRecord {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function requireArray(value: unknown, context: string): unknown[] {
  if (!Array.isArray(value)) throw new Error(`${context} must be an array`);
  return value;
}

function requireString(value: unknown, context: string): string {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${context} must be a non-empty string`);
  }
  return value;
}

function optionalString(value: unknown, context: string): string | undefined {
  if (value === undefined) return undefined;
  return requireString(value, context);
}

function requirePositiveNumber(value: unknown, context: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) {
    throw new Error(`${context} must be a positive finite number`);
  }
  return value;
}

function requireNonNegativeNumber(value: unknown, context: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
    throw new Error(`${context} must be a non-negative finite number`);
  }
  return value;
}

function requireInteger(value: unknown, context: string): number {
  if (typeof value !== 'number' || !Number.isSafeInteger(value)) {
    throw new Error(`${context} must be a safe integer`);
  }
  return value;
}

function requirePositiveInteger(value: unknown, context: string): number {
  const integer = requireInteger(value, context);
  if (integer <= 0) throw new Error(`${context} must be greater than zero`);
  return integer;
}

function requireNonNegativeInteger(value: unknown, context: string): number {
  const integer = requireInteger(value, context);
  if (integer < 0) throw new Error(`${context} must not be negative`);
  return integer;
}

function assertUnique(values: readonly string[], context: string): void {
  if (new Set(values).size !== values.length) throw new Error(`${context} must be unique`);
}
