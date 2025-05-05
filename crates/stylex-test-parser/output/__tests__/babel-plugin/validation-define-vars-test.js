function transform(source: string, opts: any = {}) {
    return transformSync(source, {
        filename: opts.filename || 'TestTheme.stylex.js',
        parserOpts: {
            flow: 'all'
        },
        plugins: [
            [
                stylexPlugin,
                {
                    stylexSheetName: '<>',
                    unstable_moduleResolution: {
                        type: 'haste'
                    },
                    ...opts
                }
            ]
        ]
    });
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[validation] stylex.defineVars()', ()=>{
        test('must be bound to a named export', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const styles = stylex.defineVars({});
        `);
            }).toThrow(messages.NON_EXPORT_NAMED_DECLARATION);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          stylex.defineVars({});
        `);
            }).toThrow(messages.UNBOUND_STYLEX_CALL_VALUE);
        });
        test('its only argument must be a single object', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars(genStyles());
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars(1);
        `);
            }).toThrow(messages.NON_OBJECT_FOR_STYLEX_CALL);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars();
        `);
            }).toThrow(messages.ILLEGAL_ARGUMENT_LENGTH);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({}, {});
        `);
            }).toThrow(messages.ILLEGAL_ARGUMENT_LENGTH);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({});
        `);
            }).not.toThrow();
        });
        test('variable keys must be a static value', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({
              [labelColor]: 'red',
          });
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
        });
        test('values must be static number or string, or keyframes in stylex.defineVars()', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({
              cornerRadius: 5,
          });
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({
              labelColor: 'red',
          });
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({
            fadeIn: stylex.keyframes({
              '0%': { opacity: 0 },
              '100%': { opacity: 1}
            }),
          });
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({
              labelColor: labelColor,
          });
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.defineVars({
              labelColor: labelColor(),
          });
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
        });
    });
});
