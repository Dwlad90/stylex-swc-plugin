import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "@stylexjs/open-props/lib/fonts.stylex";
import * as stylex from '@stylexjs/stylex';
import { fonts as f } from '@stylexjs/open-props/lib/fonts.stylex';
import { type ReactNode } from 'react';
_inject2(".x13rv2e4{color:hotpink}", 3000);
const styles = {
    text: {
        kMwMTN: "x13rv2e4",
        $$css: true
    }
};
_inject2(".x3b68l4{font-size:var(--x1cjnt43)}", 3000);
_inject2(".x59qt1o{font-size:var(--xw8ib4r)}", 3000);
const variants = {
    small: {
        kGuDYH: "x3b68l4",
        $$css: true
    },
    big: {
        kGuDYH: "x59qt1o",
        $$css: true
    }
};
export interface TextProps {
    children: ReactNode;
    size: keyof typeof variants;
}
export function Text2({ children, size: s }: TextProps) {
    return <div {...stylex.props(styles.text, variants[s])}>{children}</div>;
}
