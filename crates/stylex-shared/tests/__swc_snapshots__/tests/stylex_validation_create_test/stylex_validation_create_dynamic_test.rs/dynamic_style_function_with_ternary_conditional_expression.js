import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".x6zurak{font-size:var(--fontSize)}", 3000);
_inject2('@property --fontSize { syntax: "*"; inherits: false; }', 0);
const styles = {
    fontSizeTernary: (size: number)=>[
            {
                fontSize: (size < 10 ? '1em' : '2em') == null ? null : "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(size < 10 ? '1em' : '2em')
            }
        ]
};
const { className, style = {} } = {
    ...stylex.props(styles.fontSizeTernary(size))
};
