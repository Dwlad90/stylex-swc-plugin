import * as stylex from '@stylexjs/stylex';
const _styles2 = {
    color: (v)=>[
            {
                kMwMTN: v != null ? "x14rh7hd" : v,
                $$css: true
            },
            {
                "--x-color": v != null ? v : undefined
            }
        ]
};
export const _styles = 1;
export function render(value) {
    const styles = _styles2;
    return stylex.props(styles.color(value));
}
