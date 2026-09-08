import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
const _temp = {
    "Foo__styles.opacity": "Foo__styles.opacity",
    $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
};
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".opacity-xb4nw82{opacity:var(--x-opacity)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-opacity { syntax: "*"; inherits: false;}',
    priority: 0
});
const styles = {
    red: {
        "Foo__styles.red": "Foo__styles.red",
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    },
    opacity: (opacity)=>[
            _temp,
            {
                "opacity-kSiTet": opacity != null ? "opacity-xb4nw82" : opacity,
                $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
            },
            {
                "--x-opacity": opacity != null ? opacity : undefined
            }
        ]
};
function Foo() {
    return <div {...stylex.props(styles.red, styles.opacity(0.5))}/>;
}
