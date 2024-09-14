"use client";
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./ButtonTokens.stylex";
import * as stylex from "@stylexjs/stylex";
import { buttonTokens } from "./ButtonTokens.stylex";
import ThemeableButton from "./ThemeableButton";
export default function ButtonsDemo(_props) {
    const onClick = ()=>{
        console.log("click");
    };
    return <div {...{
        className: "Page__styles.container x78zum5 xdt5ytf x6s0dn4 xl56j7k xou54vl xzk7aed"
    }}>
      <ThemeableButton onClick={onClick}>Vanilla Button</ThemeableButton>

      <ThemeableButton onClick={onClick} style={styles.bordered}>
        Bordered Button
      </ThemeableButton>

      <ThemeableButton onClick={onClick} theme={redTheme}>
        Red Button
      </ThemeableButton>

      <div {...stylex.props(redTheme)}>
        <ThemeableButton onClick={onClick}>
          Red Button By inheritance
        </ThemeableButton>
      </div>

      <ThemeableButton onClick={onClick} style={[
        styles.bordered,
        styles.greenBorder
    ]} theme={redTheme}>
        Red - Bordered Button
      </ThemeableButton>
    </div>;
}
_inject2(".x8j0i83, .x8j0i83:root{--x1tvn83n:red;--xlb9c25:4px;--xk30bbq:4px;--xkhjxis:8px;--xte9ugm:white;}", 0.5);
const redTheme = {
    Page__redTheme: "Page__redTheme",
    $$css: true,
    "var(--x1p0kudt)": "x8j0i83"
};
_inject2(".x78zum5{display:flex}", 3000);
_inject2(".xdt5ytf{flex-direction:column}", 3000);
_inject2(".x6s0dn4{align-items:center}", 3000);
_inject2(".xl56j7k{justify-content:center}", 3000);
_inject2(".xou54vl{gap:16px}", 2000);
_inject2(".xzk7aed{padding-bottom:64px}", 4000);
_inject2(".xdh2fpr{border-width:2px}", 2000);
_inject2(".x1y0btm7{border-style:solid}", 2000);
_inject2(".x71xlcl{border-color:red}", 2000);
_inject2(".x1bg2uv5{border-color:green}", 2000);
const styles = {
    bordered: {
        "Page__styles.bordered": "Page__styles.bordered",
        borderWidth: "xdh2fpr",
        borderInlineWidth: null,
        borderInlineStartWidth: null,
        borderLeftWidth: null,
        borderInlineEndWidth: null,
        borderRightWidth: null,
        borderBlockWidth: null,
        borderTopWidth: null,
        borderBottomWidth: null,
        borderStyle: "x1y0btm7",
        borderInlineStyle: null,
        borderInlineStartStyle: null,
        borderLeftStyle: null,
        borderInlineEndStyle: null,
        borderRightStyle: null,
        borderBlockStyle: null,
        borderTopStyle: null,
        borderBottomStyle: null,
        borderColor: "x71xlcl",
        borderInlineColor: null,
        borderInlineStartColor: null,
        borderLeftColor: null,
        borderInlineEndColor: null,
        borderRightColor: null,
        borderBlockColor: null,
        borderTopColor: null,
        borderBottomColor: null,
        $$css: true
    },
    greenBorder: {
        "Page__styles.greenBorder": "Page__styles.greenBorder",
        borderColor: "x1bg2uv5",
        borderInlineColor: null,
        borderInlineStartColor: null,
        borderLeftColor: null,
        borderInlineEndColor: null,
        borderRightColor: null,
        borderBlockColor: null,
        borderTopColor: null,
        borderBottomColor: null,
        $$css: true
    }
};
