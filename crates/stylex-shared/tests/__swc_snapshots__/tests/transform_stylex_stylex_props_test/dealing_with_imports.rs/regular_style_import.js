import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
import { someStyle } from './otherFile';
_inject2({
    ltr: ".x1mqxbix{color:black}",
    priority: 3000
});
const styles = {
    default: {
        kMwMTN: "x1mqxbix",
        $$css: true
    }
};
<div {...stylex.props(styles.default, someStyle)}/>;
