import * as stylex from '@stylexjs/stylex';
const _styles = {
    color: (value)=>[
            {
                kMwMTN: value != null ? "x14rh7hd" : value,
                $$css: true
            },
            {
                "--x-color": value != null ? value : undefined
            }
        ],
    base: {
        k1xSpc: "x78zum5",
        $$css: true
    }
};
export function render(value) {
    const styles = _styles;
    return stylex.props(styles.base, styles.color(value));
}
