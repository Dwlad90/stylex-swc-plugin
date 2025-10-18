import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2(".x1t391ir{background-color:blue}", 3000);
_inject2(".x148kuu:where(.x-default-marker:hover *){background-color:red}", 3011.3);
_inject2(".xpijypl:where(.x-default-marker:focus ~ *){background-color:green}", 3021.5);
_inject2(".xoev4mv:where(.x-default-marker:active ~ *, :has(~ .x-default-marker:active)){background-color:yellow}", 3041.7);
_inject2(".x1v1vkh3:where(:has(~ .x-default-marker:focus)){background-color:purple}", 3031.5);
_inject2(".x9zntq3:where(:has(.x-default-marker:focus)){background-color:orange}", 3016.5);
const styles = {
    container: {
        kWkggS: "x1t391ir x148kuu xpijypl xoev4mv x1v1vkh3 x9zntq3",
        $$css: true
    }
};
console.log(styles.container);
