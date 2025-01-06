import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x1t391ir{background-color:blue}", 3000);
_inject2(".x1prwzq3{color:green}", 3000);
const styles = {
    red: {
        color: "x1e2nbdu",
        $$css: true
    },
    blue: {
        backgroundColor: "x1t391ir",
        $$css: true
    },
    green: {
        color: "x1prwzq3",
        $$css: true
    }
};
const stylesArr = [
    styles.red,
    styles.blue,
    ...[
        styles.green
    ]
];
stylex.props(...stylesArr);
