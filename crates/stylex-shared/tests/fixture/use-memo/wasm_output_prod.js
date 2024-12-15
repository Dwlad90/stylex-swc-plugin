//__stylex_metadata_start__[{"class_name":"xrkmrrc","style":{"rtl":null,"ltr":".xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"x1awj2ng","style":{"rtl":null,"ltr":".x1awj2ng{color:white}"},"priority":3000},{"class_name":"x1lliihq","style":{"rtl":null,"ltr":".x1lliihq{display:block}"},"priority":3000},{"class_name":"x1j61zf2","style":{"rtl":null,"ltr":".x1j61zf2{font-size:16px}"},"priority":3000},{"class_name":"xsag5q8","style":{"rtl":null,"ltr":".xsag5q8{padding-bottom:12px}"},"priority":4000},{"class_name":"x5tiur9","style":{"rtl":null,"ltr":".x5tiur9{padding-left:20px}"},"priority":4000},{"class_name":"x1s7jvk7","style":{"rtl":null,"ltr":".x1s7jvk7{padding-right:20px}"},"priority":4000},{"class_name":"xz9dl7a","style":{"rtl":null,"ltr":".xz9dl7a{padding-top:12px}"},"priority":4000}]__stylex_metadata_end__
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
