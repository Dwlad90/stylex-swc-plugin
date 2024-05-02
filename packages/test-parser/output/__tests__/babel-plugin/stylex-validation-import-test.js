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
    describe('[validation] stylex imports', ()=>{
        test('ignore non-stylex imports', ()=>{
            expect(()=>{
                transform(`
          import classnames from 'classnames';
        `);
            }).not.toThrow();
        });
        test('support named export of stylex.create()', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export const styles = stylex.create({});
        `);
            }).not.toThrow();
        });
        test('support default export of stylex.create()', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          export default stylex.create({});
        `);
            }).not.toThrow();
        });
    });
});
