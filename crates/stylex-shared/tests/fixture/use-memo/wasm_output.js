//__stylex_metadata_start__[{"class_name":"backgroundColor-xrkmrrc","style":{"rtl":null,"ltr":".backgroundColor-xrkmrrc{background-color:red}"},"priority":3000},{"class_name":"color-x1awj2ng","style":{"rtl":null,"ltr":".color-x1awj2ng{color:white}"},"priority":3000},{"class_name":"display-x1lliihq","style":{"rtl":null,"ltr":".display-x1lliihq{display:block}"},"priority":3000},{"class_name":"fontSize-x1j61zf2","style":{"rtl":null,"ltr":".fontSize-x1j61zf2{font-size:16px}"},"priority":3000},{"class_name":"paddingBottom-xsag5q8","style":{"rtl":null,"ltr":".paddingBottom-xsag5q8{padding-bottom:12px}"},"priority":4000},{"class_name":"paddingLeft-x5tiur9","style":{"rtl":null,"ltr":".paddingLeft-x5tiur9{padding-left:20px}"},"priority":4000},{"class_name":"paddingRight-x1s7jvk7","style":{"rtl":null,"ltr":".paddingRight-x1s7jvk7{padding-right:20px}"},"priority":4000},{"class_name":"paddingTop-xz9dl7a","style":{"rtl":null,"ltr":".paddingTop-xz9dl7a{padding-top:12px}"},"priority":4000}]__stylex_metadata_end__
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
        fileName: "/Users/vladislavbuinovski/Downloads/nextjs-app-dir-stylex-15/app/components/Button.tsx",
        lineNumber: 22,
        columnNumber: 10
    }, this);
}
_s(Button, "XPeb32THZfEWB+gFvzI8fl0TbTY=");
_c = Button;
_inject2(".backgroundColor-xrkmrrc{background-color:red}", 3000);
_inject2(".color-x1awj2ng{color:white}", 3000);
_inject2(".display-x1lliihq{display:block}", 3000);
_inject2(".fontSize-x1j61zf2{font-size:16px}", 3000);
_inject2(".paddingBottom-xsag5q8{padding-bottom:12px}", 4000);
_inject2(".paddingLeft-x5tiur9{padding-left:20px}", 4000);
_inject2(".paddingRight-x1s7jvk7{padding-right:20px}", 4000);
_inject2(".paddingTop-xz9dl7a{padding-top:12px}", 4000);
const styles = {
    primary: {
        "Page__styles.primary": "Page__styles.primary",
        backgroundColor: "backgroundColor-xrkmrrc",
        color: "color-x1awj2ng",
        $$css: true
    },
    root: {
        "Page__styles.root": "Page__styles.root",
        display: "display-x1lliihq",
        fontSize: "fontSize-x1j61zf2",
        paddingBottom: "paddingBottom-xsag5q8",
        paddingLeft: "paddingLeft-x5tiur9",
        paddingRight: "paddingRight-x1s7jvk7",
        paddingInlineStart: null,
        paddingInlineEnd: null,
        paddingTop: "paddingTop-xz9dl7a",
        $$css: true
    }
};
var _c;
$RefreshReg$(_c, "Button");
