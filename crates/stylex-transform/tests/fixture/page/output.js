import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import React from "react";
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".xdt5ytf{flex-direction:column}",
    priority: 3000
});
_inject2({
    ltr: ".x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".x1qughib{justify-content:space-between}",
    priority: 3000
});
_inject2({
    ltr: ".xg6iff7{min-height:100vh}",
    priority: 4000
});
_inject2({
    ltr: ".x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}",
    priority: 1000
});
_inject2({
    ltr: ".x1swossr{line-height:1.3em}",
    priority: 3000
});
_inject2({
    ltr: ".xif65rj{font-size:14px}",
    priority: 3000
});
function getStaticProps() {
    return {
        props: {}
    };
}
const { foo, ...a } = {
    foo: "bar",
    baz: "qux"
};
export default function Home() {
    const { className, style } = {
        className: "input__s.main x1e2nbdu x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 x1lmef92 input__s.title x1swossr xif65rj",
        "data-style-src": "tests/fixture/page/input.stylex.js:5; tests/fixture/page/input.stylex.js:19"
    };
    return <main className={className} style={style}>
      Main
    </main>;
}
