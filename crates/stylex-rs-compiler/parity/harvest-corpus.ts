/**
 * Regenerate `parity/corpus/harvested.json` from the Rust test suites.
 *
 * Run this after adding or changing tests that carry CSS values, so the parity
 * corpus keeps covering what the suites cover. The output is checked in: the
 * harness itself must not depend on scanning Rust sources at run time.
 *
 * Usage:
 *   pnpm parity:harvest            # rewrite the corpus file
 *   pnpm parity:harvest --check    # fail if it is out of date
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import { harvestCorpus } from './lib/harvest.js';
import type { CorpusFile } from './lib/types.js';

const parityDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(parityDir, '../../..');
const outputPath = path.join(parityDir, 'corpus/harvested.json');

const { values: cliOptions } = parseArgs({
  args: process.argv.slice(2).filter(arg => arg !== '--'),
  options: { check: { type: 'boolean', default: false } },
});

const corpus: CorpusFile = {
  set: 'harvested',
  description:
    'CSS declarations extracted from the Rust test suites by parity/harvest-corpus.ts. Generated — do not edit by hand.',
  entries: harvestCorpus({ workspaceRoot }),
};

const serialized = `${JSON.stringify(corpus, null, 2)}\n`;

if (cliOptions.check) {
  const current = fs.existsSync(outputPath) ? fs.readFileSync(outputPath, 'utf8') : '';
  if (current !== serialized) {
    console.error(
      `${path.relative(workspaceRoot, outputPath)} is out of date; run \`pnpm parity:harvest\`.`
    );
    process.exit(1);
  }
  console.log(`${corpus.entries.length} declarations — corpus is up to date.`);
} else {
  fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  fs.writeFileSync(outputPath, serialized, 'utf8');
  console.log(
    `Harvested ${corpus.entries.length} declarations into ${path.relative(workspaceRoot, outputPath)}`
  );
}
