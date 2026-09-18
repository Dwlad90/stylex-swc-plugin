import * as stylex from '@stylexjs/stylex';
export const styles = {
    root: (color, width)=>[
            {
                kMwMTN: (color ?? null) != null ? "x14rh7hd" : color ?? null,
                kzqmXN: width != null ? "x5lhr3w" : width,
                $$css: true
            },
            {
                "--x-color": (color ?? null) != null ? color ?? null : undefined,
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(width)
            }
        ]
};
