import * as stylex from "@stylexjs/stylex";
import React from "react";
const _stylex$props = {
    main: {
        color: "x1e2nbdu",
        display: "x78zum5",
        flexDirection: "xdt5ytf",
        alignItems: "x6s0dn4",
        justifyContent: "x1qughib",
        minHeight: "xg6iff7",
        padding: "x1lmef92",
        paddingInline: null,
        paddingStart: null,
        paddingLeft: null,
        paddingEnd: null,
        paddingRight: null,
        paddingBlock: null,
        paddingTop: null,
        paddingBottom: null,
        $$css: true
    },
    title: {
        lineHeight: "x1swossr",
        fontSize: "xif65rj",
        $$css: true
    }
};
export default function Home() {
    const { className, style } = stylex.props(s.main, s.title);
    return <main className={className} style={style}>

      Main

    </main>;
}
