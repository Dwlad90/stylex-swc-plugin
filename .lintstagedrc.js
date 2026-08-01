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
    '*.{json,jsonc,md,mdx,yml,yaml,css,html,vue}': ['oxfmt --no-error-on-unmatched-pattern'],
    // Syncpack owns manifest ordering, which is why `sortPackageJson` is
    // disabled in `.oxfmtrc.json`. Kept as its own entry, sequenced rather
    // than folded into the pattern above, so the two never race on one file.
    'package.json': [
      `syncpack format --config "${syncpackConfig}" --source`,
      'oxfmt --no-error-on-unmatched-pattern',
    ],
    '*.sh': ['shellcheck -x'],
  };
})();
