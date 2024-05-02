describe('stylex-first-that-works test', ()=>{
    test('reverses simple array of values', ()=>{
        expect(firstThatWorks('a', 'b', 'c')).toEqual([
            'c',
            'b',
            'a'
        ]);
    });
});
