import * as stylex from '@stylexjs/stylex';
export let render;
const _styles = {
    color: (value)=>[
            value !== undefined && {
                kMwMTN: value != null ? "x14rh7hd" : "",
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
{
    const styles = _styles;
    render = (value)=>stylex.props(styles.base, styles.color(value));
}
