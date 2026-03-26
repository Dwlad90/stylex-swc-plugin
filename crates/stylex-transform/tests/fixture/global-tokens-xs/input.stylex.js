import stylex from "@stylexjs/stylex";

const MIN_WIDTH = 320;
const MAX_WIDTH = 1240;
const MIN_SCALE = 1.2;
const MAX_SCALE = 1.333;
const MIN_BASE_SIZE = 16;
const MAX_BASE_SIZE = 20;

// Font sizes in `rem` units
const MIN_FONT = {
  xs: Math.round(MIN_BASE_SIZE / Math.pow(MIN_SCALE, 2) / 0.16) / 100,
};
// Font sizes in `rem` units
const MAX_FONT = {
  xs: Math.round(MAX_BASE_SIZE / Math.pow(MAX_SCALE, 2) / 0.16) / 100,
};
const SLOPE = {
  xs: (16 * (MAX_FONT.xs - MIN_FONT.xs)) / (MAX_WIDTH - MIN_WIDTH),
};
const INTERCEPT = {
  xs: Math.round(100 * (MIN_FONT.xs - SLOPE.xs * (MIN_WIDTH / 16))) / 100,
};

export const text = stylex.defineVars({
  xs: `${INTERCEPT.xs}rem`,
});


/**
 * Color Tokens
 */
const DARK_MODE = "@media (prefers-color-scheme: dark)";

export const globalTokens = stylex.defineVars({
  foregroundR: { default: "0", [DARK_MODE]: "255" },
  foregroundG: { default: "0", [DARK_MODE]: "255" },
  foregroundB: { default: "0", [DARK_MODE]: "255" },

  bgStartRGB: { default: "rgb(214, 219, 220)", [DARK_MODE]: "rgb(0, 0, 0)" },

  bgEndR: { default: "255", [DARK_MODE]: "0" },
  bgEndG: { default: "255", [DARK_MODE]: "0" },
  bgEndB: { default: "255", [DARK_MODE]: "0" },

  calloutRGB: { default: "rgb(238, 240, 241)", [DARK_MODE]: "rgb(20, 20, 20)" },
  calloutRGB50: {
    default: "rgba(238, 240, 241, 0.5)",
    [DARK_MODE]: "rgba(20, 20, 20, 0.5)",
  },

  calloutBorderR: { default: "172", [DARK_MODE]: "108" },
  calloutBorderG: { default: "175", [DARK_MODE]: "108" },
  calloutBorderB: { default: "176", [DARK_MODE]: "108" },

  cardR: { default: "180", [DARK_MODE]: "100" },
  cardG: { default: "185", [DARK_MODE]: "100" },
  cardB: { default: "188", [DARK_MODE]: "100" },

  cardBorderR: { default: "131", [DARK_MODE]: "200" },
  cardBorderG: { default: "134", [DARK_MODE]: "200" },
  cardBorderB: { default: "135", [DARK_MODE]: "200" },

  primaryGlow: {
    default:
      "conic-gradient(from 180deg at 50% 50%, #16abff33 0deg, #0885ff33 55deg, #54d6ff33 120deg, #0071ff33 160deg, transparent 360deg)",
    [DARK_MODE]: "radial-gradient(rgba(1, 65, 255, 0.4), rgba(1, 65, 255, 0))",
  },
  secondaryGlow: {
    default: "radial-gradient(rgba(255, 255, 255, 1), rgba(255, 255, 255, 0))",
    [DARK_MODE]: `linear-gradient(to bottom right, rgba(1, 65, 255, 0), rgba(1, 65, 255, 0), rgba(1, 65, 255, 0.3))`,
  },
});
