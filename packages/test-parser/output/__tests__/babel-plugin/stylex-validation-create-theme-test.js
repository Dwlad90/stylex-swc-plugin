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
    describe('[validation] stylex.createTheme()', ()=>{
        test('must be bound to a variable', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          stylex.createTheme({__themeName__: 'x568ih9'}, {});
        `);
            }).toThrow(messages.UNBOUND_STYLEX_CALL_VALUE);
        });
        test('it must have two arguments', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme();
        `);
            }).toThrow(messages.ILLEGAL_ARGUMENT_LENGTH);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({});
        `);
            }).toThrow(messages.ILLEGAL_ARGUMENT_LENGTH);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(genStyles(), {});
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({}, {});
        `);
            }).toThrow('Can only override variables theme created with stylex.defineVars().');
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({__themeName__: 'x568ih9'}, genStyles());
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({__themeName__: 'x568ih9'}, {});
        `);
            }).not.toThrow();
        });
        test('variable keys must be a static value', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {[labelColor]: 'red',});
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
        });
        test('values must be static number or string in stylex.createTheme()', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', cornerRadius: 'var(--cornerRadiusHash)'},
            {cornerRadius: 5,}
          );
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: 'red',}
          );
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: labelColor,}
          );
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: labelColor(),}
          );
        `);
            }).toThrow(messages.NON_STATIC_VALUE);
        });
    });
});
