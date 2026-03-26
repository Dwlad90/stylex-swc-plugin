import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
function App() {
    return _jsx("div", {
        ...{
            className: "color-x1e2nbdu",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
        },
        children: "Hello World"
    });
}
