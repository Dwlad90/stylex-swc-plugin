var testOpts = {
    rootTheme: undefined,
    supportsVariables: true
};
test('stylexinject', ()=>{
    const prevCount = styleSheet.getRuleCount();
    styleSheet.insert('hey {}', 0);
    expect(styleSheet.getRuleCount()).toBeGreaterThan(prevCount);
});
test('StyleXSheet.prototype.insert', ()=>{
    const sheet = new StyleXSheet(testOpts);
    expect(sheet.getRuleCount()).toBe(0);
    sheet.inject();
    expect(sheet.getRuleCount()).toBe(0);
    sheet.insert('.a {}', 0);
    expect(sheet.getRuleCount()).toBe(1);
    sheet.insert('.b {}', 0);
    expect(sheet.getRuleCount()).toBe(2);
    sheet.insert('.b {}', 0);
    expect(sheet.getRuleCount()).toBe(2);
});
test('StyleXSheet.prototype.insert respects priorities', ()=>{
    const sheet = new StyleXSheet(testOpts);
    sheet.insert('.last {}', 6);
    sheet.insert('.third {}', 3);
    sheet.insert('.first {}', 0);
    sheet.insert('.second {}', 1);
    expect(sheet.getCSS()).toMatchInlineSnapshot(`
    ".first {}
    .second {}
    .third {}
    .last {}"
  `);
});
test('StyleXSheet.prototype.insert respects priority floats', ()=>{
    const sheet = new StyleXSheet(testOpts);
    sheet.insert('.fourth {}', 6.8);
    sheet.insert('.third {}', 6.5);
    sheet.insert('.second {}', 6);
    sheet.insert('.first {}', 2);
    expect(sheet.getCSS()).toMatchInlineSnapshot(`
    ".first {}
    .second {}
    .third {}
    .fourth {}"
  `);
});
test('StyleXSheet.prototype.insert handles RTL rules with media queries', ()=>{
    const sheet = new StyleXSheet(testOpts);
    sheet.insert('@media (min-width: 1000px) { .foo {} }', 0, '@media (min-width: 1000px) { .foo {} }');
    expect(sheet.getCSS()).toMatchInlineSnapshot(`
    "@media (min-width: 1000px) {html:not([dir='rtl'])  .foo {} }
    @media (min-width: 1000px) {html[dir='rtl']  .foo {} }"
  `);
});
test('inlines variables for older browsers', ()=>{
    const sheet = new StyleXSheet({
        rootDarkTheme: {
            foo: 'reallydark'
        },
        rootTheme: {
            foo: 'bar'
        },
        supportsVariables: false
    });
    sheet.insert('.foo {color: var(--foo)}', 1);
    expect(sheet.getCSS()).toMatchInlineSnapshot(`
    ":root, .__fb-light-mode {
      --foo: bar;
    }
    .__fb-dark-mode:root, .__fb-dark-mode {
      --foo: reallydark;
    }
    .foo {color: bar}"
  `);
});
