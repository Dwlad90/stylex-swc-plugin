import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import React from "react";
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x78zum5{display:flex}", 3000);
_inject2(".xdt5ytf{flex-direction:column}", 3000);
_inject2(".x6s0dn4{align-items:center}", 3000);
_inject2(".x1qughib{justify-content:space-between}", 3000);
_inject2(".xg6iff7{min-height:100vh}", 4000);
_inject2(".x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}", 1000);
_inject2(".x1swossr{line-height:1.3em}", 3000);
_inject2(".xif65rj{font-size:14px}", 3000);
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
        className: "Page__s.main x1e2nbdu x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 x1lmef92 Page__s.title x1swossr xif65rj"
    };
    return <main className={className} style={style}>
      Main
    </main>;
}
