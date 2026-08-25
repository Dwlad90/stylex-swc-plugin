// The reference oracle: run one module through @stylexjs/babel-plugin and hand
// back every @media query text it emitted, in emission order.
const { createRequire } = require('node:module');
const path = require('node:path');

const REPO = '/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/bugfix';
const PKG = path.join(REPO, 'crates/stylex-rs-compiler');
const req = createRequire(path.join(PKG, 'parity/oracle.cjs'));

const babel = req('@babel/core');
const pluginModule = req('@stylexjs/babel-plugin');
const plugin = pluginModule.default ?? pluginModule;

// The same filename and options the parity harness hands both compilers, so a
// row here is comparable with a row there.
const FILENAME = path.join(PKG, 'parity/__fixture__/value.js');
const OPTIONS = { dev: false, unstable_moduleResolution: { type: 'haste', rootDir: PKG } };

function run(code, extraOptions = {}) {
  const result = babel.transformSync(code, {
    filename: FILENAME,
    babelrc: false,
    configFile: false,
    parserOpts: { sourceType: 'module', plugins: ['jsx'] },
    plugins: [[plugin, { ...OPTIONS, ...extraOptions }]],
  });
  return { rules: result?.metadata?.stylex ?? [], code: result?.code ?? '' };
}

/** Every `@media ...` prelude in one rule's CSS text, outermost first. */
function mediaPreludes(css) {
  const out = [];
  const re = /@media[^{]*/g;
  let m;
  while ((m = re.exec(css)) !== null) out.push(m[0].trim());
  return out;
}

// This compiler, loaded from `dist/` rather than from the Rust sources -- the
// same artifact the parity harness measures.
const rust = req(path.join(PKG, 'dist/index.js'));

function runRust(code, extraOptions = {}) {
  const result = rust.transform(FILENAME, code, { ...OPTIONS, ...extraOptions });
  return { rules: result.metadata.stylex, code: result.code };
}

module.exports = { run, runRust, mediaPreludes, req, babel, FILENAME, OPTIONS, PKG, REPO };
