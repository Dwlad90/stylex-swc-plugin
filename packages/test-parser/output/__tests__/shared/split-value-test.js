describe('Ensure CSS values are split correctly', ()=>{
    test('simple space-separated numbers', ()=>{
        expect(splitValue('0 1 2 3')).toEqual([
            '0',
            '1',
            '2',
            '3'
        ]);
    });
    test('simple space-separated lengths', ()=>{
        expect(splitValue('0px 1rem 2% 3em')).toEqual([
            '0px',
            '1rem',
            '2%',
            '3em'
        ]);
    });
    test('simple comma-separated numbers', ()=>{
        expect(splitValue('0, 1, 2, 3')).toEqual([
            '0',
            '1',
            '2',
            '3'
        ]);
    });
    test('simple comma-separated lengths', ()=>{
        expect(splitValue('0px, 1rem, 2%, 3em')).toEqual([
            '0px',
            '1rem',
            '2%',
            '3em'
        ]);
    });
    test('Does not lists within functions', ()=>{
        expect(splitValue('rgb(255 200 0)')).toEqual([
            'rgb(255 200 0)'
        ]);
        expect(splitValue('rgb(255 200 / 0.5)')).toEqual([
            'rgb(255 200/0.5)'
        ]);
    });
    test('Does not lists within calc', ()=>{
        expect(splitValue('calc((100% - 50px) * 0.5)')).toEqual([
            'calc((100% - 50px) * 0.5)'
        ]);
        expect(splitValue('calc((100% - 50px) * 0.5) var(--rightpadding, 20px)')).toEqual([
            'calc((100% - 50px) * 0.5)',
            'var(--rightpadding,20px)'
        ]);
    });
});
