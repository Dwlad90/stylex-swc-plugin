//__stylex_metadata_start__[{"class_name":"x8j0i83","style":{"rtl":null,"ltr":".x8j0i83, .x8j0i83:root{--x1tvn83n:red;--xlb9c25:4px;--xk30bbq:4px;--xkhjxis:8px;--xte9ugm:white;}"},"priority":0.5},{"class_name":"x78zum5","style":{"rtl":null,"ltr":".x78zum5{display:flex}"},"priority":3000},{"class_name":"xdt5ytf","style":{"rtl":null,"ltr":".xdt5ytf{flex-direction:column}"},"priority":3000},{"class_name":"x6s0dn4","style":{"rtl":null,"ltr":".x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"xl56j7k","style":{"rtl":null,"ltr":".xl56j7k{justify-content:center}"},"priority":3000},{"class_name":"xou54vl","style":{"rtl":null,"ltr":".xou54vl{gap:16px}"},"priority":2000},{"class_name":"xzk7aed","style":{"rtl":null,"ltr":".xzk7aed{padding-bottom:64px}"},"priority":4000},{"class_name":"xdh2fpr","style":{"rtl":null,"ltr":".xdh2fpr{border-width:2px}"},"priority":2000},{"class_name":"x1y0btm7","style":{"rtl":null,"ltr":".x1y0btm7{border-style:solid}"},"priority":2000},{"class_name":"x71xlcl","style":{"rtl":null,"ltr":".x71xlcl{border-color:red}"},"priority":2000},{"class_name":"x1bg2uv5","style":{"rtl":null,"ltr":".x1bg2uv5{border-color:green}"},"priority":2000}]__stylex_metadata_end__
"use client";
import * as stylex from "@stylexjs/stylex";
import { buttonTokens } from "./ButtonTokens.stylex";
import ThemeableButton from "./ThemeableButton";
export default function ButtonsDemo(_props) {
    const onClick = ()=>{
        console.log("click");
    };
    return <div {...{
        className: "x78zum5 xdt5ytf x6s0dn4 xl56j7k xou54vl xzk7aed"
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
const redTheme = {
    $$css: true,
    "var(--x1p0kudt)": "x8j0i83"
};
const styles = {
    bordered: {
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
