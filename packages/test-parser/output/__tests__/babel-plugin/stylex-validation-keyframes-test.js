function transform(source: string, opts: any = {}) {
    return transformSync(source, {
        filename: opts.filename,
        parserOpts: {
            flow: 'all'
        },
        plugins: [
            [
                stylexPlugin,
                opts
            ]
        ]
    });
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[validation] stylex.keyframes()', ()=>{
        test('only argument must be an object of objects', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const name = stylex.keyframes(null);
        `);
            }).toThrow(messages.NON_OBJECT_FOR_STYLEX_KEYFRAMES_CALL);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const name = stylex.keyframes({
            from: false
          });
        `);
            }).toThrow(messages.NON_OBJECT_KEYFRAME);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const name = stylex.keyframes({
            from: {},
            to: {},
          });
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const name = stylex.keyframes({
            '0%': {
              opacity: 0
            },
            '50%': {
              opacity: 0.5
            },
          });
        `);
            }).not.toThrow();
        });
        test('allow defined CSS variables in keyframes', ()=>{
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.keyframes({
              from: {
                backgroundColor: 'var(--bar)',
              },
            });
          `, {
                    definedStylexCSSVariables: {
                        bar: 1
                    }
                });
            }).not.toThrow();
        });
        test('allow undefined CSS variables in keyframes', ()=>{
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.keyframes({
              from: {
                backgroundColor: 'var(--foobar)',
              },
            });
          `, {
                    definedStylexCSSVariables: {
                        bar: 1
                    }
                });
            }).not.toThrow();
        });
    });
});
