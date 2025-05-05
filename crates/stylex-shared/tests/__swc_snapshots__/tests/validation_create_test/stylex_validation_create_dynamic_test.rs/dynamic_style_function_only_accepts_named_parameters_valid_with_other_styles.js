import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".x6zurak{font-size:var(--fontSize)}", 3000);
_inject2(".x3stwaq{font-weight:100}", 3000);
_inject2(".xngnso2{font-size:1.5rem}", 3000);
_inject2('@property --fontSize { syntax: "*"; inherits: false; }', 0);
const styles = {
    size: (size: number)=>[
            {
                kGuDYH: "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        k63SB2: "x3stwaq",
        $$css: true
    },
    largeNumber: {
        kGuDYH: "xngnso2",
        $$css: true
    }
};
const sizeValue = 10;
const { className, style = {} } = {
    ...stylex.props(styles.count, styles.size(sizeValue), styles.largeNumber)
};
