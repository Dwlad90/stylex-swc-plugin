function transform(source, opts = {}) {
    const options = {
        filename: opts.filename,
        plugins: [
            [
                stylexPlugin,
                {
                    ...opts,
                    runtimeInjection: true
                }
            ]
        ]
    };
    const result = transformSync(source, options);
    return result;
}
describe('[optimization] Removes `styles` variable when not needed', ()=>{
    test('Keeps used styles', ()=>{
        const result = transform(`
      import stylex from 'stylex';

      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
          color: 'blue',
        }
      });
      styles;
    `);
        expect(result.code).toMatchInlineSnapshot(`
      "import _inject from "@stylexjs/stylex/lib/stylex-inject";
      var _inject2 = _inject;
      import stylex from 'stylex';
      _inject2(".xrkmrrc{background-color:red}", 3000);
      _inject2(".xju2f9n{color:blue}", 3000);
      const styles = {
        default: {
          kWkggS: "xrkmrrc",
          kMwMTN: "xju2f9n",
          $$css: true
        }
      };
      styles;"
    `);
        expect(result.metadata).toMatchInlineSnapshot(`
      {
        "stylex": [
          [
            "xrkmrrc",
            {
              "ltr": ".xrkmrrc{background-color:red}",
              "rtl": null,
            },
            3000,
          ],
          [
            "xju2f9n",
            {
              "ltr": ".xju2f9n{color:blue}",
              "rtl": null,
            },
            3000,
          ],
        ],
      }
    `);
    });
    test('Removes unused styles', ()=>{
        const result = transform(`
      import stylex from 'stylex';

      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
          color: 'blue',
        }
      });
    `);
        expect(result.code).toMatchInlineSnapshot(`
      "import _inject from "@stylexjs/stylex/lib/stylex-inject";
      var _inject2 = _inject;
      import stylex from 'stylex';
      _inject2(".xrkmrrc{background-color:red}", 3000);
      _inject2(".xju2f9n{color:blue}", 3000);"
    `);
        expect(result.metadata).toMatchInlineSnapshot(`
      {
        "stylex": [
          [
            "xrkmrrc",
            {
              "ltr": ".xrkmrrc{background-color:red}",
              "rtl": null,
            },
            3000,
          ],
          [
            "xju2f9n",
            {
              "ltr": ".xju2f9n{color:blue}",
              "rtl": null,
            },
            3000,
          ],
        ],
      }
    `);
    });
});
