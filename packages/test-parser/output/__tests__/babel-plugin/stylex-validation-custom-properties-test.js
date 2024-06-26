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
    describe('[validation] CSS custom properties', ()=>{
        test('disallow unclosed style value functions', ()=>{
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({default: {color: 'var(--foo'}})
          `, {
                    definedStylexCSSVariables: {
                        foo: 1
                    }
                });
            }).toThrow(messages.LINT_UNCLOSED_FUNCTION);
        });
        test('disallow unprefixed custom properties', ()=>{
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({default: {color: 'var(foo'}})
          `, {
                    definedStylexCSSVariables: {
                        foo: 1
                    }
                });
            }).toThrow();
        });
        test('allow defined custom properties', ()=>{
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({foo: { color: 'var(--foo)' }});
          `, {
                    definedStylexCSSVariables: {
                        foo: 1
                    }
                });
            }).not.toThrow();
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({foo: { backgroundColor: 'var(--foo)', color: 'var(--bar)' }});
          `, {
                    definedStylexCSSVariables: {
                        foo: 1,
                        bar: 1
                    }
                });
            }).not.toThrow();
        });
        test('allow undefined custom properties', ()=>{
            expect(()=>{
                transform(`
          import stylex from 'stylex';
          const styles = stylex.create({foo: { color: 'var(--foobar)' }});
        `);
            }).not.toThrow();
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({foo: { color: 'var(--foobar)' }});
          `, {
                    definedStylexCSSVariables: null
                });
            }).not.toThrow();
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({foo: { color: 'var(--foobar)' }});
          `, {
                    definedStylexCSSVariables: undefined
                });
            }).not.toThrow();
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({foo: { color: 'var(--foobar)' }});
          `, {
                    definedStylexCSSVariables: {
                        foo: 1
                    }
                });
            }).not.toThrow();
            expect(()=>{
                transform(`
            import stylex from 'stylex';
            const styles = stylex.create({foo: { backgroundColor: 'var(--foofoo)', color: 'var(--foobar)' }});
          `, {
                    definedStylexCSSVariables: {
                        foo: 1,
                        bar: 1
                    }
                });
            }).not.toThrow();
        });
    });
});
