import { jsxDEV as _jsxDEV } from "react/jsx-dev-runtime";
var _s = $RefreshSig$();
import stylex from "@stylexjs/stylex";
import { useMemo } from "react";
export default function Button(param) {
    let { variant = "primary" } = param;
    _s();
    const colourStyle = useMemo({
        "Button.useMemo[colourStyle]": ()=>{
            return [
                variant === "primary" && styles.primary
            ];
        }
    }["Button.useMemo[colourStyle]"], [
        variant
    ]);
    return /*#__PURE__*/ _jsxDEV("button", {
        ...stylex.props([
            styles.root,
            colourStyle
        ]),
        children: "Test"
    }, void 0, false, {
        fileName: "/Users/vladislavbuinovski/Downloads/nextjs-app-dir-stylex-15/app/components/Button.tsx",
        lineNumber: 22,
        columnNumber: 10
    }, this);
}
_s(Button, "XPeb32THZfEWB+gFvzI8fl0TbTY=");
_c = Button;
const styles = {
    primary: {
        backgroundColor: "xrkmrrc",
        color: "x1awj2ng",
        $$css: true
    },
    root: {
        display: "x1lliihq",
        fontSize: "x1j61zf2",
        paddingBottom: "xsag5q8",
        paddingLeft: "x5tiur9",
        paddingRight: "x1s7jvk7",
        paddingInlineStart: null,
        paddingInlineEnd: null,
        paddingTop: "xz9dl7a",
        $$css: true
    }
};
var _c;
$RefreshReg$(_c, "Button");
