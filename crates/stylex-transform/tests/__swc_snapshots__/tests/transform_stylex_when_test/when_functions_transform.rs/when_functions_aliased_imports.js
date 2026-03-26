import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import { when as w, create } from '@stylexjs/stylex';
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
const styles = {
    container: {
        kWkggS: "x1t391ir x148kuu xpijypl",
        $$css: true
    }
};
console.log(styles.container);
