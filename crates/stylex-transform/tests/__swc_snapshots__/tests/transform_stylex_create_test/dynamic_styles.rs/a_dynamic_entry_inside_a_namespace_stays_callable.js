import * as stylex from '@stylexjs/stylex';
const _styles = {
    color: (value: string)=>[
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
export namespace Demo {
    const styles = _styles;
    export function render() {
        return stylex.props(styles.base, styles.color('red'));
    }
}
