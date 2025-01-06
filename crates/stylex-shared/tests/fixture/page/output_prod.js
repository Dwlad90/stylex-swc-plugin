import * as stylex from "@stylexjs/stylex";
import React from "react";
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
        className: "x1e2nbdu x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 x1lmef92 x1swossr xif65rj"
    };
    return <main className={className} style={style}>
      Main
    </main>;
}
