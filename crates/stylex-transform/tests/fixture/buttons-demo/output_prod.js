"use client";
import * as stylex from "@stylexjs/stylex";
import { buttonTokens } from "./ButtonTokens.stylex";
import ThemeableButton from "./ThemeableButton";
export default function ButtonsDemo(props) {
    const onClick = ()=>{
        console.log("click");
    };
    return <div {...stylex.props(styles.container, intents[props.intent], intentsFn()[props.intent])}>
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
    xhq9i64: "x1idg7kz xhq9i64",
    $$css: true
};
const styles = {
    container: {
        k1xSpc: "x78zum5",
        kXwgrk: "xdt5ytf",
        kGNEyG: "x6s0dn4",
        kjj79g: "xl56j7k",
        kOIVth: "xou54vl",
        kGO01o: "xzk7aed",
        $$css: true
    },
    bordered: {
        kMzoRj: "xdh2fpr",
        ksu8eU: "x1y0btm7",
        kVAM5u: "x71xlcl",
        $$css: true
    },
    greenBorder: {
        kVAM5u: "x1bg2uv5",
        $$css: true
    }
};
const priorityIntent = {
    xhq9i64: "x13g83z8 xhq9i64",
    $$css: true
};
const defaultIntent = {
    xhq9i64: "x13xmhq7 xhq9i64",
    $$css: true
};
const intents = {
    priority: priorityIntent,
    default: defaultIntent
};
const intentsFn = ()=>({
        priority: priorityIntent,
        default: defaultIntent
    });
