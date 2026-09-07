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
export const render = (()=>{
    const styles = _styles;
    return (value)=>stylex.props(styles.base, styles.color(value));
})();
