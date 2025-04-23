var classNamePrefix = 'x';
var defaultOpts = {
    stylexSheetName: '<>',
    unstable_moduleResolution: {
        type: 'haste'
    },
    classNamePrefix
};
function transform(file, opts = defaultOpts) {
    const result = transformFileSync(file, {
        filename: opts.filename || file || themeFile,
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
describe('create theme', ()=>{
    test('transform complex theme file', ()=>{
        transform(simpleThemeFile);
        const simpleStart = performance.now();
        const simpleResult = transform(simpleThemeFile);
        const simpleEnd = performance.now();
        expect(simpleResult.code).toMatchSnapshot();
        expect(simpleResult.styles).toMatchSnapshot();
        const simpleTimeTaken = simpleEnd - simpleStart;
        console.log('simpleTimeTaken', simpleTimeTaken);
        const start = performance.now();
        const result = transform(themeFile);
        const end = performance.now();
        const timeTaken = end - start;
        expect(result.code).toMatchSnapshot();
        expect(result.styles).toMatchSnapshot();
        console.log('timeTaken', timeTaken);
        expect(timeTaken).toBeLessThan(simpleTimeTaken * 20);
    });
});
