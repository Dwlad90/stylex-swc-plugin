module.exports = {
  'scripts/**/*.{j,t}s?(x)': ['eslint --color --fix'],
  '*.sh': ['shellcheck -x'],
  '*.{json,jsonc}': ['eslint --color --fix'],
};
