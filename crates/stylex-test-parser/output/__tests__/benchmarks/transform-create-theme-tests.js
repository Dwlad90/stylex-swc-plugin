function runSuite(options) {
    const { suite, test } = createSuite('babel-plugin: stylex.createTheme', options);
    test('basic themes', ()=>{
        transformHaste(createThemeBasic);
    });
    test('complex themes', ()=>{
        transformHaste(createThemeComplex);
    });
    suite.run();
}
