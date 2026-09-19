describe('transformValue content property tests', ()=>{
    test('preserves CSS functions without quotes', ()=>{
        const functions = [
            'counters(div, ".")',
            'counter(chapter)',
            'counter(chapter, upper-roman)',
            'attr(href)',
            'url(image.jpg)',
            'linear-gradient(#e66465, #9198e5)',
            'image-set("image1x.png" 1x, "image2x.png" 2x)',
            '"prefix"attr(href)',
            'url(foo.jpg)attr(alt)',
            'var(--test)',
            'var(--test, "default")'
        ];
        functions.forEach((input)=>{
            expect(transformValue('content', input, {})).toBe(input);
        });
    });
    test('preserves CSS keywords without quotes', ()=>{
        const keywords = [
            'normal',
            'none',
            'open-quote',
            'close-quote',
            'no-open-quote',
            'no-close-quote',
            'inherit',
            'initial',
            'revert',
            'revert-layer',
            'unset'
        ];
        keywords.forEach((keyword)=>{
            expect(transformValue('content', keyword, {})).toBe(keyword);
        });
    });
    test('handles mixed content values', ()=>{
        const mixedValues = [
            'open-quote counter(chapter)',
            '"prefix"url(image.jpg)',
            'url("test.png")/"Alt text"',
            'open-quotecounter(chapter)close-quote',
            'attr(href)normal',
            '"text"attr(href)"more text"',
            'counter(x)"text"counter(y)'
        ];
        mixedValues.forEach((input)=>{
            expect(transformValue('content', input, {})).toBe(input);
        });
    });
    test('adds quotes to plain strings', ()=>{
        const strings = [
            [
                'Hello world',
                '"Hello world"'
            ],
            [
                'Simple text',
                '"Simple text"'
            ],
            [
                '123',
                '"123"'
            ]
        ];
        strings.forEach(([input, expected])=>{
            expect(transformValue('content', input, {})).toBe(expected);
        });
    });
    test('adds quotes to plain strings containing quote characters', ()=>{
        const strings = [
            [
                "Bob's and Jim's",
                '"Bob\'s and Jim\'s"'
            ],
            [
                "It's a test, isn't it",
                '"It\'s a test, isn\'t it"'
            ],
            [
                'He said "hello"',
                '"He said \\"hello\\""'
            ],
            [
                'say "hi" now',
                '"say \\"hi\\" now"'
            ],
            [
                '"hello" is what he said',
                '"\\"hello\\" is what he said"'
            ]
        ];
        strings.forEach(([input, expected])=>{
            expect(transformValue('content', input, {})).toBe(expected);
        });
    });
    test('preserves CSS escape sequences when adding quotes', ()=>{
        const strings = [
            [
                '\\2014',
                '"\\2014"'
            ],
            [
                '\\201C hello \\201D',
                '"\\201C hello \\201D"'
            ],
            [
                'back\\slash',
                '"back\\slash"'
            ],
            [
                'C:\\\\Users',
                '"C:\\\\Users"'
            ],
            [
                'He said \\"hello\\"',
                '"He said \\"hello\\""'
            ],
            [
                '50% off \\',
                '"50% off \\\\"'
            ],
            [
                'line one\nline two',
                '"line one\\A line two"'
            ]
        ];
        strings.forEach(([input, expected])=>{
            expect(transformValue('content', input, {})).toBe(expected);
        });
    });
    test('preserves quote keywords combined with strings', ()=>{
        const values = [
            '"a" "b"',
            'open-quote "hello"',
            '"prefix" no-close-quote',
            'open-quote "text" close-quote'
        ];
        values.forEach((input)=>{
            expect(transformValue('content', input, {})).toBe(input);
        });
    });
    test('preserve units in zero values CSS variables', ()=>{
        const variables = [
            [
                '--test',
                '0px',
                '0px'
            ],
            [
                '--test',
                '0vdh',
                '0vdh'
            ],
            [
                'transform',
                '0rad',
                '0deg'
            ],
            [
                'animation-duration',
                '0ms',
                '0s'
            ],
            [
                'grid-template-rows',
                '0fr',
                '0fr'
            ],
            [
                'width',
                '0%',
                '0%'
            ],
            [
                'margin',
                '0px',
                '0'
            ]
        ];
        variables.forEach(([key, value, expected])=>{
            expect(transformValue(key, value, {})).toBe(expected);
        });
    });
});
