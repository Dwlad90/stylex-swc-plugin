function transform(source, opts = {}) {
    return transformSync(source, {
        filename: opts.filename,
        parserOpts: {
            flow: 'all'
        },
        plugins: [
            [
                stylexPlugin,
                {
                    ...opts
                }
            ]
        ]
    });
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[validation] stylex.keyframes()', ()=>{
        test('local variable keyframes object', ()=>{
            const callTransform = ()=>transform(`
        import * as stylex from '@stylexjs/stylex';
        const keyframes = {
          from: {
            color: 'red',
          },
          to: {
            color: 'blue',
          }
        };
        export const name = stylex.keyframes(keyframes);
      `);
            expect(callTransform).toThrow();
        });
        test('only argument must be an object of objects', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const name = stylex.keyframes(null);
        `);
            }).toThrow(messages.nonStyleObject('keyframes'));
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
            '0%': {
              opacity: 0
            },
            '50%': {
              opacity: 0.5
            },
          });
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const name = stylex.keyframes({
            from: {},
            to: {},
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
