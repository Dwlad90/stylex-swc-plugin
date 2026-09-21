import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    main: {
        "Foo__styles.main": "Foo__styles.main",
        "color-kMwMTN": "x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    }
};
function App(name) {
    const _el$ = _$createElement("div");
    _$setAttribute(_el$, "sx");
    _$setAttribute(_el$, name, styles.main);
    _$setAttribute(_el$, "class", styles.main);
    return _el$;
}
