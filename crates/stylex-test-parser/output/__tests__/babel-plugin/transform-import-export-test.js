function transform(source, opts = {}) {
    const { code, metadata } = transformSync(source, {
        filename: opts.filename,
        parserOpts: {
            flow: 'all'
        },
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
var defaultImportText = '* as stylex';
var defaultImportSource = '@stylexjs/stylex';
var defaultImportMap = {
    create: 'stylex.create',
    createTheme: 'stylex.createTheme',
    defineVars: 'stylex.defineVars',
    firstThatWorks: 'stylex.firstThatWorks',
    keyframes: 'stylex.keyframes',
    props: 'stylex.props'
};
function createStylesFixture({ importText: _importText, importSource: _importSource, importMap: _importMap } = {}) {
    const importText = _importText || defaultImportText;
    const importSource = _importSource || defaultImportSource;
    const importMap = _importMap || defaultImportMap;
    const { create, createTheme, defineVars, firstThatWorks, keyframes, props } = importMap;
    const from = importSource?.from || importSource;
    const defineVarsOutput = transform(`
    import ${importText} from "${from}";
    export const vars = ${defineVars}({
      bar: 'left'
    });
  `, {
        filename: '/stylex/packages/vars.stylex.js',
        importSources: [
            importSource
        ]
    }).code;
    return `
    ${defineVarsOutput}
    const styles = ${create}({
      root: {
        animationName: ${keyframes}({
          from: {
            backgroundColor: 'yellow'
          },
          to: {
            backgroundColor: 'orange'
          },
        }),
        color: 'red',
        position: ${firstThatWorks}('sticky', 'fixed')
      }
    });

    const theme = ${createTheme}(vars, {
      bar: 'green'
    });

    ${props}(styles.root, theme);
  `;
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[transform] stylex imports', ()=>{
        let expectedImportTestMetadata = null;
        beforeEach(()=>{
            expectedImportTestMetadata = transform(createStylesFixture()).metadata;
        });
        test('import: none', ()=>{
            const { code, metadata } = transform(`
        export const styles = stylex.create({
          root: {
            color: 'red'
          }
        });
      `);
            expect(code).toMatchInlineSnapshot(`
        "export const styles = stylex.create({
          root: {
            color: 'red'
          }
        });"
      `);
            expect(metadata).toMatchInlineSnapshot(`
        {
          "stylex": [],
        }
      `);
        });
        test('import: non-stylex', ()=>{
            const { code, metadata } = transform(`
        import {foo, bar} from 'other';
      `);
            expect(code).toMatchInlineSnapshot('"import { foo, bar } from \'other\';"');
            expect(metadata).toMatchInlineSnapshot(`
        {
          "stylex": [],
        }
      `);
        });
        test('require: non-stylex', ()=>{
            const { code, metadata } = transform(`
        const {foo, bar} = require('other');
      `);
            expect(code).toMatchInlineSnapshot(`
        "const {
          foo,
          bar
        } = require('other');"
      `);
            expect(metadata).toMatchInlineSnapshot(`
        {
          "stylex": [],
        }
      `);
        });
        test('import: wildcard (the default)', ()=>{
            const fixture = createStylesFixture();
            const { code, metadata } = transform(fixture);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from "@stylexjs/stylex";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        stylex.props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
        test('import: wildcard (non-stylex name)', ()=>{
            const fixture = createStylesFixture({
                importText: '* as foo',
                importMap: {
                    create: 'foo.create',
                    createTheme: 'foo.createTheme',
                    defineVars: 'foo.defineVars',
                    firstThatWorks: 'foo.firstThatWorks',
                    keyframes: 'foo.keyframes',
                    props: 'foo.props'
                }
            });
            const { code, metadata } = transform(fixture);
            expect(code).toMatchInlineSnapshot(`
        "import * as foo from "@stylexjs/stylex";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        foo.props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
        test('import: named', ()=>{
            const fixture = createStylesFixture({
                importText: '{create, createTheme, defineVars, firstThatWorks, keyframes, props}',
                importMap: {
                    create: 'create',
                    createTheme: 'createTheme',
                    defineVars: 'defineVars',
                    firstThatWorks: 'firstThatWorks',
                    keyframes: 'keyframes',
                    props: 'props'
                }
            });
            const { code, metadata } = transform(fixture);
            expect(code).toMatchInlineSnapshot(`
        "import { create, createTheme, defineVars, firstThatWorks, keyframes, props } from "@stylexjs/stylex";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
        test('import: named alias', ()=>{
            const fixture = createStylesFixture({
                importText: `{
          create as _create,
          createTheme as _createTheme,
          defineVars as _defineVars,
          firstThatWorks as _firstThatWorks,
          keyframes as _keyframes,
          props as _props
        }`,
                importMap: {
                    create: '_create',
                    createTheme: '_createTheme',
                    defineVars: '_defineVars',
                    firstThatWorks: '_firstThatWorks',
                    keyframes: '_keyframes',
                    props: '_props'
                }
            });
            const { code, metadata } = transform(fixture);
            expect(code).toMatchInlineSnapshot(`
        "import { create as _create, createTheme as _createTheme, defineVars as _defineVars, firstThatWorks as _firstThatWorks, keyframes as _keyframes, props as _props } from "@stylexjs/stylex";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        _props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
        test('importSources (string)', ()=>{
            const importSource = 'foo-bar';
            const fixture = createStylesFixture({
                importText: '* as stylex',
                importSource
            });
            const options = {
                importSources: [
                    importSource
                ]
            };
            const { code, metadata } = transform(fixture, options);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from "foo-bar";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        stylex.props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
        test('importSources (object)', ()=>{
            const importSource = {
                as: 'css',
                from: 'react-strict-dom'
            };
            const fixture = createStylesFixture({
                importText: '{css, html}',
                importSource,
                importMap: {
                    create: 'css.create',
                    createTheme: 'css.createTheme',
                    defineVars: 'css.defineVars',
                    firstThatWorks: 'css.firstThatWorks',
                    keyframes: 'css.keyframes',
                    props: 'css.props'
                }
            });
            const options = {
                importSources: [
                    importSource
                ]
            };
            const { code, metadata } = transform(fixture, options);
            expect(code).toMatchInlineSnapshot(`
        "import { css, html } from "react-strict-dom";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        css.props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
        test('[META-ONLY] import: default', ()=>{
            const fixture = createStylesFixture({
                importText: 'stylex',
                importSource: 'stylex'
            });
            const { code, metadata } = transform(fixture);
            expect(code).toMatchInlineSnapshot(`
        "import stylex from "stylex";
        export const vars = {
          bar: "var(--x1hi1hmf)",
          __themeName__: "xop34xu"
        };
        const styles = {
          root: {
            kKVMdj: "x1qar0u3",
            kMwMTN: "x1e2nbdu",
            kVAEAm: "x15oojuh",
            $$css: true
          }
        };
        const theme = {
          $$css: true,
          xop34xu: "xfnndu4 xop34xu"
        };
        stylex.props(styles.root, theme);"
      `);
            expect(metadata).toEqual(expectedImportTestMetadata);
        });
    });
    describe('[transform] stylex exports', ()=>{
        let expectedExportTestMetadata = null;
        const fixture = `stylex.create({
      root: {
        color: 'red',
      }
    })`;
        beforeEach(()=>{
            expectedExportTestMetadata = transform(`
        import * as stylex from '@stylexjs/stylex';
        const styles = ${fixture};
      `).metadata;
        });
        test('export: named property', ()=>{
            const { code, metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        const styles = ${fixture};
        export {styles}
      `);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from '@stylexjs/stylex';
        const styles = {
          root: {
            kMwMTN: "x1e2nbdu",
            $$css: true
          }
        };
        export { styles };"
      `);
            expect(metadata).toEqual(expectedExportTestMetadata);
        });
        test('export: named declaration', ()=>{
            const { code, metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        export const styles = ${fixture};
      `);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from '@stylexjs/stylex';
        export const styles = {
          root: {
            kMwMTN: "x1e2nbdu",
            $$css: true
          }
        };"
      `);
            expect(metadata).toEqual(expectedExportTestMetadata);
        });
        test('export: default', ()=>{
            const { code, metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        export default (${fixture});
      `);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from '@stylexjs/stylex';
        export default {
          root: {
            kMwMTN: "x1e2nbdu",
            $$css: true
          }
        };"
      `);
            expect(metadata).toEqual(expectedExportTestMetadata);
        });
        test('module.export', ()=>{
            const { code, metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        const styles = ${fixture};
        module.export = styles;
      `);
            expect(code).toMatchInlineSnapshot(`
        "import * as stylex from '@stylexjs/stylex';
        const styles = {
          root: {
            kMwMTN: "x1e2nbdu",
            $$css: true
          }
        };
        module.export = styles;"
      `);
            expect(metadata).toEqual(expectedExportTestMetadata);
        });
    });
});
