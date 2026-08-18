import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
_inject2({
    ltr: ".x14rh7hd{color:var(--x-color)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-color { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    a: (props)=>[
            {
                kMwMTN: (props.flag && "documentation".startsWith(props.q)) != null ? "x14rh7hd" : props.flag && "documentation".startsWith(props.q),
                $$css: true
            },
            {
                "--x-color": (props.flag && "documentation".startsWith(props.q)) != null ? props.flag && "documentation".startsWith(props.q) : undefined
            }
        ],
    b: (props)=>[
            {
                kMwMTN: (props.flag || "documentation".startsWith(props.q)) != null ? "x14rh7hd" : props.flag || "documentation".startsWith(props.q),
                $$css: true
            },
            {
                "--x-color": (props.flag || "documentation".startsWith(props.q)) != null ? props.flag || "documentation".startsWith(props.q) : undefined
            }
        ],
    c: (props)=>[
            {
                kMwMTN: (props.flag ?? "documentation".startsWith(props.q)) != null ? "x14rh7hd" : props.flag ?? "documentation".startsWith(props.q),
                $$css: true
            },
            {
                "--x-color": (props.flag ?? "documentation".startsWith(props.q)) != null ? props.flag ?? "documentation".startsWith(props.q) : undefined
            }
        ]
};
