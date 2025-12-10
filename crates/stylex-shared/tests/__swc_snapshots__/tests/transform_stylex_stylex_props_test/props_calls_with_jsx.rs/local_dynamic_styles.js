import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".color-x1e2nbdu{color:red}", 3000);
_inject2(".opacity-xb4nw82{opacity:var(--x-opacity)}", 3000);
_inject2('@property --x-opacity { syntax: "*"; inherits: false; }', 0);
const styles = {
    red: {
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    },
    opacity: (opacity)=>[
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
    return <div id="test" {...stylex.props(styles.red, styles.opacity(1))}>
          Hello World
        </div>;
}
