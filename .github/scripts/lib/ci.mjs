/**
 * Small helpers shared by release-time CI scripts. Kept as .mjs alongside the
 * scripts that need them; introducing a package boundary just for two
 * functions would be worse than the duplication it removes.
 */

export function requireEnv(name) {
  const value = process.env[name];
  if (!value) fail(`Environment variable ${name} is required`);
  return value;
}

export function fail(message) {
  console.error(message);
  process.exit(1);
}

export function failWithErrors(header, errors) {
  console.error(header);
  for (const message of errors) console.error(`  - ${message}`);
  process.exit(1);
}
