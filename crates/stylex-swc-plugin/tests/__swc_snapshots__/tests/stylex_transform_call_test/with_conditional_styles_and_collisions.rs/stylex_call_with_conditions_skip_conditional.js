//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"xju2f9n","style":{"rtl":null,"ltr":".xju2f9n{color:blue}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".xrkmrrc{background-color:red}", 3000);
_inject2(".xju2f9n{color:blue}", 3000);
const styles = {
    default: {
        backgroundColor: "xrkmrrc",
        $$css: true
    },
    active: {
        color: "xju2f9n",
        $$css: true
    }
};
stylex(styles.default, isActive && styles.active);
