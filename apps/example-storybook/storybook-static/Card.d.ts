import { FC } from '../../../node_modules/.pnpm/react@19.1.1/node_modules/react';
import * as stylex from '@stylexjs/stylex';
export declare const cardStyles: Readonly<{
    readonly base: Readonly<{
        readonly backgroundColor: stylex.StyleXClassNameFor<"backgroundColor", "white">;
        readonly borderRadius: stylex.StyleXClassNameFor<"borderRadius", 8>;
        readonly padding: stylex.StyleXClassNameFor<"padding", 16>;
        readonly boxShadow: stylex.StyleXClassNameFor<"boxShadow", "0 2px 4px rgba(0, 0, 0, 0.1)">;
        readonly borderWidth: stylex.StyleXClassNameFor<"borderWidth", 1>;
        readonly borderStyle: stylex.StyleXClassNameFor<"borderStyle", "solid">;
        readonly borderColor: stylex.StyleXClassNameFor<"borderColor", "#e0e0e0">;
        readonly maxWidth: stylex.StyleXClassNameFor<"maxWidth", 300>;
    }>;
    readonly title: Readonly<{
        readonly fontSize: stylex.StyleXClassNameFor<"fontSize", 18>;
        readonly fontWeight: stylex.StyleXClassNameFor<"fontWeight", "bold">;
        readonly marginBottom: stylex.StyleXClassNameFor<"marginBottom", 8>;
        readonly color: stylex.StyleXClassNameFor<"color", "#333">;
    }>;
    readonly content: Readonly<{
        readonly fontSize: stylex.StyleXClassNameFor<"fontSize", 14>;
        readonly lineHeight: stylex.StyleXClassNameFor<"lineHeight", 1.5>;
        readonly color: stylex.StyleXClassNameFor<"color", "#666">;
    }>;
    readonly elevated: Readonly<{
        readonly boxShadow: stylex.StyleXClassNameFor<"boxShadow", "0 4px 8px rgba(0, 0, 0, 0.15)">;
    }>;
}>;
type CardProps = {
    title: string;
    content: string;
    elevated?: boolean;
};
export declare const Card: FC<CardProps>;
export {};
