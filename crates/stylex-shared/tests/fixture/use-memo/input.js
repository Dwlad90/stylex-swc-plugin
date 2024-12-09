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
const styles = stylex.create({
    primary: {
        backgroundColor: "red",
        color: "white"
    },
    root: {
        display: "block",
        fontSize: 16,
        paddingBottom: 12,
        paddingLeft: 20,
        paddingRight: 20,
        paddingTop: 12
    }
});
var _c;
$RefreshReg$(_c, "Button");
