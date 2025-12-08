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
    describe('[validation] stylex.defineMarker()', ()=>{
        test('must be bound to a named export', ()=>{
            expect(()=>{
                transform(`
          import * as stylex from '@stylexjs/stylex';
          const marker = stylex.defineMarker();
        `);
            }).toThrow(messages.nonExportNamedDeclaration('defineMarker'));
        });
        test('no arguments allowed', ()=>{
            expect(()=>{
                transform(`
          import * as stylex from '@stylexjs/stylex';
          export const marker = stylex.defineMarker(1);
        `);
            }).toThrow(messages.illegalArgumentLength('defineMarker', 0));
        });
    });
});
