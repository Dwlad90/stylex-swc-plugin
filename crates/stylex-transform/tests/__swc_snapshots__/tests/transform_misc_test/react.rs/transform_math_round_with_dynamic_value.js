import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from '@stylexjs/stylex';
const _temp = {
    kGuDYH: "xdmh292",
    $$css: true
};
const _temp2 = {
    kGuDYH: "xdmh292",
    $$css: true
};
const _temp3 = {
    kGuDYH: "xdmh292",
    $$css: true
};
const _temp4 = {
    kGuDYH: "xdmh292",
    $$css: true
};
_inject2({
    ltr: ".xdmh292{font-size:var(--x-fontSize)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-fontSize { syntax: "*"; inherits: false;}',
    priority: 0
});
const styles = {
    round: (size)=>[
            _temp,
            {
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(`${Math.round(size * (2 / 3))}px`)
            }
        ],
    min: (size)=>[
            _temp2,
            {
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(`${Math.min(size * (2 / 3), 12)}px`)
            }
        ],
    abs: (size)=>[
            _temp3,
            {
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(Math.abs(size * (2 / 3)) + "px")
            }
        ],
    pow: (size)=>[
            _temp4,
            {
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(Math.pow(size * (2 / 3), 2) + "px")
            }
        ]
};
export default function Component({ round, min, abs, pow }) {
    return <div {...stylex.props(round && styles.round(12), min && styles.min(12), abs && styles.abs(12), pow && styles.pow(12))}/>;
}
