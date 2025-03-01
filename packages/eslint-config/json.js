module.exports = {
  extends: ['plugin:jsonc/recommended-with-json'],
  parser: 'jsonc-eslint-parser',
  overrides: [
    {
      files: [
        '**/.eslintrc.json',
        '**/tsconfig.*.json',
        '**/tsconfig.json',
      ],
      rules: {
        'jsonc/no-comments': 'off',
        'max-len': 'off',
      },
    },
  ],
};
