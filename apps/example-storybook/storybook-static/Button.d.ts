import { FC } from '../../../node_modules/.pnpm/react@19.1.1/node_modules/react';
import * as stylex from '@stylexjs/stylex';
export declare const buttonStyles: Readonly<{
    readonly base: Readonly<{
        readonly fontFamily: stylex.StyleXClassNameFor<"fontFamily", "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif">;
        readonly fontWeight: stylex.StyleXClassNameFor<"fontWeight", 500>;
        readonly cursor: stylex.StyleXClassNameFor<"cursor", "pointer">;
        readonly borderWidth: stylex.StyleXClassNameFor<"borderWidth", 0>;
        readonly borderStyle: stylex.StyleXClassNameFor<"borderStyle", "none">;
        readonly borderColor: stylex.StyleXClassNameFor<"borderColor", "transparent">;
        readonly borderRadius: stylex.StyleXClassNameFor<"borderRadius", 6>;
        readonly transition: stylex.StyleXClassNameFor<"transition", "all 0.2s ease">;
        readonly display: stylex.StyleXClassNameFor<"display", "inline-flex">;
        readonly alignItems: stylex.StyleXClassNameFor<"alignItems", "center">;
        readonly justifyContent: stylex.StyleXClassNameFor<"justifyContent", "center">;
        readonly textDecoration: stylex.StyleXClassNameFor<"textDecoration", "none">;
        readonly transform: stylex.StyleXClassNameFor<"transform", "translateY(0)" | "translateY(-1px)">;
        readonly boxShadow: stylex.StyleXClassNameFor<"boxShadow", "none" | "0 4px 8px rgba(0, 0, 0, 0.12)">;
    }>;
    readonly primary: Readonly<{
        readonly backgroundColor: stylex.StyleXClassNameFor<"backgroundColor", "#0066cc" | "#0052a3">;
        readonly color: stylex.StyleXClassNameFor<"color", "white">;
    }>;
    readonly secondary: Readonly<{
        readonly backgroundColor: stylex.StyleXClassNameFor<"backgroundColor", "#f5f5f5" | "#e8e8e8">;
        readonly color: stylex.StyleXClassNameFor<"color", "#333">;
        readonly borderWidth: stylex.StyleXClassNameFor<"borderWidth", 1>;
        readonly borderStyle: stylex.StyleXClassNameFor<"borderStyle", "solid">;
        readonly borderColor: stylex.StyleXClassNameFor<"borderColor", "#ddd">;
    }>;
    readonly danger: Readonly<{
        readonly backgroundColor: stylex.StyleXClassNameFor<"backgroundColor", "#dc3545" | "#c82333">;
        readonly color: stylex.StyleXClassNameFor<"color", "white">;
    }>;
    readonly small: Readonly<{
        readonly fontSize: stylex.StyleXClassNameFor<"fontSize", 12>;
        readonly padding: stylex.StyleXClassNameFor<"padding", "6px 12px">;
        readonly minHeight: stylex.StyleXClassNameFor<"minHeight", 28>;
    }>;
    readonly medium: Readonly<{
        readonly fontSize: stylex.StyleXClassNameFor<"fontSize", 14>;
        readonly padding: stylex.StyleXClassNameFor<"padding", "8px 16px">;
        readonly minHeight: stylex.StyleXClassNameFor<"minHeight", 36>;
    }>;
    readonly large: Readonly<{
        readonly fontSize: stylex.StyleXClassNameFor<"fontSize", 16>;
        readonly padding: stylex.StyleXClassNameFor<"padding", "12px 24px">;
        readonly minHeight: stylex.StyleXClassNameFor<"minHeight", 44>;
    }>;
}>;
type ButtonProps = {
    /**
     * The size of the button
     * @default 'medium'
     */
    size?: 'small' | 'medium' | 'large';
    /** The variant of the button
     * @default 'primary'
     */
    variant?: 'primary' | 'secondary' | 'danger';
    /**
     * The label of the button
     * @example 'Click me'
     */
    label: string;
    /**
     * Function to call when the button is clicked
     */
    onClick?: () => void;
};
export declare const Button: FC<ButtonProps>;
export {};
