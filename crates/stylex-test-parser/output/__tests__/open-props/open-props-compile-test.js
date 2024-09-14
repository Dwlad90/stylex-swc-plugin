function transform(source, opts = {}) {
    return transformSync(source, {
        filename: opts.filename,
        parserOpts: {
            flow: 'all'
        },
        plugins: [
            flowPlugin,
            [
                stylexPlugin,
                {
                    ...opts,
                    treeshakeCompensation: true,
                    unstable_moduleResolution: {
                        type: 'commonJS',
                        rootDir: path.join(__dirname, '../../')
                    }
                }
            ]
        ]
    });
}
describe('commonJS results of exported styles and variables', ()=>{
    files.forEach((file)=>{
        if (file.endsWith('.js')) {
            const filename = path.join(__dirname, '../src', file);
            const source = fs.readFileSync(filename, 'utf8');
            const { code, metadata } = transform(source, {
                dev: false,
                filename: filename
            });
            test(file, ()=>{
                expect(code).toMatchSnapshot();
                expect(metadata.stylex).toMatchSnapshot();
            });
        }
    });
});
