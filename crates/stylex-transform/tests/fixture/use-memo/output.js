import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
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
        fileName: "/root/app/components/Button.tsx",
        lineNumber: 22,
        columnNumber: 10
    }, this);
}
_s(Button, "XPeb32THZfEWB+gFvzI8fl0TbTY=");
_c = Button;
_inject2({
    ltr: ".xrkmrrc{background-color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x1awj2ng{color:white}",
    priority: 3000
});
_inject2({
    ltr: ".x1lliihq{display:block}",
    priority: 3000
});
_inject2({
    ltr: ".x1j61zf2{font-size:16px}",
    priority: 3000
});
_inject2({
    ltr: ".xsag5q8{padding-bottom:12px}",
    priority: 4000
});
_inject2({
    ltr: ".x5tiur9{padding-left:20px}",
    priority: 4000
});
_inject2({
    ltr: ".x1s7jvk7{padding-right:20px}",
    priority: 4000
});
_inject2({
    ltr: ".xz9dl7a{padding-top:12px}",
    priority: 4000
});
const styles = {
    primary: {
        "input__styles.primary": "input__styles.primary",
        backgroundColor: "xrkmrrc",
        color: "x1awj2ng",
        $$css: "tests/fixture/use-memo/input.stylex.js:33"
    },
    root: {
        "input__styles.root": "input__styles.root",
        display: "x1lliihq",
        fontSize: "x1j61zf2",
        paddingBottom: "xsag5q8",
        paddingLeft: "x5tiur9",
        paddingRight: "x1s7jvk7",
        paddingTop: "xz9dl7a",
        $$css: "tests/fixture/use-memo/input.stylex.js:37"
    }
};
var _c;
$RefreshReg$(_c, "Button");
