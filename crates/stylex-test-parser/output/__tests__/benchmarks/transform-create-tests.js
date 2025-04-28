function runSuite(options) {
    const { suite, test } = createSuite('babel-plugin: stylex.create', options);
    test('basic create', ()=>{
        transformHaste(createBasic);
    });
    test('complex create', ()=>{
        transformHaste(createComplex);
    });
    suite.run();
}
