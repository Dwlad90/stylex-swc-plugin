/**
 * `sx` prop harness: reproduce both reported failures at HEAD, and compare
 * every `sx` case with the reference implementation.
 *
 * Run it from a worktree root, or name the worktree to measure:
 *
 *   node .scratch/fix_sx-prop-is-skipped-for-compiled-jsx-shorthand/sx-parity.mjs
 *   node sx-parity.mjs /path/to/worktree
 *
 * The harness stays in the tracker directory, which has no installed modules
 * of its own, so it resolves every dependency through the Vite plugin
 * package, which already carries the compiler, the reference plugin and the
 * JSX-compiling tool the reporter used.
 *
 * Three parts:
 *
 *   1. Reproduction. A compiled call written with the object shorthand comes
 *      back still carrying a native prop, and a real bundler plugin refuses a
 *      module that uses the prop and imports nothing — a module the compiler
 *      handles correctly when it is called directly.
 *
 *   2. Raw markup parity. Same source, same configured prop name. Both
 *      outputs are re-printed by one printer and must then be identical.
 *
 *   3. Compiled-call parity, indirectly. The reference implementation has no
 *      compiled-call path and returns such input unchanged, so comparing the
 *      two outputs for compiled input would assert the defect. Each compiled
 *      case is instead paired with the raw markup it was compiled from, and
 *      the check is that our compiled output carries the same props call, the
 *      same resolved binding, the same injected import and no leftover native
 *      prop, as the reference output for that markup. Only the call shape may
 *      differ. Two cases are marked as controls: see `CASES`.
 *
 * Exit code is 0 when every comparison holds, 1 otherwise. At HEAD the
 * compiled cases that use the shorthand are expected to differ; that is what
 * part 1 reports.
 */

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';

/** The package under test whose installed modules this harness borrows. */
const HOST_PACKAGE = 'packages/unplugin/package.json';

/**
 * Find the worktree to measure: the first argument, or the working directory.
 *
 * The tracker directory is a symlink shared by every worktree, so the path of
 * this file names no particular one and cannot be used. The working directory
 * does name one, which is also how a maintainer measures a second worktree —
 * by running this same file from there.
 */
function findWorktree() {
  const start = path.resolve(process.argv[2] ?? process.cwd());
  let directory = start;

  for (;;) {
    if (fs.existsSync(path.join(directory, HOST_PACKAGE))) {
      return directory;
    }

    const parent = path.dirname(directory);

    if (parent === directory) {
      throw new Error(
        `No ${HOST_PACKAGE} above ${start}.\n` +
          'This harness measures a worktree, and the tracker directory it lives in is\n' +
          'shared by all of them. Run it from a worktree root, or name one:\n' +
          '  node .scratch/<tracker>/sx-parity.mjs\n' +
          '  node sx-parity.mjs /path/to/worktree'
      );
    }

    directory = parent;
  }
}

const worktree = findWorktree();
const require = createRequire(path.join(worktree, HOST_PACKAGE));

/** The commit the measured build was made from. */
const headCommit = execFileSync('git', ['-C', worktree, 'rev-parse', '--short', 'HEAD'], {
  encoding: 'utf8',
}).trim();

/** The sources behind each built artefact this harness measures. */
const crateSources = path.join(worktree, 'crates');
const pluginSources = path.join(worktree, 'packages/unplugin/src');

/**
 * The compiled addon itself, not the JavaScript wrapper beside it. The
 * wrapper is rewritten by a TypeScript build that a Rust change does not run,
 * so its date says nothing about which Rust sources are being measured.
 */
const compilerDist = path.join(worktree, 'crates/stylex-rs-compiler/dist');
const nativeAddon = path.join(
  compilerDist,
  fs.readdirSync(compilerDist).find(entry => entry.endsWith('.node'))
);

const { transform: rustTransform } = require('@stylexswc/rs-compiler');
const babel = require('@babel/core');
const esbuild = require('esbuild');

