var defaultOpts = {
    stylexSheetName: '<>',
    unstable_moduleResolution: {
        type: 'haste'
    },
    classNamePrefix: 'x'
};
function transform(file, opts = defaultOpts) {
    const result = transformFileSync(file, {
        filename: opts.filename || file || themes,
        parserOpts: {
            flow: 'all'
        },
        babelrc: false,
        plugins: [
            [
                'babel-plugin-syntax-hermes-parser',
                {
                    flow: 'detect'
                }
            ],
            [
                stylexPlugin,
                {
                    ...defaultOpts,
                    ...opts
                }
            ]
        ]
    });
    return {
        code: result.code,
        styles: result.metadata.stylex
    };
}
function runSuite(options) {
    const { suite, test } = createSuite('babel-plugin: createTheme', options);
    test('basic theme', ()=>{
        transform(themeBasic);
    });
    test('complex theme', ()=>{
        transform(themes);
    });
    suite.run();
}
