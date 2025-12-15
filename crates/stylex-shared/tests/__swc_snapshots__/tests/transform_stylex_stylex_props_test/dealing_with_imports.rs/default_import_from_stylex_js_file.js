import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./constants.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { someStyle, vars } from './constants.stylex.js';
_inject2({
    ltr: ".x1mqxbix{color:black}",
    priority: 3000
});
_inject2({
    ltr: ".xs7t3iu{background-color:var(--x1p9zm3x)}",
    priority: 3000
});
const styles = {
    default: {
        kMwMTN: "x1mqxbix",
        kWkggS: "xs7t3iu",
        $$css: true
    }
};
<div {...stylex.props(styles.default, someStyle)}/>;
