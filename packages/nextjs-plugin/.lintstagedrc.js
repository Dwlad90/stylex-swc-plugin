module.exports = {
  '*.{j,t}s?(x)': ['eslint --color --fix'],
  '*.mdx': ['eslint --color --fix'],
  '*.json': 'eslint --color --fix',
  'package.json': ['syncpack format', 'eslint --color --fix'],
};
