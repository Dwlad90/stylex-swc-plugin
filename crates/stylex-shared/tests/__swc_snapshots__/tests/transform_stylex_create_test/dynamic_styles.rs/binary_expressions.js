import * as stylex from '@stylexjs/stylex';
const _temp = {
    kzqmXN: "x1bl4301",
    kZKoxP: "x1f5funs",
    kogj98: "x1cpkpif",
    kmVPX3: "x6rcfto",
    $$css: true
};
export const styles = {
    root: (width, height)=>[
            _temp,
            {
                "--width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width + 100),
                "--height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(height * 2),
                "--margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width - 50),
                "--padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(height / 2)
            }
        ]
};
