import * as stylex from '@stylexjs/stylex';
const _styles = {
    box: (value)=>[
            {
                kzqmXN: value != null ? "x5lhr3w" : value,
                kogj98: gap != null ? "xb9ncqk" : gap,
                $$css: true
            },
            {
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(value),
                "--x-margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(gap)
            }
        ]
};
export function render(gap) {
    const styles = _styles;
    return stylex.props(styles.box(1));
}
