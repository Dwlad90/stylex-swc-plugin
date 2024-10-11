//__stylex_metadata_start__[{"class_name":"x13jbg0v","style":{"rtl":null,"ltr":".x13jbg0v{font-size:var(--fontSize,revert)}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".x13jbg0v{font-size:var(--fontSize,revert)}", 3000);
const styles = {
    fontSizeTernary: (size: number)=>[
            {
                fontSize: "x13jbg0v",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : "initial")(size < 10 ? '1em' : '2em')
            }
        ]
};
const { className, style = {} } = {
    ...stylex.props(styles.fontSizeTernary(size))
};