const referencePluginModule = require('@stylexjs/babel-plugin');
const referencePlugin = referencePluginModule.default ?? referencePluginModule;

// The Vite plugin is loaded as an ES module: its build is ESM only.
const unpluginModule = await import(require.resolve('@stylexswc/unplugin'));
const unpluginFactory = unpluginModule.unpluginFactory ?? unpluginModule.default?.unpluginFactory;

/** The file name both compilers are told they are compiling. */
const FILENAME = '/js/App.jsx';

/**
 * The import sources both compilers recognise by default. An import of one of
 * them is a fact a comparison looks at, so the list is named here rather than
 * matched by shape.
 */
const IMPORT_SOURCES = ['@stylexjs/stylex', 'stylex'];

/**
 * The options both compilers are handed before a case adds its own.
 *
 * `haste` resolution keeps both from needing a real module layout beside the
 * fixture, and `dev: false` keeps debug class names — which encode a file
 * path — out of the comparison.
 */
const BASE_OPTIONS = {
  dev: false,
  unstable_moduleResolution: { type: 'haste', rootDir: '/js' },
};

/* ------------------------------------------------------------------ cases */

/**
 * One raw-markup case.
 *
 * `compile: true` marks a case that is also run through the JSX-compiling
 * tool and compared indirectly in part 3.
 *
 * `control: true` marks a case whose correct outcome is that nothing happens.
 * Both compilers leave the prop where it is, so the case agrees whether the
 * compiler handles the prop correctly or ignores it altogether. It catches a
 * fix that does too much, never one that does too little, and part 3 says so
 * rather than letting it read as proof.
 *
 * `reshape` names a further rewrite of the compiled output, for a call shape
 * the JSX-compiling tool does not emit but another toolchain does. It is a
 * rewrite of generated code, not hand-written input.
 */
const CASES = [
  {
    name: 'forwarded prop',
    why: 'the reported shape: a leaf component that only forwards the prop',
    compile: true,
    source: `import * as stylex from '@stylexjs/stylex';
export function Leaf({ sx }) {
  return <div sx={sx}>Hello</div>;
}
`,
  },
  {
    name: 'static style',
    why: 'the prop resolves at compile time, so no runtime call survives',
    compile: true,
    source: `import * as stylex from '@stylexjs/stylex';
const styles = stylex.create({ main: { color: 'red' } });
export function App() {
  return <div sx={styles.main}>Hello</div>;
}
`,
  },
  {
    name: 'array value',
    why: 'a list of styles, one of them only known at runtime',
    compile: true,
    source: `import * as stylex from '@stylexjs/stylex';
const styles = stylex.create({ a: { color: 'red' }, b: { backgroundColor: 'blue' } });
export function App({ extra }) {
  return <div sx={[styles.a, styles.b, extra]}>Hello</div>;
}
`,
  },
  {
    name: 'renamed prop',
    why: 'both halves of the fix must follow the configured name',
    compile: true,
    options: { sxPropName: 'css' },
    source: `import * as stylex from '@stylexjs/stylex';
export function Leaf({ css }) {
  return <div css={css}>Hello</div>;
}
`,
  },
  {
    name: 'computed key',
    why: 'a toolchain that emits a statically known computed key',
    compile: true,
    reshape: toComputedKey,
    source: `import * as stylex from '@stylexjs/stylex';
export function Leaf({ sx }) {
  return <div sx={sx}>Hello</div>;
}
`,
  },
  {
    name: 'no import',
    why: 'the compiler handles it; the plugins are what drop it',
    compile: false,
    source: `export function Leaf({ sx }) {
  return <div sx={sx}>Hello</div>;
}
`,
  },
  {
    name: 'prop disabled',
    why: 'with the feature off the prop must be left alone',
    compile: true,
    control: true,
    options: { sxPropName: false },
    source: `import * as stylex from '@stylexjs/stylex';
export function Leaf({ sx }) {
  return <div sx={sx}>Hello</div>;
}
`,
  },
  {
    name: 'component element',
    why: 'only a host element carries the prop',
    compile: true,
    control: true,
    source: `import * as stylex from '@stylexjs/stylex';
const styles = stylex.create({ main: { color: 'red' } });
export function App() {
  return <MyComponent sx={styles.main}>Hello</MyComponent>;
}
`,
  },
];

