import { COMMON_SIZES } from "@/app/components/Test";
import * as stylex from "@stylexjs/stylex";


const stretchAnimation = stylex.keyframes({
  from: {
    transform: "perspective(120px) rotateX(0deg) rotateY(0deg)",
  },
  to: {
    transform: "perspective(120px) rotateX(-180.1deg) rotateY(0deg)",
  },
});

const styles = stylex.create({
  root: {
    display: "grid",
    gridAutoFlow: "column",
    gridTemplateColumns: "repeat(5, 12%)",
    alignItems: "center",
    justifyContent: "center",
    gap: "8%",
  },
  rect: {
    backgroundColor: "#2c3e50",
    borderRadius: "4px",
    zIndex: "100",
    animationName: stretchAnimation,
    animationDuration: "1.2s",
    animationIterationCount: "infinite",
    animationTimingFunction: "ease-in-out",
  },
  rect1: {
    height: "24%",
  },
  rect2: {
    animationDelay: "-1.1s",
    height: "62%",
  },
  rect3: {
    animationDelay: "-1s",
    height: "100%",
  },
  rect4: {
    animationDelay: "-0.9s",
    backgroundColor: "#ff4500",
    height: "62%",
  },
  rect5: {
    animationDelay: "-0.8s",
    height: "24%",
  },
  sizeSmall: {
    height: "2rem",
    width: "2rem",
  },
  size_small: {
    height: "2rem",
    width: "2rem",
  },
  size_normal: {
    height: "4rem",
    width: "4rem",
  },
  size_large: {
    height: "6rem",
    width: "6rem",
  },
});

const SpotLoader = ({
  isLoading = true,
  style,
  size = COMMON_SIZES.normal,
}) => {

  return (
    isLoading && (
      <>
        <div {...stylex.props(styles[size])}>{size}</div>
        <div {...stylex.props(styles.root, styles.sizeSmall)}>styles.sizeSmall</div>
        <div {...stylex.props(styles.root, styles.sizeSmall, style)}>styles.sizeSmall with styles</div>
      </>
    )
  );
};

export default SpotLoader;
