import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".borderRadius-x12oqio5{border-radius:4px}",
    priority: 2000
});
function App() {
    return _createElementBlock("div", {
        ...{
            className: "color-x1e2nbdu",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
        }
    }, [
        _createElementVNode("div", {
            ...{
                className: "borderRadius-x12oqio5",
                "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
            }
        }, "Hello World")
    ]);
}
