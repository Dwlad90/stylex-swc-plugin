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
    xhq9i64: "x1idg7kz ",
    $$css: true
};
const styles = {
    container: {
        k1xSpc: "x78zum5",
        kXwgrk: "xdt5ytf",
        kGNEyG: "x6s0dn4",
        kjj79g: "xl56j7k",
        kOIVth: "xou54vl",
        khm7nJ: null,
        k1C7PZ: null,
        kGO01o: "xzk7aed",
        $$css: true
    },
    bordered: {
        kMzoRj: "xdh2fpr",
        kjGldf: null,
        k2ei4v: null,
        kZ1KPB: null,
        ke9TFa: null,
        kWqL5O: null,
        kLoX6v: null,
        kEafiO: null,
        kt9PQ7: null,
        ksu8eU: "x1y0btm7",
        kJRH4f: null,
        kVhnKS: null,
        k4WBpm: null,
        k8ry5P: null,
        kSWEuD: null,
        kDUl1X: null,
        kPef9Z: null,
        kfdmCh: null,
        kVAM5u: "x71xlcl",
        kzOINU: null,
        kGJrpR: null,
        kaZRDh: null,
        kBCPoo: null,
        k26BEO: null,
        k5QoK5: null,
        kLZC3w: null,
        kL6WhQ: null,
        $$css: true
    },
    greenBorder: {
        kVAM5u: "x1bg2uv5",
        kzOINU: null,
        kGJrpR: null,
        kaZRDh: null,
        kBCPoo: null,
        k26BEO: null,
        k5QoK5: null,
        kLZC3w: null,
        kL6WhQ: null,
        $$css: true
    }
};
const priorityIntent = {
    xhq9i64: "x13g83z8 ",
    $$css: true
};
const defaultIntent = {
    xhq9i64: "x13xmhq7 ",
    $$css: true
};
const intents = {
    priority: priorityIntent,
    default: defaultIntent
};
