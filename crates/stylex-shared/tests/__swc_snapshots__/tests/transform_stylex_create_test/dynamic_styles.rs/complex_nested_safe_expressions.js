import * as stylex from '@stylexjs/stylex';
const _temp = {
    kzqmXN: "x1bl4301",
    kZKoxP: "x1f5funs",
    kWkggS: "xr5ldyu",
    kMwMTN: "xfx01vb",
    $$css: true
};
export const styles = {
    root: (width, height, color)=>[
            _temp,
            {
                "--width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width + 100 || 200),
                "--height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)((height * 2) ?? 300),
                "--backgroundColor": (`${color}` || 'red') != null ? `${color}` || 'red' : undefined,
                "--color": (-color || 'black') != null ? -color || 'black' : undefined
            }
        ]
};
