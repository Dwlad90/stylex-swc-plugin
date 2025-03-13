import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
const foo = 'bar';
function bar() {
    const foo = 'baz';
}
_inject2(".color-x1e2nbdu{color:red}", 3000);
export const styles = {
    test: {
        color: "color-x1e2nbdu",
        $$css: true
    }
};
