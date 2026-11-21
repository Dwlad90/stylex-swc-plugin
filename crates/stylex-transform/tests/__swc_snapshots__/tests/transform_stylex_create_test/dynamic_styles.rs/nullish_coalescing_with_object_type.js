import * as stylex from '@stylexjs/stylex';
export const styles = {
    fn: (opt: {
        height?: number;
    })=>[
            (opt.height ?? null) !== undefined && {
                kZKoxP: (opt.height ?? null) != null ? "x16ye13r" : "",
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(opt.height ?? null)
            }
        ]
};
