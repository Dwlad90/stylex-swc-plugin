describe('createSheet', ()=>{
    describe('client side', ()=>{
        let createSheet;
        beforeAll(async ()=>{
            jest.resetModules();
            jest.mock('../src/stylesheet/utils.js', ()=>({
                    canUseDOM: true
                }));
            ({ createSheet } = await import('../src/stylesheet/createSheet.js'));
        });
        test('creates a sheet', ()=>{
            const sheet = createSheet();
            expect(typeof sheet.getTextContent()).toBe('string');
            expect(typeof sheet.insert).toBe('function');
        });
        test('creates multiple sheets', ()=>{
            const sheet1 = createSheet();
            const sheet2 = createSheet();
            expect(typeof sheet1.getTextContent()).toBe('string');
            expect(typeof sheet1.insert).toBe('function');
            expect(typeof sheet2.getTextContent()).toBe('string');
            expect(typeof sheet2.insert).toBe('function');
        });
        test('reuses existing sheet for given root', ()=>{
            const sheet1 = createSheet();
            const sheet2 = createSheet();
            sheet1.insert('.test{color:red}');
            expect(sheet1.getTextContent()).toEqual(sheet2.getTextContent());
            expect(document.querySelectorAll('[data-stylex]').length).toBe(1);
        });
        test('supports updating styles across multiple documents', ()=>{
            const sheet = createSheet();
            sheet.insert('.test-sheet { opacity: 1 }', 3);
            const iframe = document.createElement('iframe');
            document.body.appendChild(iframe);
            const iframeDoc = iframe.contentWindow.document;
            const iframeRootTag = document.createElement('div');
            iframeRootTag.id = 'test';
            iframeDoc.body.appendChild(iframeRootTag);
            const iframeSheet = createSheet(iframeRootTag);
            expect(sheet).not.toBe(iframeSheet);
            expect(typeof iframeSheet.insert).toBe('function');
            expect(iframeSheet.getTextContent().includes('test-sheet')).toBe(true);
            sheet.insert('.test-iframe { opacity: 0 }', 3);
            expect(iframeSheet.getTextContent().includes('test-iframe')).toBe(true);
            const div = document.createElement('div');
            const shadowRoot = div.attachShadow({
                mode: 'open'
            });
            const shadowRootTag = document.createElement('div');
            shadowRoot.appendChild(shadowRootTag);
            document.body.appendChild(shadowRoot);
            const shadowSheet = createSheet(shadowRootTag);
            expect(sheet).not.toBe(shadowSheet);
            expect(typeof shadowSheet.insert).toBe('function');
            expect(shadowSheet.getTextContent().includes('test-sheet')).toBe(true);
            sheet.insert('.test-shadow { opacity: 0 }', 3);
            expect(shadowSheet.getTextContent().includes('test-shadow')).toBe(true);
        });
    });
    describe('server side', ()=>{
        let createSheet;
        beforeAll(async ()=>{
            jest.resetModules();
            jest.mock('../src/stylesheet/utils.js', ()=>({
                    canUseDOM: false
                }));
            ({ createSheet } = await import('../src/stylesheet/createSheet.js'));
        });
        test('creates a sheet', ()=>{
            const sheet = createSheet();
            expect(typeof sheet.getTextContent()).toBe('string');
            expect(typeof sheet.insert).toBe('function');
        });
        test('creates multiple sheets', ()=>{
            const sheet1 = createSheet();
            const sheet2 = createSheet();
            expect(typeof sheet1.getTextContent()).toBe('string');
            expect(typeof sheet1.insert).toBe('function');
            expect(typeof sheet2.getTextContent()).toBe('string');
            expect(typeof sheet2.insert).toBe('function');
        });
    });
});
