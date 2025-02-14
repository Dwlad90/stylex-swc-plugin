import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "@stylexjs/open-props/lib/fonts.stylex";
import * as stylex from '@stylexjs/stylex';
import { fonts as foo } from '@stylexjs/open-props/lib/fonts.stylex';
import { type ReactNode } from 'react';
_inject2(".x13rv2e4{color:hotpink}", 3000);
const styles = {
    text: {
        color: "x13rv2e4",
        $$css: true
    }
};
_inject2(".x1nbzn64{font-size:var(--xz4eux4)}", 3000);
_inject2(".xx5h6fz{font-size:var(--xsnljwq)}", 3000);
const variants = {
    small: {
        fontSize: "x1nbzn64",
        $$css: true
    },
    big: {
        fontSize: "xx5h6fz",
        $$css: true
    }
};
export interface TextProps {
    children: ReactNode;
    size: keyof typeof variants;
}
export function Text2({ children, size: foo }: TextProps) {
    return <div {...stylex.props(styles.text, variants[foo])}>{children}</div>;
}
