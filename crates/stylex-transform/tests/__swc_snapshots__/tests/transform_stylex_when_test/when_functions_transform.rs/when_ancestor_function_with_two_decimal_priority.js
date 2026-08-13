import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import { when, create } from '@stylexjs/stylex';
_inject2({
    ltr: ".x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".x1ctjlu4.x1ctjlu4:where(.x-default-marker:first-child *){background-color:red}",
    priority: 3010.52
});
_inject2({
    ltr: ".xtt38zc.xtt38zc:where(.x-default-marker:first-of-type *){background-color:green}",
    priority: 3010.53
});
const styles = {
    container: {
        kWkggS: "x1t391ir x1ctjlu4 xtt38zc",
        $$css: true
    }
};
console.log(styles.container);
