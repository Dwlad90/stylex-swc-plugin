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
    return <div {...stylex.props(styles.container, intents[props.intent], intentsFn()[props.intent])}>
      <ThemeableButton onClick={onClick}>Vanilla Button</ThemeableButton>

      <ThemeableButton onClick={onClick} style={styles.bordered}>
        Bordered Button
      </ThemeableButton>

      <ThemeableButton onClick={onClick} theme={redTheme}>
        Red Button
      </ThemeableButton>

      <div className="input__redTheme x1582kvi xhq9i64">
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
_inject2({
    ltr: ".x1582kvi, .x1582kvi:root{--bgColor-xxn4pt7:red;--cornerRadius-xebqz1:4px;--paddingBlock-x9c4myw:4px;--paddingInline-xscmh3t:8px;--textColor-xnblhe2:white;}",
    priority: 0.5
});
const redTheme = {
    input__redTheme: "input__redTheme",
    xhq9i64: "x1582kvi xhq9i64",
    $$css: true
};
_inject2({
    ltr: ".display-x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".flexDirection-xdt5ytf{flex-direction:column}",
    priority: 3000
});
_inject2({
    ltr: ".alignItems-x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".justifyContent-xl56j7k{justify-content:center}",
    priority: 3000
});
_inject2({
    ltr: ".gap-xou54vl{gap:16px}",
    priority: 2000
});
_inject2({
    ltr: ".paddingBottom-xzk7aed{padding-bottom:64px}",
    priority: 4000
});
_inject2({
    ltr: ".borderWidth-xdh2fpr{border-width:2px}",
    priority: 2000
});
_inject2({
    ltr: ".borderStyle-x1y0btm7{border-style:solid}",
    priority: 2000
});
_inject2({
    ltr: ".borderColor-x71xlcl{border-color:red}",
    priority: 2000
});
_inject2({
    ltr: ".borderColor-x1bg2uv5{border-color:green}",
    priority: 2000
});
const styles = {
    container: {
        display: "display-x78zum5",
        flexDirection: "flexDirection-xdt5ytf",
        alignItems: "alignItems-x6s0dn4",
        justifyContent: "justifyContent-xl56j7k",
        gap: "gap-xou54vl",
        paddingBottom: "paddingBottom-xzk7aed",
        $$css: "tests/fixture/buttons-demo/input.stylex.js:42"
    },
    bordered: {
        borderWidth: "borderWidth-xdh2fpr",
        borderStyle: "borderStyle-x1y0btm7",
        borderColor: "borderColor-x71xlcl",
        $$css: "tests/fixture/buttons-demo/input.stylex.js:50"
    },
    greenBorder: {
        borderColor: "borderColor-x1bg2uv5",
        $$css: "tests/fixture/buttons-demo/input.stylex.js:55"
    }
};
_inject2({
    ltr: ".xm1pwqw, .xm1pwqw:root{--background-x166rmrk:#000;--text-x1rr8s3j:#fff;}",
    priority: 0.5
});
const priorityIntent = {
    input__priorityIntent: "input__priorityIntent",
    xhq9i64: "xm1pwqw xhq9i64",
    $$css: true
};
_inject2({
    ltr: ".x1h9f7e8, .x1h9f7e8:root{--background-x166rmrk:#000000;--text-x1rr8s3j:#555555;}",
    priority: 0.5
});
const defaultIntent = {
    input__defaultIntent: "input__defaultIntent",
    xhq9i64: "x1h9f7e8 xhq9i64",
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
