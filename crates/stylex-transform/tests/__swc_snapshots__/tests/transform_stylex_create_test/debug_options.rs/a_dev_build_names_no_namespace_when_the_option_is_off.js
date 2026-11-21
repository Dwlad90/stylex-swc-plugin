import * as stylex from '@stylexjs/stylex';
export const styles = {
    root: {
        "color-kMwMTN": "x1e2nbdu",
        $$css: "@stylexswc/transform:MyComponent.js:4"
    },
    dyn: (width)=>[
            width !== undefined && {
                "width-kzqmXN": width != null ? "x5lhr3w" : "",
                $$css: "@stylexswc/transform:MyComponent.js:5"
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
