import * as stylex from '@stylexjs/stylex';
const _temp = {
    "MyComponent__styles.dyn": "MyComponent__styles.dyn",
    $$css: "MyComponent.js:5"
};
export const styles = {
    root: {
        "MyComponent__styles.root": "MyComponent__styles.root",
        "color-kMwMTN": "x1e2nbdu",
        $$css: "MyComponent.js:4"
    },
    dyn: (width)=>[
            _temp,
            {
                "width-kzqmXN": width != null ? "x5lhr3w" : width,
                $$css: "MyComponent.js:5"
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
