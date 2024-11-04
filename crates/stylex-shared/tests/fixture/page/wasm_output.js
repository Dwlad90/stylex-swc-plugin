//__stylex_metadata_start__[{"class_name":"color-x1e2nbdu","style":{"rtl":null,"ltr":".color-x1e2nbdu{color:red}"},"priority":3000},{"class_name":"display-x78zum5","style":{"rtl":null,"ltr":".display-x78zum5{display:flex}"},"priority":3000},{"class_name":"flexDirection-xdt5ytf","style":{"rtl":null,"ltr":".flexDirection-xdt5ytf{flex-direction:column}"},"priority":3000},{"class_name":"alignItems-x6s0dn4","style":{"rtl":null,"ltr":".alignItems-x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"justifyContent-x1qughib","style":{"rtl":null,"ltr":".justifyContent-x1qughib{justify-content:space-between}"},"priority":3000},{"class_name":"minHeight-xg6iff7","style":{"rtl":null,"ltr":".minHeight-xg6iff7{min-height:100vh}"},"priority":4000},{"class_name":"padding-x1lmef92","style":{"rtl":null,"ltr":".padding-x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}"},"priority":1000},{"class_name":"lineHeight-x1swossr","style":{"rtl":null,"ltr":".lineHeight-x1swossr{line-height:1.3em}"},"priority":3000},{"class_name":"fontSize-xif65rj","style":{"rtl":null,"ltr":".fontSize-xif65rj{font-size:14px}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import React from "react";
_inject2(".color-x1e2nbdu{color:red}", 3000);
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".flexDirection-xdt5ytf{flex-direction:column}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-x1qughib{justify-content:space-between}", 3000);
_inject2(".minHeight-xg6iff7{min-height:100vh}", 4000);
_inject2(".padding-x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}", 1000);
_inject2(".lineHeight-x1swossr{line-height:1.3em}", 3000);
_inject2(".fontSize-xif65rj{font-size:14px}", 3000);
function getStaticProps() {
    return {
        props: {}
    };
}
const { foo, ...a } = {
    foo: "bar",
    baz: "qux"
};
export default function Home() {
    const { className, style } = {
        className: "Page__s.main color-x1e2nbdu display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-x1qughib minHeight-xg6iff7 padding-x1lmef92 Page__s.title lineHeight-x1swossr fontSize-xif65rj"
    };
    return <main className={className} style={style}>
      Main
    </main>;
}
