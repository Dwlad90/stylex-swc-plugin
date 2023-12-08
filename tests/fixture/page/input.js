import * as stylex from "@stylexjs/stylex";
import React from "react";

const s = stylex.create({
  main: {
    color: "red",
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "space-between",
    minHeight: "100vh",
    padding: "calc((100% - 50px) * 0.5) var(--rightpadding, 20px)",
    // paddingTop: spacing.xxl,
    // paddingBottom: {
    //   default: spacing.xxl,
    //   [MEDIA_MOBILE]: spacing.md,
    // },
  },
  title: {
    lineHeight: "1.3em",
    fontSize: "14px",
  },
  // optional: {
  //   backgroundColor: "lightblue",
  // },
  // optiona2: {
  //   backgroundColor: "lightgray",
  // },
  // optiona3: {
  //   backgroundColor: "lightred",
  // },
});

const optional = false;
const optional2 = false;
const optional3 = false;

export default function Home() {
  const { className, style } = stylex.props(
    s.main,
    s.title,
    // optional && s.optional,
    // optional2 && s.optiona2,
    // optional3 && s.optiona3,
  );

  return (
    <main className={className} style={style}>
      Main
    </main>
  );
}
