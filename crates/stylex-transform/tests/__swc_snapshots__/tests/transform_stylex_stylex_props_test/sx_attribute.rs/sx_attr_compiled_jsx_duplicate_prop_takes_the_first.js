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
const styles = {
    card: {
        "Foo__styles.card": "Foo__styles.card",
        "borderRadius-kaIpWk": "borderRadius-x12oqio5",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
    }
};
function App() {
    return _jsx("div", {
        ...{
            className: "Foo__styles.main color-x1e2nbdu",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
        },
        ["sx"]: styles.card,
        children: "Hello World"
    });
}
