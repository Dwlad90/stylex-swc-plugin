import test from 'ava';

import { transform } from '../dist/index';

test('sync function from native code', t => {
  const fixture = `
    import stylex from "@stylexjs/stylex";

    export const styles = stylex.create({
      default: {
        backgroundColor: "red",
        color: "blue",
        backgroundPosition: "end",
        float: "start"
      },
    });
  `;

  const result = transform('page.tsx', fixture, {
    dev: false,
    treeshakeCompensation: true,
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  });

  const expected = {
    code: `import stylex from "@stylexjs/stylex";
export const styles = {
    default: {
        kWkggS: "xrkmrrc",
        kMwMTN: "xju2f9n",
        k1YJky: "x1ifmvib",
        kyUFMd: "xrbpyxo",
        $$css: true
    }
};
`,
    metadata: {
      stylex: [
        [
          'xrkmrrc',
          {
            ltr: '.xrkmrrc{background-color:red}',
            rtl: null,
          },
          3000,
        ],
        [
          'xju2f9n',
          {
            ltr: '.xju2f9n{color:blue}',
            rtl: null,
          },
          3000,
        ],
        [
          'x1ifmvib',
          {
            ltr: '.x1ifmvib{background-position:right}',
            rtl: '.x1ifmvib{background-position:left}',
          },
          2000,
        ],
        [
          'xrbpyxo',
          {
            ltr: '.xrbpyxo{float:left}',
            rtl: '.xrbpyxo{float:right}',
          },
          3000,
        ],
      ],
    },
    map: '{"version":3,"sources":["page.tsx"],"names":[],"mappings":"AACI;AAEA;;;;;;;;EAOG"}',
  };

  t.deepEqual(result, expected);
});
