import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./constants.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { someStyle } from './constants.stylex.js';
_inject2({
    ltr: ".x1mqxbix{color:black}",
    priority: 3000
});
_inject2({
    ltr: ".xbcwliy{background-color:var(--x1y0rcvx)}",
    priority: 3000
});
const styles = {
    default: {
        kMwMTN: "x1mqxbix",
        kWkggS: "xbcwliy",
        $$css: true
    }
};
<div {...stylex.props(styles.default, someStyle.foo)}/>;
