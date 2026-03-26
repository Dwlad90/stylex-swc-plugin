import * as stylex from '@stylexjs/stylex';
export const styles = {
    fn: (opt: {
        size?: 'xlarge' | 'large' | 'medium' | 'small';
    })=>[
            {
                kaIpWk: {
                    xlarge: 16,
                    large: 12,
                    medium: 8,
                    small: 8
                }[opt?.size ?? 'large'] != null ? "x7yrpt8" : ({
                    xlarge: 16,
                    large: 12,
                    medium: 8,
                    small: 8
                })[opt?.size ?? 'large'],
                $$css: true
            },
            {
                "--x-borderRadius": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)({
                    xlarge: 16,
                    large: 12,
                    medium: 8,
                    small: 8
                }[opt?.size ?? 'large'])
            }
        ]
};
