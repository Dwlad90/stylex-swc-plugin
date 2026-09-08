import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    "MyComponent__styles.color": "MyComponent__styles.color",
    $$css: "MyComponent.tsx:5"
};
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
const _styles = {
    color: (value: string)=>[
            _temp,
            {
                "color-kMwMTN": value != null ? "x14rh7hd" : value,
                $$css: "MyComponent.tsx:5"
            },
            {
                "--x-color": value != null ? value : undefined
            }
        ],
    base: {
        "MyComponent__styles.base": "MyComponent__styles.base",
        "display-k1xSpc": "x78zum5",
        $$css: "MyComponent.tsx:6"
    }
};
export namespace Demo {
    const styles = _styles;
    export function render() {
        return stylex.props(styles.base, styles.color('red'));
    }
}
