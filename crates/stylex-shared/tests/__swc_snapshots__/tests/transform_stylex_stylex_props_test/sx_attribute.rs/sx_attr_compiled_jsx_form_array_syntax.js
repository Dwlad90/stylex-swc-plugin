import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".borderRadius-x12oqio5{border-radius:4px}",
    priority: 2000
});
_inject2({
    ltr: ".backgroundColor-x1t391ir{background-color:blue}",
    priority: 3000
});
function App() {
    return _jsx("div", {
        ...{
            className: "borderRadius-x12oqio5 backgroundColor-x1t391ir",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3; npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
        },
        children: "Hello World"
    });
}
