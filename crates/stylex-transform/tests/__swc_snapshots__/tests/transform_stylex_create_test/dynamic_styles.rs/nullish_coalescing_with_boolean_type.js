import * as stylex from '@stylexjs/stylex';
export const styles = {
    fn: (opt?: {
        isPressed: boolean;
    })=>[
            {
                true: 'red',
                false: 'blue'
            }[String(!!opt?.isPressed)] !== undefined && {
                kI3sdo: ({
                    true: 'red',
                    false: 'blue'
                })[String(!!opt?.isPressed)] != null ? "xem5pho" : "",
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
