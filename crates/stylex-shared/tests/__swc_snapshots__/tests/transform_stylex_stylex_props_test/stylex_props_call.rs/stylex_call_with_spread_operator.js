import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".x1prwzq3{color:green}",
    priority: 3000
});
const styles = {
    red: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    },
    blue: {
        kWkggS: "x1t391ir",
        $$css: true
    },
    green: {
        kMwMTN: "x1prwzq3",
        $$css: true
    }
};
stylex.props(...[
    styles.red,
    styles.blue,
    ...[
        styles.green
    ]
]);
