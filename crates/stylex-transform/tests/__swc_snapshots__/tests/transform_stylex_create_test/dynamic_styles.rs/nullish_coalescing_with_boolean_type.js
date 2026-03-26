import * as stylex from '@stylexjs/stylex';
export const styles = {
    fn: (opt?: {
        isPressed: boolean;
    })=>[
            {
                kI3sdo: {
                    true: 'red',
                    false: 'blue'
                }[String(!!opt?.isPressed)] != null ? "xem5pho" : ({
                    true: 'red',
                    false: 'blue'
                })[String(!!opt?.isPressed)],
                $$css: true
            },
            {
                "--x-outline": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)({
                    true: 'red',
                    false: 'blue'
                }[String(!!opt?.isPressed)])
            }
        ]
};
