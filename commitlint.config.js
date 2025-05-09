const fs = require('fs');
const path = require('path');

const czConfig = JSON.parse(fs.readFileSync(path.resolve(__dirname, '.czrc'), 'utf-8'));
const types = czConfig.types || {};

module.exports = {
  parserPreset: {
    parserOpts: {
      headerPattern: /^([\w-]+)(?:\(([^)]*)\))?: (.*)$/,
      headerCorrespondence: ['type', 'scope', 'subject']
    }
  },
  rules: {
    'header-max-length': [2, 'always', 120],
    'type-enum': [2, 'always', Object.keys(types)],
    'type-case': [2, 'always', 'lower-case'],
    'type-empty': [2, 'never'],
    'subject-empty': [2, 'never'],
    'body-leading-blank': [2, 'always'],
    'footer-leading-blank': [2, 'always']
  },
  ignores: [
    (commit) => commit.startsWith('Merge')
  ]
};