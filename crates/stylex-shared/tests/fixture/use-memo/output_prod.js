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
    return _jsxDEV("button", {
        ...stylex.props([
            styles.root,
            colourStyle
        ]),
        children: "Test"
    }, void 0, false, {
        fileName: "/root/app/components/Button.tsx",
        lineNumber: 22,
        columnNumber: 10
    }, this);
}
_s(Button, "XPeb32THZfEWB+gFvzI8fl0TbTY=");
_c = Button;
const styles = {
    primary: {
        kWkggS: "xrkmrrc",
        kMwMTN: "x1awj2ng",
        $$css: true
    },
    root: {
        k1xSpc: "x1lliihq",
        kGuDYH: "x1j61zf2",
        kGO01o: "xsag5q8",
        kE3dHu: "x5tiur9",
        kpe85a: "x1s7jvk7",
        kZCmMZ: null,
        kwRFfy: null,
        kLKAdn: "xz9dl7a",
        $$css: true
    }
};
var _c;
$RefreshReg$(_c, "Button");
