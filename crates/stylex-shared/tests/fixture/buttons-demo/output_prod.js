"use client";
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
const redTheme = {
    $$css: true,
    xhq9i64: "x1idg7kz "
};
const styles = {
    container: {
        display: "x78zum5",
        flexDirection: "xdt5ytf",
        alignItems: "x6s0dn4",
        justifyContent: "xl56j7k",
        gap: "xou54vl",
        rowGap: null,
        columnGap: null,
        paddingBottom: "xzk7aed",
        $$css: true
    },
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
const priorityIntent = {
    $$css: true,
    xhq9i64: "x13g83z8 "
};
const defaultIntent = {
    $$css: true,
    xhq9i64: "x13xmhq7 "
};
const intents = {
    priority: priorityIntent,
    default: defaultIntent
};
