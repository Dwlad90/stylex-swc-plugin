import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    main: {
        "color-kMwMTN": "x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    }
};
function App() {
    return _jsxs("div", {
        children: [
            _jsx("Div", {
                sx: styles.main
            }),
            _jsx("", {
                sx: styles.main
            })
        ]
    });
}
