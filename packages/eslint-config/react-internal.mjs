import { resolve } from "node:path";
import js from "@eslint/js";
import globals from "globals";
import prettierConfig from "eslint-config-prettier";
import turboConfig from "eslint-config-turbo";

const project = resolve(process.cwd(), "tsconfig.json");

/*
 * This is a custom ESLint configuration for use with
 * internal (bundled by their consumer) libraries
 * that utilize React.
 */

/** @type {import("eslint").FlatConfig[]} */
export default [
  {
    name: "react-internal:base",
    languageOptions: {
      globals: {
        ...globals.browser,
        React: true,
        JSX: true,
      },
    },
    settings: {
      react: { version: "19" },
      "import/resolver": {
        typescript: {
          project,
        },
      },
    },
    ignores: [
      ".*.js",
      "node_modules/",
      "dist/",
    ],
  },
  js.configs.recommended,
  prettierConfig,
  turboConfig,
  {
    name: "react-internal:js-ts",
    files: ["*.js?(x)", "*.ts?(x)"],
  }
];