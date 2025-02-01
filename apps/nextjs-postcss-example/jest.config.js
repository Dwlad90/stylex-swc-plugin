
const path = require("path");

const rootDir = __dirname;

const customJestConfig = {
  setupFilesAfterEnv: ["<rootDir>/jest.setup.js"],
  testEnvironment: "jsdom",
  transform: {
    "^.+\\.(ts|tsx|js|jsx|mjs|cjs|html)$": [
      'jest-chain-transform',
      {
        transformers: [
          ["@stylexswc/jest", {
            rsOptions: {
              aliases: {
                '@/*': [path.join(rootDir, '*')],
              },
              unstable_moduleResolution: {
                type: 'commonJS',
              },
            }
          }], ['@swc/jest', {
            "$schema": "https://json.schemastore.org/swcrc",
            "jsc": {
              "parser": {
                "syntax": "typescript",
                "tsx": true,
                "dynamicImport": true,
                "decorators": true,
                "dts": true
              },
              "transform": {
                "react": {
                  "useBuiltins": true,
                  "runtime": "automatic"
                }
              },
              "target": "esnext",
              "loose": false,
              "externalHelpers": false,
              "keepClassNames": true,
              "baseUrl": "./",

              "paths": {
                "@/*": ["./*"]
              }
            },
            "module": {
              "type": "es6"
            },
            "minify": false,
          }
          ]
        ]
      }
    ],
  },
};

module.exports = customJestConfig
