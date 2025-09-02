import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "vars.stylex.js";
import * as stylex from '@stylexjs/stylex';
import { vars } from 'vars.stylex.js';
_inject2(".x5fq457{--x1anmu0j:var(--x---x1anmu0j)}", 1);
_inject2('@property --x---x1anmu0j { syntax: "*"; inherits: false; }', 0);
export const styles = {
    root: (width)=>[
            {
                "--x1anmu0j": width != null ? "x5fq457" : width,
                $$css: true
            },
            {
                "--x---x1anmu0j": width != null ? width : undefined
            }
        ]
};
