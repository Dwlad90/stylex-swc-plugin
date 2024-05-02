function transform(source, opts = {}) {
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
    }).code;
}
describe('@stylexjs/babel-plugin', ()=>{
    describe.skip('[transform] stylex polyfills', ()=>{
        test('lineClamp', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { lineClamp: 3 } });
        `)).toMatchInlineSnapshot();
        });
        test('pointerEvents', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({
            a: { pointerEvents: 'auto' },
            b: { pointerEvents: 'box-none' },
            c: { pointerEvents: 'box-only' },
            d: { pointerEvents: 'none' }
          });
        `)).toMatchInlineSnapshot();
        });
        test('scrollbarWidth', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
        `)).toMatchInlineSnapshot();
        });
    });
});
