describe('inject', ()=>{
    test('@keyframes', ()=>{
        const cssText = '@keyframes name { from: { color: red }, to: { color: blue } }';
        expect(inject(cssText, 10)).toMatchInlineSnapshot('"@keyframes name { from: { color: red }, to: { color: blue } }"');
    });
    test('@media', ()=>{
        const cssText = '@media (min-width: 320px) { .color { color: red } }';
        expect(inject(cssText, 200)).toMatchInlineSnapshot('"@media (min-width: 320px) { .color { color: red } }"');
    });
    test('::before', ()=>{
        const cssText = '.color::before { color: red }';
        expect(inject(cssText, 5000)).toMatchInlineSnapshot('".color:not(#\\#):not(#\\#):not(#\\#):not(#\\#):not(#\\#)::before { color: red }"');
    });
    test(':hover', ()=>{
        const cssText = '.color:hover { color: red }';
        expect(inject(cssText, 130)).toMatchInlineSnapshot('".color:hover { color: red }"');
    });
    test('::before:hover', ()=>{
        const cssText = '.color::before:hover { color: red }';
        expect(inject(cssText, 5000)).toMatchInlineSnapshot('".color:not(#\\#):not(#\\#):not(#\\#):not(#\\#):not(#\\#)::before:hover { color: red }"');
    });
});
