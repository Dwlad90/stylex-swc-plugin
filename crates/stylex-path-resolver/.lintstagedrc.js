module.exports = {
  '*.mdx': ['eslint --color --fix'],
  '*.json': 'eslint --color --fix',
  'package.json': ['syncpack format --config ../../.syncpackrc --source', 'eslint --color --fix'],
};
