var { transform } = require('./index')

console.log(
  JSON.stringify(
    transform(`
  import stylex from '@stylexjs/stylex';
  export const styles = stylex.create({
      default: {
          backgroundColor: 'red',
          color: 'blue',
      }
  });
  `),
    null,
    2,
  ),
)

/**
 * Output:
  {
    "code": "import stylex from '@stylexjs/stylex';\nexport const styles = {\n    default: {\n        backgroundColor: \"xrkmrrc\",\n        color: \"xju2f9n\",\n        $$css: true\n    }\n};\n",
    "metadata": {
      "stylex": {
        "styles": [
          {
            "className": "xrkmrrc",
            "style": {
              "ltr": ".xrkmrrc{background-color:red}",
              "rtl": null
            },
            "priority": 3000
          },
          {
            "className": "xju2f9n",
            "style": {
              "ltr": ".xju2f9n{color:blue}",
              "rtl": null
            },
            "priority": 3000
          }
        ]
      }
    },
    "sourcemap": "{\"version\":3,\"sources\":[\"<anon>\"],\"names\":[],\"mappings\":\"AACE;AACA;;;;;;EAKG\"}"
  }
 */
