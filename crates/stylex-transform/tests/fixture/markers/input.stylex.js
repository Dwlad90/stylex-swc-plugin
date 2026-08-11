import * as stylex from "@stylexjs/stylex";

import { importedMarker } from "other-markers.stylex";

export const localMarker = stylex.defineMarker();

export const styles = stylex.create({
  label: {
    color: {
      default: "gray",
      [stylex.when.ancestor("[data-open]", localMarker)]: "white",
      [stylex.when.descendant(":focus", importedMarker)]: "blue",
      [stylex.when.siblingBefore(":hover")]: "black",
    },
  },
});
