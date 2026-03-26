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
_inject2({
    ltr: ".backgroundColor-xrkmrrc{background-color:red}",
    priority: 3000
});
_inject2({
    ltr: ".color-x1awj2ng{color:white}",
    priority: 3000
});
_inject2({
    ltr: ".display-x1lliihq{display:block}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-x1j61zf2{font-size:16px}",
    priority: 3000
});
_inject2({
    ltr: ".paddingBottom-xsag5q8{padding-bottom:12px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingLeft-x5tiur9{padding-left:20px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingRight-x1s7jvk7{padding-right:20px}",
    priority: 4000
});
_inject2({
    ltr: ".paddingTop-xz9dl7a{padding-top:12px}",
    priority: 4000
});
const styles = {
    primary: {
        backgroundColor: "backgroundColor-xrkmrrc",
        color: "color-x1awj2ng",
        $$css: "tests/fixture/use-memo/input.stylex.js:32"
    },
    root: {
        display: "display-x1lliihq",
        fontSize: "fontSize-x1j61zf2",
        paddingBottom: "paddingBottom-xsag5q8",
        paddingLeft: "paddingLeft-x5tiur9",
        paddingRight: "paddingRight-x1s7jvk7",
        paddingTop: "paddingTop-xz9dl7a",
        $$css: "tests/fixture/use-memo/input.stylex.js:36"
    }
};
var _c;
$RefreshReg$(_c, "Button");
