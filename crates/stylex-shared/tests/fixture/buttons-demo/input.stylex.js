"use client";

import * as stylex from "@stylexjs/stylex";
import { buttonTokens } from "./ButtonTokens.stylex";
import ThemeableButton from "./ThemeableButton";

export default function ButtonsDemo(props) {
  const onClick = () => {
    console.log("click");
  };
  return (
    <div {...stylex.props(styles.container, intents[props.intent])}>
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

      <ThemeableButton
        onClick={onClick}
        style={[styles.bordered, styles.greenBorder]}
        theme={redTheme}
      >
        Red - Bordered Button
      </ThemeableButton>
    </div>
  );
}

const redTheme = stylex.createTheme(buttonTokens, {
  bgColor: "red",
  textColor: "white",
  cornerRadius: "4px",
  paddingBlock: "4px",
  paddingInline: "8px",
});

const styles = stylex.create({
  container: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    gap: 16,
    paddingBottom: 64,
  },
  bordered: {
    borderWidth: 2,
    borderStyle: "solid",
    borderColor: "red",
  },
  greenBorder: {
    borderColor: "green",
  },
});


const priorityIntent = stylex.createTheme(buttonTokens, {
  background: { default: '#000' },
  text: { default: '#fff' },
});

const defaultIntent = stylex.createTheme(buttonTokens, {
  // backgroundColor: `color-mix(red, white 20%)`, // red but 20% more white
  background: { default: '#000000' },
  text: { default: '#555555' },
});

const intents = {
  priority: priorityIntent,
  default: defaultIntent,
};