/* ------------------------------------------------------------- compilation */

function optionsFor(testCase) {
  return { ...BASE_OPTIONS, ...testCase.options };
}

function runRust(source, options) {
  return rustTransform(FILENAME, source, options).code;
}

async function runReference(source, options) {
  const result = await babel.transformAsync(source, {
    filename: FILENAME,
    babelrc: false,
    configFile: false,
    parserOpts: { plugins: ['jsx'] },
    plugins: [referencePlugin.withOptions(options)],
  });

  return result.code;
}

/**
 * Run one module through a real bundler plugin, rather than through the
 * predicate it asks. The reported failure is the plugin dropping a module
 * before the compiler is reached, so the plugin is what has to be seen doing
 * it.
 *
 * Returns the transformed code, or `null` when the plugin skipped the module.
 */
async function runPlugin(source, options) {
  const plugin = unpluginFactory({ rsOptions: options }, { framework: 'rollup', versions: {} });
  const instances = Array.isArray(plugin) ? plugin : [plugin];

  if (instances.length !== 1) {
    throw new Error(
      `The plugin factory returned ${instances.length} instances; this harness reads one.`
    );
  }

  const [instance] = instances;

  // The hooks the plugin calls on its host. `error` must throw, the way a
  // real host's does: the plugin calls it and then returns null, so a stub
  // that swallows it would let a compiler crash read as the module being
  // skipped — which is the very thing being measured here.
  const context = {
    addWatchFile() {},
    emitFile: () => '',
    getWatchFiles: () => [],
    parse: () => ({}),
    error(problem) {
      throw problem instanceof Error ? problem : new Error(String(problem?.message ?? problem));
    },
    warn() {},
  };

  await instance.buildStart.call(context);

  // A real build asks this first, so a module dropped here never reaches the
  // transform hook. Asking only the transform hook would report a module as
  // handled that a build still drops.
  if (!instance.transformInclude.call(context, FILENAME)) {
    return null;
  }

  const result = await instance.transform.call(context, source, FILENAME);

  return result?.code ?? null;
}

/**
 * The JSX-compiling pass the reporter's build ran, with the settings that
 * produce the object shorthand: an identifier whose name is the prop name
 * collapses to `{ sx }`.
 */
async function compileJsx(source) {
  const { code } = await esbuild.transform(source, {
    loader: 'jsx',
    jsx: 'automatic',
    format: 'esm',
  });

  return code;
}

/**
 * Rewrite every property that names the prop into a computed key holding the
 * same text: `{ sx: value }` and `{ sx }` both become `{ ["sx"]: value }`.
 *
 * Read off the compiled output rather than written by hand, so the case
 * stays a statement about a real call the compiler is handed.
 */
function toComputedKey(code, propName) {
  const { types } = babel;

  const rewrite = {
    visitor: {
      ObjectProperty(nodePath) {
        // A destructured parameter is a pattern, not an element's props, and
        // a computed key there would not even parse the same way.
        if (nodePath.parentPath.isObjectPattern() || nodePath.node.computed) {
          return;
        }

        if (keyName(nodePath.node) !== propName) {
          return;
        }

        nodePath.node.computed = true;
        nodePath.node.shorthand = false;
        nodePath.node.key = types.stringLiteral(propName);
      },
    },
  };

  const result = babel.transformSync(code, {
    babelrc: false,
    configFile: false,
    parserOpts: { plugins: ['jsx'] },
    plugins: [rewrite],
  });

  return result.code;
}

/* -------------------------------------------------------------- comparison */

