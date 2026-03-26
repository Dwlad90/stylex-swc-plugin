import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import React from "react";
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".display-x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".flexDirection-xdt5ytf{flex-direction:column}",
    priority: 3000
});
_inject2({
    ltr: ".alignItems-x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".justifyContent-x1qughib{justify-content:space-between}",
    priority: 3000
});
_inject2({
    ltr: ".minHeight-xg6iff7{min-height:100vh}",
    priority: 4000
});
_inject2({
    ltr: ".padding-x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}",
    priority: 1000
});
_inject2({
    ltr: ".lineHeight-x1swossr{line-height:1.3em}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-xif65rj{font-size:14px}",
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
        className: "color-x1e2nbdu display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-x1qughib minHeight-xg6iff7 padding-x1lmef92 lineHeight-x1swossr fontSize-xif65rj",
        "data-style-src": "tests/fixture/page/input.stylex.js:4; tests/fixture/page/input.stylex.js:13"
    };
    return <main className={className} style={style}>
      Main
    </main>;
}
