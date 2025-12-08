function transform(source, opts = {}) {
    const { code, metadata } = transformSync(source, {
        filename: opts.filename || '/stylex/packages/vars.stylex.js',
        parserOpts: {
            flow: 'all'
        },
        babelrc: false,
        plugins: [
            [
                stylexPlugin,
                {
                    unstable_moduleResolution: {
                        rootDir: '/stylex/packages/',
                        type: 'commonJS'
                    },
                    ...opts
                }
            ]
        ]
    });
    return {
        code,
        metadata
    };
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[transform] stylex.defineMarker()', ()=>{
        test('member call', ()=>{
            const { code, metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        export const fooBar = stylex.defineMarker();
      `);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from '@stylexjs/stylex';
        export const fooBar = {
          x1jdyizh: "x1jdyizh",
          $$css: true
        };"
      `);
            expect(metadata).toMatchInlineSnapshot(`
        {
          "stylex": [],
        }
      `);
        });
        test('named import call', ()=>{
            const { code } = transform(`
        import { defineMarker } from '@stylexjs/stylex';
        export const baz = defineMarker();
      `);
            expect(code).toMatchInlineSnapshot(`
        "import { defineMarker } from '@stylexjs/stylex';
        export const baz = {
          x1i61hkd: "x1i61hkd",
          $$css: true
        };"
      `);
        });
    });
});
