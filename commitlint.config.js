const { types } = require('conventional-commit-types');

module.exports = {
  rules: {
    'body-leading-blank': [2, 'always',],
    'footer-leading-blank': [2, 'always',],
    'header-max-length': [2, 'always', 100,],
    'subject-empty': [2, 'never',],
    'type-enum': [2, 'always', Object.keys(types).map(type => type),],
    'type-case': [2, 'always', 'lower-case',],
    'type-empty': [2, 'never',],
  },
};
