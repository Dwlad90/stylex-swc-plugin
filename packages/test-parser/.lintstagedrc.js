module.exports = {
  '*.rs': ['cargo clippy --fix'],
  '*.mdx': ['eslint --color --fix'],
  '*.json': 'eslint --color --fix',
  'package.json': ['syncpack format', 'eslint --color --fix'],
};
