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
export class Foo {
    render() {
        return stylex.props(styles.root, this.on && {
            $$css: true,
            a: `x${this.size}`
        });
    }
}
export function Bar() {
    return stylex.props(styles.root, {
        $$css: true,
        [arguments[0]]: 'x'
    });
}
