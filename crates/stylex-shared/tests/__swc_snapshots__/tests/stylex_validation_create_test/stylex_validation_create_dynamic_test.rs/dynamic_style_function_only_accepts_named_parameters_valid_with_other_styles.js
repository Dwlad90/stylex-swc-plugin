//__stylex_metadata_start__[{"class_name":"x6zurak","style":{"rtl":null,"ltr":".x6zurak{font-size:var(--fontSize)}"},"priority":3000},{"class_name":"x3stwaq","style":{"rtl":null,"ltr":".x3stwaq{font-weight:100}"},"priority":3000},{"class_name":"xngnso2","style":{"rtl":null,"ltr":".xngnso2{font-size:1.5rem}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from "@stylexjs/stylex";
_inject2(".x6zurak{font-size:var(--fontSize)}", 3000);
_inject2(".x3stwaq{font-weight:100}", 3000);
_inject2(".xngnso2{font-size:1.5rem}", 3000);
const styles = {
    size: (size: number)=>[
            {
                fontSize: 8 * size + 'px' == null ? null : "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        fontWeight: "x3stwaq",
        $$css: true
    },
    largeNumber: {
        fontSize: "xngnso2",
        $$css: true
    }
};
const { className, style = {} } = {
    ...stylex.props(styles.count, styles.size(size), styles.largeNumber)
};
