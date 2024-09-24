//__stylex_metadata_start__[{"class_name":"x1e2nbdu","style":{"rtl":null,"ltr":".x1e2nbdu{color:red}"},"priority":3000},{"class_name":"x78zum5","style":{"rtl":null,"ltr":".x78zum5{display:flex}"},"priority":3000},{"class_name":"xdt5ytf","style":{"rtl":null,"ltr":".xdt5ytf{flex-direction:column}"},"priority":3000},{"class_name":"x6s0dn4","style":{"rtl":null,"ltr":".x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"x1qughib","style":{"rtl":null,"ltr":".x1qughib{justify-content:space-between}"},"priority":3000},{"class_name":"xg6iff7","style":{"rtl":null,"ltr":".xg6iff7{min-height:100vh}"},"priority":4000},{"class_name":"x1lmef92","style":{"rtl":null,"ltr":".x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}"},"priority":1000},{"class_name":"x1swossr","style":{"rtl":null,"ltr":".x1swossr{line-height:1.3em}"},"priority":3000},{"class_name":"xif65rj","style":{"rtl":null,"ltr":".xif65rj{font-size:14px}"},"priority":3000}]__stylex_metadata_end__
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
import React from "react";
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x78zum5{display:flex}", 3000);
_inject2(".xdt5ytf{flex-direction:column}", 3000);
_inject2(".x6s0dn4{align-items:center}", 3000);
_inject2(".x1qughib{justify-content:space-between}", 3000);
_inject2(".xg6iff7{min-height:100vh}", 4000);
_inject2(".x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}", 1000);
_inject2(".x1swossr{line-height:1.3em}", 3000);
_inject2(".xif65rj{font-size:14px}", 3000);
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
        className: "Page__s.main x1e2nbdu x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 x1lmef92 Page__s.title x1swossr xif65rj"
    };
    return <main className={className} style={style}>
      Main
    </main>;
}
