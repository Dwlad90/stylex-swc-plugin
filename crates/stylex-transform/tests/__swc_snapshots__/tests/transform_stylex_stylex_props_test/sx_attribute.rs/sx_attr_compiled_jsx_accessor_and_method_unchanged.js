import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    main: {
        "Foo__styles.main": "Foo__styles.main",
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    }
};
function App() {
    return _jsx("div", {
        get sx () {
            return styles.main;
        },
        children: [
            _jsx("span", {
                sx () {
                    return styles.main;
                }
            }),
            _jsx("b", {
                set sx (value){
                    this.value = value;
                }
            })
        ]
    });
}
