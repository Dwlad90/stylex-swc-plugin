import * as stylex from '@stylexjs/stylex';
export const _styles = 1;
const _styles2 = {
    color: (v)=>[
            v !== undefined && {
                kMwMTN: v != null ? "x14rh7hd" : "",
                $$css: true
            },
            {
                "--x-color": v != null ? v : undefined
            }
        ]
};
export function render(value) {
    const styles = _styles2;
    return stylex.props(styles.color(value));
}
