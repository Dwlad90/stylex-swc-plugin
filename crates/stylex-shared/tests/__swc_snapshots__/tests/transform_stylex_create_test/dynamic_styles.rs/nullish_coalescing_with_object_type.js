import * as stylex from '@stylexjs/stylex';
export const styles = {
    fn: (opt: {
        height?: number;
    })=>[
            {
                kZKoxP: (opt.height ?? null) != null ? "x16ye13r" : opt.height ?? null,
                $$css: true
            },
            {
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(opt.height ?? null)
            }
        ]
};
