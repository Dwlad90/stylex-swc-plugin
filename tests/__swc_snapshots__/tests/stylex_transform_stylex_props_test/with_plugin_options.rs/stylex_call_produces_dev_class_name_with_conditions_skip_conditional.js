import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    default: {
        "FooBar__styles.default": "FooBar__styles.default",
        color: "x1e2nbdu",
        $$css: true
    }
};
_inject2(".x1t391ir{background-color:blue}", 3000);
const otherStyles = {
    default: {
        "FooBar__otherStyles.default": "FooBar__otherStyles.default",
        backgroundColor: "x1t391ir",
        $$css: true
    }
};
stylex.props([
    styles.default,
    isActive && otherStyles.default
]);
