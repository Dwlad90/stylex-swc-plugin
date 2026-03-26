import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".x148kuu.x148kuu:where(.x-default-marker:hover *){background-color:red}",
    priority: 3011.3
});
_inject2({
    ltr: ".xpijypl.xpijypl:where(.x-default-marker:focus ~ *){background-color:green}",
    priority: 3031.5
});
_inject2({
    ltr: ".xoev4mv.xoev4mv:where(.x-default-marker:active ~ *, :has(~ .x-default-marker:active)){background-color:yellow}",
    priority: 3021.7
});
_inject2({
    ltr: ".x1v1vkh3.x1v1vkh3:where(:has(~ .x-default-marker:focus)){background-color:purple}",
    priority: 3041.5
});
_inject2({
    ltr: ".x9zntq3.x9zntq3:where(:has(.x-default-marker:focus)){background-color:orange}",
    priority: 3016.5
});
const styles = {
    container: {
        kWkggS: "x1t391ir x148kuu xpijypl xoev4mv x1v1vkh3 x9zntq3",
        $$css: true
    }
};
console.log(styles.container);
