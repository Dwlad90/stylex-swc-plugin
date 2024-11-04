//__stylex_metadata_start__[{"class_name":"x8j0i83","style":{"rtl":null,"ltr":".x8j0i83, .x8j0i83:root{--x1tvn83n:red;--xlb9c25:4px;--xk30bbq:4px;--xkhjxis:8px;--xte9ugm:white;}"},"priority":0.5},{"class_name":"display-x78zum5","style":{"rtl":null,"ltr":".display-x78zum5{display:flex}"},"priority":3000},{"class_name":"flexDirection-xdt5ytf","style":{"rtl":null,"ltr":".flexDirection-xdt5ytf{flex-direction:column}"},"priority":3000},{"class_name":"alignItems-x6s0dn4","style":{"rtl":null,"ltr":".alignItems-x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"justifyContent-xl56j7k","style":{"rtl":null,"ltr":".justifyContent-xl56j7k{justify-content:center}"},"priority":3000},{"class_name":"gap-xou54vl","style":{"rtl":null,"ltr":".gap-xou54vl{gap:16px}"},"priority":2000},{"class_name":"paddingBottom-xzk7aed","style":{"rtl":null,"ltr":".paddingBottom-xzk7aed{padding-bottom:64px}"},"priority":4000},{"class_name":"borderWidth-xdh2fpr","style":{"rtl":null,"ltr":".borderWidth-xdh2fpr{border-width:2px}"},"priority":2000},{"class_name":"borderStyle-x1y0btm7","style":{"rtl":null,"ltr":".borderStyle-x1y0btm7{border-style:solid}"},"priority":2000},{"class_name":"borderColor-x71xlcl","style":{"rtl":null,"ltr":".borderColor-x71xlcl{border-color:red}"},"priority":2000},{"class_name":"borderColor-x1bg2uv5","style":{"rtl":null,"ltr":".borderColor-x1bg2uv5{border-color:green}"},"priority":2000}]__stylex_metadata_end__
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
        className: "Page__styles.container display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-xl56j7k gap-xou54vl paddingBottom-xzk7aed"
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
    bordered: {
        "Page__styles.bordered": "Page__styles.bordered",
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
        $$css: true
    },
    greenBorder: {
        "Page__styles.greenBorder": "Page__styles.greenBorder",
        borderColor: "borderColor-x1bg2uv5",
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
