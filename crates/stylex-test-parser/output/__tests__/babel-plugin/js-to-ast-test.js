describe('convertObjectToAST', ()=>{
    test('converts empty object', ()=>{
        const result = convertObjectToAST({});
        expect(generate(result).code).toMatchInlineSnapshot('"{}"');
    });
    test('converts object with values', ()=>{
        const result = convertObjectToAST({
            base: {
                width: {
                    default: 800,
                    '@media (max-width: 800px)': '100%',
                    '@media (min-width: 1540px)': 1366
                }
            }
        });
        expect(generate(result).code).toMatchInlineSnapshot(`
      "{
        base: {
          width: {
            default: 800,
            "@media (max-width: 800px)": "100%",
            "@media (min-width: 1540px)": 1366
          }
        }
      }"
    `);
    });
});
