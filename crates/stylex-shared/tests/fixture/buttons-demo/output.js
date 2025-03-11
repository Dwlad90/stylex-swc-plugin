"use client";
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./ButtonTokens.stylex";
import * as stylex from "@stylexjs/stylex";
import { buttonTokens } from "./ButtonTokens.stylex";
import ThemeableButton from "./ThemeableButton";
export default function ButtonsDemo(props) {
    const onClick = ()=>{
        console.log("click");
    };
    return <div {...stylex.props(styles.container, intents[props.intent])}>
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
_inject2(".x1582kvi, .x1582kvi:root{--bgColor-xxn4pt7:red;--cornerRadius-xebqz1:4px;--paddingBlock-x9c4myw:4px;--paddingInline-xscmh3t:8px;--textColor-xnblhe2:white;}", 0.5);
const redTheme = {
    input__redTheme: "input__redTheme",
    $$css: true,
    "__themeName__-xhq9i64": "x1582kvi "
};
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".flexDirection-xdt5ytf{flex-direction:column}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".gap-xou54vl{gap:16px}", 2000);
_inject2(".paddingBottom-xzk7aed{padding-bottom:64px}", 4000);
_inject2(".borderWidth-xdh2fpr{border-width:2px}", 2000);
_inject2(".borderStyle-x1y0btm7{border-style:solid}", 2000);
_inject2(".borderColor-x71xlcl{border-color:red}", 2000);
_inject2(".borderColor-x1bg2uv5{border-color:green}", 2000);
const styles = {
    container: {
        display: "display-x78zum5",
        flexDirection: "flexDirection-xdt5ytf",
        alignItems: "alignItems-x6s0dn4",
        justifyContent: "justifyContent-xl56j7k",
        gap: "gap-xou54vl",
        rowGap: null,
        columnGap: null,
        paddingBottom: "paddingBottom-xzk7aed",
        $$css: "input.stylex.js:49"
    },
    bordered: {
        borderWidth: "borderWidth-xdh2fpr",
        borderInlineWidth: null,
        borderInlineStartWidth: null,
        borderLeftWidth: null,
        borderInlineEndWidth: null,
        borderRightWidth: null,
        borderBlockWidth: null,
        borderTopWidth: null,
        borderBottomWidth: null,
        borderStyle: "borderStyle-x1y0btm7",
        borderInlineStyle: null,
        borderInlineStartStyle: null,
        borderLeftStyle: null,
        borderInlineEndStyle: null,
        borderRightStyle: null,
        borderBlockStyle: null,
        borderTopStyle: null,
        borderBottomStyle: null,
        borderColor: "borderColor-x71xlcl",
        borderInlineColor: null,
        borderInlineStartColor: null,
        borderLeftColor: null,
        borderInlineEndColor: null,
        borderRightColor: null,
        borderBlockColor: null,
        borderTopColor: null,
        borderBottomColor: null,
        $$css: "input.stylex.js:57"
    },
    greenBorder: {
        borderColor: "borderColor-x1bg2uv5",
        borderInlineColor: null,
        borderInlineStartColor: null,
        borderLeftColor: null,
        borderInlineEndColor: null,
        borderRightColor: null,
        borderBlockColor: null,
        borderTopColor: null,
        borderBottomColor: null,
        $$css: "input.stylex.js:62"
    }
};
_inject2(".xm1pwqw, .xm1pwqw:root{--background-x166rmrk:#000;--text-x1rr8s3j:#fff;}", 0.5);
const priorityIntent = {
    input__priorityIntent: "input__priorityIntent",
    $$css: true,
    "__themeName__-xhq9i64": "xm1pwqw "
};
_inject2(".x1h9f7e8, .x1h9f7e8:root{--background-x166rmrk:#000000;--text-x1rr8s3j:#555555;}", 0.5);
const defaultIntent = {
    input__defaultIntent: "input__defaultIntent",
    $$css: true,
    "__themeName__-xhq9i64": "x1h9f7e8 "
};
const intents = {
    priority: priorityIntent,
    default: defaultIntent
};
