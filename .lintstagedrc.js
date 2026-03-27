// eslint-disable-next-line no-undef
module.exports = (() => {
  // eslint-disable-next-line @typescript-eslint/no-require-imports, no-undef
  const { execSync } = require('child_process');
  const root = execSync('git rev-parse --show-toplevel').toString().trim();
  const syncpackConfig = `${root}/.syncpackrc`;
  return {
    'scripts/**/*.{j,t}s?(x)': ['eslint --color --fix'],
    '*.sh': ['shellcheck -x'],
    '*.{j,t}s?(x)': ['eslint --color --fix'],
    '*.mdx': ['eslint --color --fix'],
    'package.json': [
      `syncpack format --config "${syncpackConfig}" --source`,
      'eslint --color --fix'
    ],
    '*.{json,jsonc}': ['eslint --color --fix'],
  };
})();