/**
 * Re-print code with one printer, so that a difference in line breaks is not
 * read as a difference in output. Both compilers are free to format; only
 * what they emit is being compared.
 */
function normalize(code) {
  const result = babel.transformSync(code, {
    babelrc: false,
    configFile: false,
    // Comments are kept. `/* @__PURE__ */` decides whether a bundler may drop
    // a call, so a divergence in it is a real one, not formatting.
    comments: true,
    parserOpts: { plugins: ['jsx'] },
    plugins: [],
  });

  return result.code;
}

/**
 * Re-print one expression, so that an argument compared across two call
 * shapes is compared by what it is rather than by how it was wrapped. The
 * two compilers indent a list differently, and each indents it differently
 * again inside a compiled call.
 */
function printExpression(text) {
  // Printed as a statement, so the trailing semicolon comes back off. The
  // wrapping parentheses the printer needs to accept it are dropped by the
  // printer itself, unless the expression needs them.
  return normalize(`(${text})`).replace(/;$/, '');
}

/** The name of a JSX attribute or an object property, when it has a plain one. */
function keyName(node) {
  const key = node.type === 'JSXAttribute' ? node.name : node.key;

  if (node.type !== 'JSXAttribute' && node.computed) {
    // A computed key is only a name when its text is known at compile time,
    // which is one of the forms this fix is about. A template literal with
    // no expressions counts; anything else does not.
    if (key.type === 'StringLiteral') {
      return key.value;
    }

    if (key.type === 'TemplateLiteral' && key.expressions.length === 0) {
      return key.quasis[0]?.value.cooked;
    }

    return undefined;
  }

  switch (key.type) {
    case 'Identifier':
    case 'JSXIdentifier':
      return key.name;
    case 'StringLiteral':
      return key.value;
    default:
      return undefined;
  }
}

/**
 * What a JSX attribute or an object property is set to, as text. A method has
 * no value expression, which is one of the forms this fix deliberately leaves
 * alone, so it is named rather than printed.
 */
function valueText(node, code) {
  const { value } = node;

  if (!value) {
    return '(no value expression)';
  }

  return value.type === 'StringLiteral'
    ? value.value
    : printExpression(code.slice(value.start, value.end));
}

/**
 * What a comparison of a compiled call against raw markup is allowed to look
 * at: the parts that must survive the change of call shape.
 *
 * Every fact here is read off an element's props — a JSX attribute, or a
 * property of a props object — rather than off the file. Reading the file
 * would collect the class names inside the `$$css` style objects that
 * `stylex.create` emits either way, and those agree whether or not the prop
 * was handled at all, which would make a case pass while asserting nothing.
 *
 * `nativeProps` is the defining symptom of the report: a prop the compiler
 * failed to consume is still sitting on the element.
 */
function styleFacts(code, options) {
  const ast = babel.parseSync(code, {
    babelrc: false,
    configFile: false,
    parserOpts: { plugins: ['jsx'] },
    filename: FILENAME,
  });

  const sources = options.importSources ?? IMPORT_SOURCES;

  // Watched even when the prop is turned off, because then the fact to check
  // is that the prop survived untouched.
  const propName = typeof options.sxPropName === 'string' ? options.sxPropName : 'sx';

  const imports = [];
  const propsCalls = [];
  const classNames = [];
  const nativeProps = [];

  function readElementProp(node) {
    const name = keyName(node);

    if (name === propName) {
      nativeProps.push(name);

      return;
    }

    if (name === 'className') {
      classNames.push(valueText(node, code));
    }
  }

  babel.traverse(ast, {
    ImportDeclaration(nodePath) {
      const source = nodePath.node.source.value;

      if (sources.includes(source)) {
        const locals = nodePath.node.specifiers.map(specifier => specifier.local.name);

        imports.push(`${source}:${locals.join(',')}`);
      }
    },
    CallExpression(nodePath) {
      const { callee } = nodePath.node;

      if (
        callee.type === 'MemberExpression' &&
        callee.property.type === 'Identifier' &&
        callee.property.name === 'props' &&
        callee.object.type === 'Identifier'
      ) {
        const { node } = nodePath;

        // Printed whole rather than argument by argument, so that a spread
        // argument is printed as the spread it is.
        propsCalls.push(printExpression(code.slice(node.start, node.end)));
      }
    },
    JSXAttribute(nodePath) {
      readElementProp(nodePath.node);
    },
    ObjectProperty(nodePath) {
      // `function Leaf({ sx })` destructures a parameter; it names no
      // element. Every case here forwards a prop of that name, so counting
      // the pattern would put the prop in the facts whatever the compiler did
      // with the element.
      if (nodePath.parentPath.isObjectPattern()) {
        return;
      }

      readElementProp(nodePath.node);
    },
    // A getter, a setter or a method named like the prop is still the prop
    // sitting on the element, so it is watched the same way.
    ObjectMethod(nodePath) {
      readElementProp(nodePath.node);
    },
  });

  return { imports, propsCalls, classNames, nativeProps };
}

