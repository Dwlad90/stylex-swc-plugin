import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x1fvhpy6{background-color:var(--x-bcbnzo)}",
    priority: 3000
});
_inject2({
    ltr: ".x1j2k28p:hover{background-color:var(--x-1e2mv7m)}",
    priority: 3130
});
_inject2({
    ltr: '@property --x-bcbnzo { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-1e2mv7m { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    a: (backgroundColor, backgroundColorHover)=>[
            {
                kWkggS: (backgroundColor != null ? "x1fvhpy6 " : backgroundColor) + (backgroundColorHover != null ? "x1j2k28p" : backgroundColorHover),
                $$css: true
            },
            {
                "--x-bcbnzo": backgroundColor != null ? backgroundColor : undefined,
                "--x-1e2mv7m": backgroundColorHover != null ? backgroundColorHover : undefined
            }
        ]
};
