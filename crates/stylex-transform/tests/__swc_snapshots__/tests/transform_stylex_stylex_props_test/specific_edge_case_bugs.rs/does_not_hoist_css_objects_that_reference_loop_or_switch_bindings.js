import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
const styles = {
    root: {
        k1xSpc: "x78zum5",
        $$css: true
    }
};
export const out = [];
for (const className of classNames){
    out.push(stylex.props(styles.root, className && {
        $$css: true,
        __: className
    }));
}
switch(kind){
    case 'a':
        let name = getName();
        out.push(stylex.props(styles.root, name && {
            $$css: true,
            __: name
        }));
}