function factsAgree(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

/* ------------------------------------------------------------------ report */

const ESC = '\u001b';
const GREEN = `${ESC}[32m`;
const RED = `${ESC}[31m`;
const DIM = `${ESC}[2m`;
const BOLD = `${ESC}[1m`;
const RESET = `${ESC}[0m`;

let failures = 0;

function heading(text) {
  console.log(`\n${BOLD}${text}${RESET}\n${'='.repeat(text.length)}`);
}

function verdict(ok, label) {
  if (!ok) {
    failures += 1;
  }

  console.log(`  ${ok ? `${GREEN}agree ` : `${RED}DIFFER`}${RESET}  ${label}`);
}

function block(label, text) {
  console.log(`${DIM}--- ${label} ---${RESET}`);
  console.log(text.trimEnd());
}

/* -------------------------------------------------------------------- main */

console.log(`${BOLD}sx prop parity harness${RESET}`);
/**
 * Name a built artefact and say how old it is relative to the sources it was
 * built from.
 *
 * "At HEAD" is a claim about build output, not about the checkout, and a
 * compiled addon keeps its own file date even when a build produced nothing.
 * A report that does not say which build it measured cannot be trusted to be
 * measuring this commit.
 */
function artefactLine(label, artefactPath, sourceDir, sourceExtensions) {
  const built = fs.statSync(artefactPath).mtime;
  const newestSource = newestSourceUnder(sourceDir, sourceExtensions);
  const stale = newestSource > built;

  return (
    `  ${label.padEnd(13)} ${path.relative(worktree, artefactPath)}\n` +
    `  ${''.padEnd(13)} built ${built.toISOString()}, newest source ${newestSource.toISOString()}` +
    `${stale ? `\n  ${''.padEnd(13)} ${RED}OLDER THAN ITS SOURCES — REBUILD${RESET}` : ''}`
  );
}

/**
 * The most recent change to a source file under a directory.
 *
 * Only the named extensions count. A build writes its own output back under
 * these trees, so counting every file would make an artefact look older than
 * itself the moment it was built.
 */
function newestSourceUnder(directory, extensions) {
  let newest = new Date(0);

  for (const entry of fs.readdirSync(directory, { withFileTypes: true, recursive: true })) {
    if (!entry.isFile() || !extensions.includes(path.extname(entry.name))) {
      continue;
    }

    const changed = fs.statSync(path.join(entry.parentPath, entry.name)).mtime;

    if (changed > newest) {
      newest = changed;
    }
  }

  return newest;
}

console.log(`  commit        ${headCommit}`);
console.log(artefactLine('compiler', nativeAddon, crateSources, ['.rs', '.toml']));
console.log(artefactLine('vite plugin', require.resolve('@stylexswc/unplugin'), pluginSources, [
  '.ts',
]));
console.log(`  reference     ${require.resolve('@stylexjs/babel-plugin')}`);
console.log(`  jsx compiler  esbuild ${esbuild.version}`);

heading('1. The two reported failures, at HEAD');

const reported = CASES.find(testCase => testCase.name === 'forwarded prop');
const reportedCompiled = await compileJsx(reported.source);
const reportedOutput = runRust(reportedCompiled, optionsFor(reported));

block('compiled input', reportedCompiled);
block('compiler output', reportedOutput);

const shorthandUntransformed = styleFacts(reportedOutput, optionsFor(reported)).nativeProps.length > 0;

console.log(
  `  ${shorthandUntransformed ? `${RED}REPRODUCED` : `${GREEN}fixed     `}${RESET}  ` +
    'a compiled call written with the object shorthand keeps a native prop'
);

const noImport = CASES.find(testCase => testCase.name === 'no import');
const withImport = CASES.find(testCase => testCase.name === 'forwarded prop');
const pluginOutput = await runPlugin(noImport.source, optionsFor(noImport));
const compilerOutput = runRust(noImport.source, optionsFor(noImport));

// The same module with an import it does not otherwise need. It isolates the
// import as the only reason the module above was dropped: without this, a
// plugin broken in some unrelated way would read as the reported defect.
const controlOutput = await runPlugin(withImport.source, optionsFor(withImport));

block('module that uses the prop and imports nothing', noImport.source);
block('the plugin', pluginOutput ?? '(skipped — the compiler was never reached)');
block('the compiler, called directly', compilerOutput);
block('the plugin, on the same module with an import added', controlOutput ?? '(skipped)');

const moduleRefused = pluginOutput === null;
const compilerStylesIt = styleFacts(compilerOutput, optionsFor(noImport)).propsCalls.length > 0;
const pluginWorksOtherwise =
  controlOutput !== null && styleFacts(controlOutput, optionsFor(withImport)).propsCalls.length > 0;

console.log(
  `  ${moduleRefused && compilerStylesIt && pluginWorksOtherwise ? `${RED}REPRODUCED` : `${GREEN}fixed     `}${RESET}  ` +
    'the plugin refuses a module the compiler handles when called directly'
);

heading('2. Raw markup: outputs must be identical');

for (const testCase of CASES) {
  const options = optionsFor(testCase);
  const ours = normalize(runRust(testCase.source, options));
  const theirs = normalize(await runReference(testCase.source, options));

  console.log(`\n${BOLD}${testCase.name}${RESET} ${DIM}— ${testCase.why}${RESET}`);
  block('ours', ours);
  block('reference', theirs);
  verdict(ours === theirs, testCase.name);
}

heading('3. Compiled calls: paired with the markup they came from');

for (const testCase of CASES.filter(entry => entry.compile)) {
  const options = optionsFor(testCase);
  const generated = await compileJsx(testCase.source);
  const compiled = testCase.reshape
    ? testCase.reshape(generated, typeof options.sxPropName === 'string' ? options.sxPropName : 'sx')
    : generated;
  const ours = runRust(compiled, options);
  const reference = await runReference(testCase.source, options);

  const ourFacts = styleFacts(ours, options);
  const referenceFacts = styleFacts(reference, options);

  console.log(`\n${BOLD}${testCase.name}${RESET} ${DIM}— ${testCase.why}${RESET}`);
  block('compiled input', compiled);
  block('our output for the compiled input', ours);
  block('reference output for the paired markup', reference);
  console.log(`${DIM}  ours      ${JSON.stringify(ourFacts)}${RESET}`);
  console.log(`${DIM}  reference ${JSON.stringify(referenceFacts)}${RESET}`);

  verdict(
    factsAgree(ourFacts, referenceFacts),
    `${testCase.name} (compiled)${testCase.control ? ' — control: agrees either way' : ''}`
  );
}

heading('Result');

if (failures === 0) {
  console.log(`${GREEN}every comparison holds${RESET}`);
} else {
  console.log(`${RED}${failures} comparison(s) differ${RESET}`);
}

process.exitCode = failures === 0 ? 0 : 1;
