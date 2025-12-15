import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import * as React from "react";
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
const _styles = {
    div: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
_inject2({
    ltr: ".xju2f9n{color:blue}",
    priority: 3000
});
const _styles2 = {
    div: {
        kMwMTN: "xju2f9n",
        $$css: true
    }
};
function Foo() {
    const styles = _styles;
    return <div {...stylex.props(styles.div)}>Hello, foo!</div>;
}
function Bar() {
    const styles = _styles2;
    return <div {...stylex.props(styles.div)}>Hello, bar!</div>;
}
export function App() {
    return <>
          <Foo/>
          <Bar/>
        </>;
}
