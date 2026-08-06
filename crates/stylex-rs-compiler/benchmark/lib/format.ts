/**
 * Single latency formatter shared by every benchmark entry point.
 *
 * Previously `bench.ts` and `bench-compare.ts` each carried their own copy
 * with slightly different unit rules (ns/µs/ms vs µs/ms/s). Keeping one
 * formatter avoids drift between the human-readable outputs.
 */

export function formatLatency(milliseconds: number): string {
  if (!Number.isFinite(milliseconds)) return 'n/a';

  const nanoseconds = milliseconds * 1_000_000;

  if (nanoseconds >= 1_000_000_000) {
    return `${(nanoseconds / 1_000_000_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} s`;
  }

  if (nanoseconds >= 1_000_000) {
    return `${(nanoseconds / 1_000_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} ms`;
  }

  if (nanoseconds >= 1_000) {
    return `${(nanoseconds / 1_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} µs`;
  }

  return `${nanoseconds.toLocaleString('en-US', {
    maximumFractionDigits: 0,
  })} ns`;
}
