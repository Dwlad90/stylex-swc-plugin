import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
const styles = {
    default: {
        "FooBar__styles.default": "FooBar__styles.default",
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "js/FooBar.react.js:3"
    }
};
_inject2({
    ltr: ".backgroundColor-x1t391ir{background-color:blue}",
    priority: 3000
});
const otherStyles = {
    default: {
        "FooBar__otherStyles.default": "FooBar__otherStyles.default",
        "backgroundColor-kWkggS": "backgroundColor-x1t391ir",
        $$css: "js/FooBar.react.js:8"
    }
};
stylex.props([
    styles.default,
    isActive && otherStyles.default
]);
