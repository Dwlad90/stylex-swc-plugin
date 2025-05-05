import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".x6zurak{font-size:var(--fontSize)}", 3000);
_inject2('@property --fontSize { syntax: "*"; inherits: false; }', 0);
const styles = {
    fontSizeFallback: (size: number)=>[
            {
                kGuDYH: "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(size ?? '1em')
            }
        ]
};
const sizeValue = 10;
const { className, style = {} } = {
    ...stylex.props(styles.fontSizeFallback(sizeValue))
};
