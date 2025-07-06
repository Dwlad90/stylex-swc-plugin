function transform(source, opts = {}) {
    const { code, metadata } = transformSync(source, {
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
    return {
        code,
        metadata
    };
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[transform] CSS property polyfills', ()=>{
        test.skip('lineClamp', ()=>{
            const { metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { lineClamp: 3 } });
      `);
            expect(metadata).toMatchInlineSnapshot();
        });
        test.skip('pointerEvents', ()=>{
            const { metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({
          a: { pointerEvents: 'auto' },
          b: { pointerEvents: 'box-none' },
          c: { pointerEvents: 'box-only' },
          d: { pointerEvents: 'none' }
        });
      `);
            expect(metadata).toMatchInlineSnapshot();
        });
        test.skip('scrollbarWidth', ()=>{
            const { metadata } = transform(`
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
      `);
            expect(metadata).toMatchInlineSnapshot();
        });
    });
});
