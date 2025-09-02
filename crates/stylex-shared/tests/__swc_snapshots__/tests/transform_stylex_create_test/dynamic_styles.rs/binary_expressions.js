import * as stylex from '@stylexjs/stylex';
const _temp = {
    kzqmXN: "x5lhr3w",
    kZKoxP: "x16ye13r",
    kogj98: "xb9ncqk",
    kmVPX3: "x1fozly0",
    $$css: true
};
export const styles = {
    root: (width, height)=>[
            _temp,
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width + 100),
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(height * 2),
                "--x-margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width - 50),
                "--x-padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(height / 2)
            }
        ]
};
