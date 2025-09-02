import * as stylex from '@stylexjs/stylex';
const _temp = {
    kzqmXN: "x5lhr3w",
    kZKoxP: "x16ye13r",
    kWkggS: "xl8spv7",
    kMwMTN: "x14rh7hd",
    $$css: true
};
export const styles = {
    root: (width, height, color)=>[
            _temp,
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width + 100 || 200),
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)((height * 2) ?? 300),
                "--x-backgroundColor": (`${color}` || 'red') != null ? `${color}` || 'red' : undefined,
                "--x-color": (-color || 'black') != null ? -color || 'black' : undefined
            }
        ]
};
