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
                    ...opts,
                    runtimeInjection: true
                }
            ]
        ]
    }).code;
}
describe('@stylexjs/babel-plugin', ()=>{
    describe('[transform] CSS logical properties', ()=>{
        test('"borderBlockColor"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockColor: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1lkbs04{border-block-color:0}", 3000);
        const classnames = "x1lkbs04";"
      `);
        });
        test('"borderBlockStartColor"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockStartColor: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x4q076{border-top-color:0}", 4000);
        const classnames = "x4q076";"
      `);
        });
        test('"borderBlockEndColor"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockEndColor: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1ylptbq{border-bottom-color:0}", 4000);
        const classnames = "x1ylptbq";"
      `);
        });
        test('"borderInlineColor"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineColor: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1v09clb{border-inline-color:0}", 2000);
        const classnames = "x1v09clb";"
      `);
        });
        test('"borderInlineStartColor"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineStartColor: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1t19a1o{border-inline-start-color:0}", 3000);
        const classnames = "x1t19a1o";"
      `);
        });
        test('"borderInlineEndColor"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineEndColor: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x14mj1wy{border-inline-end-color:0}", 3000);
        const classnames = "x14mj1wy";"
      `);
        });
        test('"borderBlockStyle"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockStyle: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x7mea6a{border-block-style:0}", 3000);
        const classnames = "x7mea6a";"
      `);
        });
        test('"borderBlockStartStyle"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockStartStyle: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1d917x0{border-top-style:0}", 4000);
        const classnames = "x1d917x0";"
      `);
        });
        test('"borderBlockEndStyle"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockEndStyle: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1nmap2y{border-bottom-style:0}", 4000);
        const classnames = "x1nmap2y";"
      `);
        });
        test('"borderInlineStyle"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineStyle: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xt8kkye{border-inline-style:0}", 2000);
        const classnames = "xt8kkye";"
      `);
        });
        test('"borderInlineStartStyle"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineStartStyle: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xl8mozw{border-inline-start-style:0}", 3000);
        const classnames = "xl8mozw";"
      `);
        });
        test('"borderInlineEndStyle"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineEndStyle: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x10o505a{border-inline-end-style:0}", 3000);
        const classnames = "x10o505a";"
      `);
        });
        test('"borderBlockWidth"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockWidth: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1616tdu{border-block-width:0}", 3000);
        const classnames = "x1616tdu";"
      `);
        });
        test('"borderBlockStartWidth"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockStartWidth: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x972fbf{border-top-width:0}", 4000);
        const classnames = "x972fbf";"
      `);
        });
        test('"borderBlockEndWidth"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderBlockEndWidth: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1qhh985{border-bottom-width:0}", 4000);
        const classnames = "x1qhh985";"
      `);
        });
        test('"borderInlineWidth"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineWidth: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xuxrje7{border-inline-width:0}", 2000);
        const classnames = "xuxrje7";"
      `);
        });
        test('"borderInlineStartWidth"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineStartWidth: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x14e42zd{border-inline-start-width:0}", 3000);
        const classnames = "x14e42zd";"
      `);
        });
        test('"borderInlineEndWidth"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { borderInlineEndWidth: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x10w94by{border-inline-end-width:0}", 3000);
        const classnames = "x10w94by";"
      `);
        });
        test('"insetBlock"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { insetBlock: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x10no89f{inset-block:0}", 2000);
        const classnames = "x10no89f";"
      `);
        });
        test('"insetBlockEnd"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { insetBlockEnd: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1ey2m1c{bottom:0}", 4000);
        const classnames = "x1ey2m1c";"
      `);
        });
        test('"insetBlockStart"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { insetBlockStart: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x13vifvy{top:0}", 4000);
        const classnames = "x13vifvy";"
      `);
        });
        test('"insetInline"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { insetInline: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x17y0mx6{inset-inline:0}", 2000);
        const classnames = "x17y0mx6";"
      `);
        });
        test('"insetInlineEnd"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { insetInlineEnd: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xtijo5x{inset-inline-end:0}", 3000);
        const classnames = "xtijo5x";"
      `);
        });
        test('"insetInlineStart"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { insetInlineStart: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1o0tod{inset-inline-start:0}", 3000);
        const classnames = "x1o0tod";"
      `);
        });
        test('"marginBlock"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { marginBlock: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x10im51j{margin-block:0}", 2000);
        const classnames = "x10im51j";"
      `);
        });
        test('"marginBlockEnd"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { marginBlockEnd: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xat24cr{margin-bottom:0}", 4000);
        const classnames = "xat24cr";"
      `);
        });
        test('"marginBlockStart"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { marginBlockStart: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xdj266r{margin-top:0}", 4000);
        const classnames = "xdj266r";"
      `);
        });
        test('"marginInline"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { marginInline: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xrxpjvj{margin-inline:0}", 2000);
        const classnames = "xrxpjvj";"
      `);
        });
        test('"marginInlineEnd"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { marginInlineEnd: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x14z9mp{margin-inline-end:0}", 3000);
        const classnames = "x14z9mp";"
      `);
        });
        test('"marginInlineStart"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { marginInlineStart: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1lziwak{margin-inline-start:0}", 3000);
        const classnames = "x1lziwak";"
      `);
        });
        test('"paddingBlock"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { paddingBlock: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xt970qd{padding-block:0}", 2000);
        const classnames = "xt970qd";"
      `);
        });
        test('"paddingBlockEnd"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { paddingBlockEnd: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x18d9i69{padding-bottom:0}", 4000);
        const classnames = "x18d9i69";"
      `);
        });
        test('"paddingBlockStart"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { paddingBlockStart: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xexx8yu{padding-top:0}", 4000);
        const classnames = "xexx8yu";"
      `);
        });
        test('"paddingInline"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { paddingInline: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xnjsko4{padding-inline:0}", 2000);
        const classnames = "xnjsko4";"
      `);
        });
        test('"paddingInlineEnd"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { paddingInlineEnd: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xyri2b{padding-inline-end:0}", 3000);
        const classnames = "xyri2b";"
      `);
        });
        test('"paddingInlineStart"', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({ x: { paddingInlineStart: 0 } });
          const classnames = stylex(styles.x);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".x1c1uobl{padding-inline-start:0}", 3000);
        const classnames = "x1c1uobl";"
      `);
        });
        test('[legacy] short-form property value flipping', ()=>{
            expect(transform(`
          import stylex from 'stylex';
          const styles = stylex.create({
            four: {
              margin: '1 2 3 4',
            }
          });
          stylex(styles.four);
        `)).toMatchInlineSnapshot(`
        "import _inject from "@stylexjs/stylex/lib/stylex-inject";
        var _inject2 = _inject;
        import stylex from 'stylex';
        _inject2(".xi71r3n{margin:1 2 3 4}", 1000);
        "xi71r3n";"
      `);
        });
    });
});
