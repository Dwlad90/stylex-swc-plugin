'use client';
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./globalTokens.stylex";
import "@stylexjs/open-props/lib/colors.stylex";
import * as stylex from '@stylexjs/stylex';
import { spacing, text, globalTokens as $ } from './globalTokens.stylex';
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { useState } from 'react';
const _temp = {
    "input__styles.size": "input__styles.size",
    fontSize: "xdmh292",
    $$css: "tests/fixture/counter-with-dynamic-styles/input.stylex.js:41"
};
export default function Counter() {
    const [count, setCount] = useState(0);
    return <div className="input__styles.container x78zum5 x6s0dn4 xl56j7k x1q0g3np x12xgqvu xmkeg23 x1y0btm7 xqpy6nh x87erls x1alyrvt x1749g51" data-style-src="tests/fixture/counter-with-dynamic-styles/input.stylex.js:42">
      <button className="input__styles.button x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 xgopyf5 xpotius x1gk0e8 x10vqmf9 xnha941 xc342km xng3xce xvm41bv xsbzlvg x1i5nj67 x1ypdohk xtqx43c x1u4xmye xglsxx3" data-style-src="tests/fixture/counter-with-dynamic-styles/input.stylex.js:55" onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...stylex.props(styles.count, styles.size(count), Math.abs(count) > 99 && styles.largeNumber)}>
        {count}
      </div>
      <button className="input__styles.button x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 xgopyf5 xpotius x1gk0e8 x10vqmf9 xnha941 xc342km xng3xce xvm41bv xsbzlvg x1i5nj67 x1ypdohk xtqx43c x1u4xmye xglsxx3" data-style-src="tests/fixture/counter-with-dynamic-styles/input.stylex.js:55" onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
_inject2({
    ltr: ".xdmh292{font-size:var(--x-fontSize)}",
    priority: 3000
});
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".xl56j7k{justify-content:center}",
    priority: 3000
});
_inject2({
    ltr: ".x1q0g3np{flex-direction:row}",
    priority: 3000
});
_inject2({
    ltr: ".x12xgqvu{border-radius:var(--xz85zqu)}",
    priority: 2000
});
_inject2({
    ltr: ".xmkeg23{border-width:1px}",
    priority: 2000
});
_inject2({
    ltr: ".x1y0btm7{border-style:solid}",
    priority: 2000
});
_inject2({
    ltr: ".xqpy6nh{border-color:var(--x1kefbne)}",
    priority: 2000
});
_inject2({
    ltr: ".x87erls{padding:var(--x1jgrv4s)}",
    priority: 1000
});
_inject2({
    ltr: ".x1alyrvt{font-family:var(--x1v0ot8g)}",
    priority: 3000
});
_inject2({
    ltr: ".x1749g51{gap:var(--x1yemeo2)}",
    priority: 2000
});
_inject2({
    ltr: ".x17frcva{height:6rem}",
    priority: 4000
});
_inject2({
    ltr: ".x1plog1{aspect-ratio:1}",
    priority: 3000
});
_inject2({
    ltr: ".xgopyf5{color:var(--x1kefbne)}",
    priority: 3000
});
_inject2({
    ltr: ".xpotius{background-color:var(--x1h92w08)}",
    priority: 3000
});
_inject2({
    ltr: ".x1gk0e8:hover{background-color:var(--xw5rm9m)}",
    priority: 3130
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){.x10vqmf9.x10vqmf9{background-color:var(--x1k6ilpx)}}",
    priority: 3200
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){.xnha941.xnha941:hover{background-color:var(--x1ru5ylq)}}",
    priority: 3330
});
_inject2({
    ltr: ".xc342km{border-width:0}",
    priority: 2000
});
_inject2({
    ltr: ".xng3xce{border-style:none}",
    priority: 2000
});
_inject2({
    ltr: ".xvm41bv{border-radius:var(--x1yemeo2)}",
    priority: 2000
});
_inject2({
    ltr: ".xsbzlvg{padding:var(--x1yemeo2)}",
    priority: 1000
});
_inject2({
    ltr: ".x1i5nj67{margin:var(--x1yemeo2)}",
    priority: 1000
});
_inject2({
    ltr: ".x1ypdohk{cursor:pointer}",
    priority: 3000
});
_inject2({
    ltr: ".xtqx43c{font-size:var(--x1al5pe7)}",
    priority: 3000
});
_inject2({
    ltr: ".x1u4xmye:hover{transform:scale(1.025)}",
    priority: 3130
});
_inject2({
    ltr: ".xglsxx3:active{transform:scale(.975)}",
    priority: 3170
});
_inject2({
    ltr: ".x3stwaq{font-weight:100}",
    priority: 3000
});
_inject2({
    ltr: ".xxzazoc{color:var(--x16pcyb)}",
    priority: 3000
});
_inject2({
    ltr: ".x1843ork{min-width:6rem}",
    priority: 4000
});
_inject2({
    ltr: ".x2b8uid{text-align:center}",
    priority: 3000
});
_inject2({
    ltr: ".xum72dy{font-family:var(--xgc26q9)}",
    priority: 3000
});
_inject2({
    ltr: ".xs6c6ls{font-size:var(--xbf52ah)}",
    priority: 3000
});
_inject2({
    ltr: '@property --x-fontSize { syntax: "*"; inherits: false;}',
    priority: 0
});
const styles = {
    size: (size)=>[
            _temp,
            {
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        "input__styles.count": "input__styles.count",
        fontSize: "xtqx43c",
        fontWeight: "x3stwaq",
        color: "xxzazoc",
        minWidth: "x1843ork",
        textAlign: "x2b8uid",
        fontFamily: "xum72dy",
        $$css: "tests/fixture/counter-with-dynamic-styles/input.stylex.js:83"
    },
    largeNumber: {
        "input__styles.largeNumber": "input__styles.largeNumber",
        fontSize: "xs6c6ls",
        $$css: "tests/fixture/counter-with-dynamic-styles/input.stylex.js:91"
    }
};
