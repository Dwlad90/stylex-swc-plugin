import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".color-x1e2nbdu{color:red}", 3000);
const styles = {
    default: {
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "js/FooBar.react.js:3"
    }
};
_inject2(".backgroundColor-x1t391ir{background-color:blue}", 3000);
const otherStyles = {
    default: {
        "backgroundColor-kWkggS": "backgroundColor-x1t391ir",
        $$css: "js/FooBar.react.js:8"
    }
};
stylex.props([
    styles.default,
    isActive && otherStyles.default
]);
