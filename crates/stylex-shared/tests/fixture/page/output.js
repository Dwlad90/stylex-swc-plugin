import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import React from "react";
_inject2(".color-x1e2nbdu{color:red}", 3000);
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".flexDirection-xdt5ytf{flex-direction:column}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-x1qughib{justify-content:space-between}", 3000);
_inject2(".minHeight-xg6iff7{min-height:100vh}", 4000);
_inject2(".padding-x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}", 1000);
_inject2(".lineHeight-x1swossr{line-height:1.3em}", 3000);
_inject2(".fontSize-xif65rj{font-size:14px}", 3000);
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
