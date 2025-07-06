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
                    unstable_moduleResolution: {
                        type: 'commonJS'
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
            }).toThrow(messages.unboundCallValue('createTheme'));
        });
        test('it must have two arguments', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme();
        `);
            }).toThrow(messages.illegalArgumentLength('createTheme', 2));
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({});
        `);
            }).toThrow(messages.illegalArgumentLength('createTheme', 2));
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(genStyles(), {});
        `);
            }).toThrow(messages.nonStaticValue('createTheme'));
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({}, {});
        `);
            }).toThrow('Can only override variables theme created with defineVars().');
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme({__themeName__: 'x568ih9'}, genStyles());
        `);
            }).toThrow(messages.nonStaticValue('createTheme'));
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
            }).toThrow(messages.nonStaticValue('createTheme'));
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
            }).toThrow(messages.nonStaticValue('createTheme'));
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: labelColor(),}
          );
        `);
            }).toThrow(messages.nonStaticValue('createTheme'));
        });
    });
});
