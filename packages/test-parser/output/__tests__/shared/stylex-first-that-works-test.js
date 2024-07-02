describe('stylex-first-that-works test', ()=>{
    test.skip('reverses simple array of values', ()=>{
        expect(firstThatWorks('a', 'b')).toEqual([
            'b',
            'a'
        ]);
        expect(firstThatWorks('a', 'b', 'c')).toEqual([
            'c',
            'b',
            'a'
        ]);
    });
    test('creates fallbacks for variables', ()=>{
        expect(firstThatWorks('var(--accent)', 'blue')).toEqual('var(--accent, blue)');
    });
    test.skip('Allow variables to be fallbacks too', ()=>{
        expect(firstThatWorks('color-mix(in srgb, currentColor 20%, transparent)', 'var(--accent)', 'blue')).toEqual([
            'var(--accent, blue)',
            'color-mix(in srgb, currentColor 20%, transparent)'
        ]);
    });
    test.skip('Omit all but first fallback after the last variable', ()=>{
        expect(firstThatWorks('color-mix(in oklch, currentColor 20%, transparent)', 'color-mix(in srgb, currentColor 20%, transparent)', 'var(--accent)', 'var(--primary)', 'var(--secondary)', 'red', 'blue', 'green')).toEqual([
            'var(--accent, var(--primary, var(--secondary, red)))',
            'color-mix(in srgb, currentColor 20%, transparent)',
            'color-mix(in oklch, currentColor 20%, transparent)'
        ]);
    });
});
