import {
  assertUnique,
  optionalString,
  requireArray,
  requireInteger,
  requireNonNegativeInteger,
  requireNonNegativeNumber,
  requirePositiveInteger,
  requirePositiveNumber,
  requireRecord,
  requireString,
} from './json.js';
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

export interface ParseRawStatsOptions {
  /**
   * `pair` (default) is what the verdict engine needs: exactly one base
   * and one candidate. `any` also accepts the single-subject files written
   * by the historical `bench` run, which the budget check consumes.
   */
  subjects?: 'pair' | 'any';
}

export function parseRawStats(
  value: unknown,
  source: string,
  options: ParseRawStatsOptions = {}
): RawStatsFile {
  const file = requireRecord(value, source);
  if (file.schemaVersion !== RAW_STATS_SCHEMA_VERSION) {
    throw new Error(
      `${source} schemaVersion ${String(file.schemaVersion)} is not supported ` +
        `(expected ${String(RAW_STATS_SCHEMA_VERSION)})`
    );
  }

  const subjects = requireArray(file.subjects, `${source}.subjects`).map((subject, index) =>
    parseSubject(subject, `${source}.subjects[${index}]`)
  );
  if (options.subjects === 'any') {
    if (subjects.length === 0) throw new Error(`${source} must expose at least one subject`);
  } else if (subjects.length !== 2) {
    throw new Error(`${source} must expose exactly two subjects`);
  }
  assertUnique(
    subjects.map(subject => subject.label),
    `${source} subject labels`
  );

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

/**
 * Whether `label` measured every round of `fixture`.
 *
 * A paired run can hold a fixture only one of its subjects measured, because
 * the release leg compares against a published version that is behind by whole
 * features. Both readers of the file ask this question. The verdict engine asks
 * it to find the fixtures it can take a ratio for, and the budget asks it to
 * find the ones its ceilings describe. One answer, so the two cannot disagree.
 */
export function measuredBy(fixture: FixtureRawStats, label: string): boolean {
  return fixture.rounds.every(round => round.perSubject[label] !== undefined);
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
  const runnerImageVersion = optionalString(
    environment.runnerImageVersion,
    `${context}.runnerImageVersion`
  );

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
    ...(runnerImageVersion === undefined ? {} : { runnerImageVersion }),
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
  // A fixture a subject refused is measured by the subjects that answered, and
  // by those same subjects in every round of it. Rounds that disagree describe
  // a fixture half of which was measured against something else, and no reader
  // of this file could tell which half.
  const firstOrder = [...rounds[0]!.subjectOrder].toSorted();
  for (const round of rounds) {
    const order = [...round.subjectOrder].toSorted();
    if (
      order.length !== firstOrder.length ||
      order.some((label, index) => label !== firstOrder[index])
    ) {
      throw new Error(`${context}.rounds must name the same subjects in every round`);
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
  // A subset rather than the whole set: the release leg compares against the
  // last published version, and a fixture that prices a feature that version
  // does not carry is measured by the candidate alone. What the round may not
  // do is name a subject the file does not declare, or name one two times.
  const declared = new Set(subjects.map(subject => subject.label));
  if (subjectOrder.length === 0) {
    throw new Error(`${context}.subjectOrder must name at least one subject`);
  }
  if (new Set(subjectOrder).size !== subjectOrder.length) {
    throw new Error(`${context}.subjectOrder must name each subject at most one time`);
  }
  for (const label of subjectOrder) {
    if (!declared.has(label)) {
      throw new Error(
        `${context}.subjectOrder names ${JSON.stringify(label)}, which is not a subject of this file`
      );
    }
  }

  const rawPerSubject = requireRecord(round.perSubject, `${context}.perSubject`);
  const perSubject: Record<string, RawLatencySamples> = {};
  for (const label of subjectOrder) {
    perSubject[label] = parseSamples(
      rawPerSubject[label],
      `${context}.perSubject[${JSON.stringify(label)}]`
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

/**
 * Roles are required; the bootstrap statistics are optional and must be
 * present or absent together. A release run records roles alone -- see
 * `computePairedStats` in runner.ts.
 */
function parsePaired(value: unknown, context: string): FixturePairedStats {
  const paired = requireRecord(value, context);
  const roles = {
    base: requireString(paired.base, `${context}.base`),
    candidate: requireString(paired.candidate, `${context}.candidate`),
  };

  if (paired.ratios === undefined && paired.confidence === undefined) return roles;
  if (paired.ratios === undefined || paired.confidence === undefined) {
    throw new Error(`${context} must declare ratios and confidence together, or neither`);
  }

  return {
    ...roles,
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
