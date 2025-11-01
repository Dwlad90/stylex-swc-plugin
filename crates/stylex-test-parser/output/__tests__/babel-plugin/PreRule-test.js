var options = {
    classNamePrefix: 'x',
    styleResolution: 'legacy-expand-shorthands',
    dev: false,
    debug: false,
    enableFontSizePxToRem: true,
    runtimeInjection: false,
    test: false
} as const;
describe('Converting PreRule to CSS', ()=>{
    test('should convert a PreRule to CSS', ()=>{
        expect(new PreRule('color', 'red', [
            'color'
        ]).compiled(options)).toMatchInlineSnapshot(`
      [
        [
          "x1e2nbdu",
          {
            "ltr": ".x1e2nbdu{color:red}",
            "priority": 3000,
            "rtl": null,
          },
          {
            "x1e2nbdu": [
              "color",
            ],
          },
        ],
      ]
    `);
    });
});
