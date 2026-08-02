// Single root configuration. There used to be 55 copies of this file, 54 of
// them symlinks to this one, which lint-staged resolved per staged file and
// which therefore all behaved identically.
module.exports = (() => {
  const { execSync } = require('child_process');
  const root = execSync('git rev-parse --show-toplevel').toString().trim();
  const syncpackConfig = `${root}/.syncpackrc`;

  return {
    // Lint first so that autofixes are then formatted, not the reverse.
    '*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}': [
      'oxlint --fix --no-error-on-unmatched-pattern',
      'oxfmt --no-error-on-unmatched-pattern',
    ],
    // `!(package)` is load-bearing. lint-staged runs the task lists of
    // *different* patterns concurrently, so while `package.json` also matched
    // this group, `oxfmt` here raced `syncpack format` below on the same file.
    // Excluding it is what actually serialises the two; giving the manifest its
    // own entry never did, it only guaranteed a second concurrent writer.
    '!(package).{json,jsonc,md,mdx,yml,yaml,css,html,vue}': [
      'oxfmt --no-error-on-unmatched-pattern',
    ],
    // Syncpack owns manifest ordering, which is why `sortPackageJson` is
    // disabled in `.oxfmtrc.json`.
    //
    // A function rather than a string list: lint-staged appends the staged
    // paths to a string command verbatim, but `syncpack --source` takes one
    // glob per occurrence. Staging two manifests therefore fed the second in as
    // a bare argument and syncpack rejected it — so this hook failed on exactly
    // the commits that touch more than one manifest. Expanding the paths into
    // repeated `--source` flags is the fix; returning a function is what makes
    // that possible, and it means the file list must be appended by hand here.
    'package.json': files => [
      [
        'syncpack format',
        `--config ${JSON.stringify(syncpackConfig)}`,
        ...files.map(file => `--source ${JSON.stringify(file)}`),
      ].join(' '),
      `oxfmt --no-error-on-unmatched-pattern ${files.map(f => JSON.stringify(f)).join(' ')}`,
    ],
    '*.sh': ['shellcheck -x'],
  };
})();
