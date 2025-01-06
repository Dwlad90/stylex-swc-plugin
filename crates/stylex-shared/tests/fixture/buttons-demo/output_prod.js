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
    x1p0kudt: "x8j0i83 "
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
