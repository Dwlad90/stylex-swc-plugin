//__stylex_metadata_start__[{"class_name":"x6zurak","style":{"rtl":null,"ltr":".x6zurak{font-size:var(--fontSize)}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".x6zurak{font-size:var(--fontSize)}", 3000);
const styles = {
    fontSizeFallback: (size: number)=>[
            {
                fontSize: (size ?? '1em') == null ? null : "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(size ?? '1em')
            }
        ]
};
const { className, style = {} } = {
    ...stylex.props(styles.fontSizeFallback(size))
};
