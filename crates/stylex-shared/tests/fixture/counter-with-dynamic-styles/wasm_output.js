//__stylex_metadata_start__[{"class_name":"x6zurak","style":{"rtl":null,"ltr":".x6zurak{font-size:var(--fontSize)}"},"priority":3000},{"class_name":"x78zum5","style":{"rtl":null,"ltr":".x78zum5{display:flex}"},"priority":3000},{"class_name":"x6s0dn4","style":{"rtl":null,"ltr":".x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"xl56j7k","style":{"rtl":null,"ltr":".xl56j7k{justify-content:center}"},"priority":3000},{"class_name":"x1q0g3np","style":{"rtl":null,"ltr":".x1q0g3np{flex-direction:row}"},"priority":3000},{"class_name":"xkorlav","style":{"rtl":null,"ltr":".xkorlav{border-radius:var(--x120tmbh)}"},"priority":2000},{"class_name":"xmkeg23","style":{"rtl":null,"ltr":".xmkeg23{border-width:1px}"},"priority":2000},{"class_name":"x1y0btm7","style":{"rtl":null,"ltr":".x1y0btm7{border-style:solid}"},"priority":2000},{"class_name":"xzj82u7","style":{"rtl":null,"ltr":".xzj82u7{border-color:var(--x1g16e7s)}"},"priority":2000},{"class_name":"xhcr65l","style":{"rtl":null,"ltr":".xhcr65l{padding:var(--xk88l2w)}"},"priority":1000},{"class_name":"x1byiw6p","style":{"rtl":null,"ltr":".x1byiw6p{font-family:var(--x6ywdb8)}"},"priority":3000},{"class_name":"x1l7lfc5","style":{"rtl":null,"ltr":".x1l7lfc5{gap:var(--xvp50ho)}"},"priority":2000},{"class_name":"x17frcva","style":{"rtl":null,"ltr":".x17frcva{height:6rem}"},"priority":4000},{"class_name":"x1plog1","style":{"rtl":null,"ltr":".x1plog1{aspect-ratio:1}"},"priority":3000},{"class_name":"x1ynku2j","style":{"rtl":null,"ltr":".x1ynku2j{color:var(--x1g16e7s)}"},"priority":3000},{"class_name":"xij5jp","style":{"rtl":null,"ltr":".xij5jp{background-color:var(--x1wnl0mb)}"},"priority":3000},{"class_name":"x6lnu34","style":{"rtl":null,"ltr":".x6lnu34:hover{background-color:var(--x1987uwy)}"},"priority":3130},{"class_name":"xd84qqf","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){.xd84qqf.xd84qqf{background-color:var(--xv11w9p)}}"},"priority":3200},{"class_name":"x1lod3q0","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){.x1lod3q0.x1lod3q0:hover{background-color:var(--xd0alct)}}"},"priority":3330},{"class_name":"xc342km","style":{"rtl":null,"ltr":".xc342km{border-width:0}"},"priority":2000},{"class_name":"xng3xce","style":{"rtl":null,"ltr":".xng3xce{border-style:none}"},"priority":2000},{"class_name":"x12ugs8o","style":{"rtl":null,"ltr":".x12ugs8o{border-radius:var(--xvp50ho)}"},"priority":2000},{"class_name":"x1kopudh","style":{"rtl":null,"ltr":".x1kopudh{padding:var(--xvp50ho)}"},"priority":1000},{"class_name":"xp822f4","style":{"rtl":null,"ltr":".xp822f4{margin:var(--xvp50ho)}"},"priority":1000},{"class_name":"x1ypdohk","style":{"rtl":null,"ltr":".x1ypdohk{cursor:pointer}"},"priority":3000},{"class_name":"xf8wwq","style":{"rtl":null,"ltr":".xf8wwq{font-size:var(--x1nryaqe)}"},"priority":3000},{"class_name":"x1u4xmye","style":{"rtl":null,"ltr":".x1u4xmye:hover{transform:scale(1.025)}"},"priority":3130},{"class_name":"xglsxx3","style":{"rtl":null,"ltr":".xglsxx3:active{transform:scale(.975)}"},"priority":3170},{"class_name":"x3stwaq","style":{"rtl":null,"ltr":".x3stwaq{font-weight:100}"},"priority":3000},{"class_name":"x1fk3gbn","style":{"rtl":null,"ltr":".x1fk3gbn{color:var(--x146xnew)}"},"priority":3000},{"class_name":"x1843ork","style":{"rtl":null,"ltr":".x1843ork{min-width:6rem}"},"priority":4000},{"class_name":"x2b8uid","style":{"rtl":null,"ltr":".x2b8uid{text-align:center}"},"priority":3000},{"class_name":"x1nlbcxq","style":{"rtl":null,"ltr":".x1nlbcxq{font-family:var(--xur0yta)}"},"priority":3000},{"class_name":"x8c9cfh","style":{"rtl":null,"ltr":".x8c9cfh{font-size:var(--x1yfd0fu)}"},"priority":3000}]__stylex_metadata_end__
'use client';
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./globalTokens.stylex";
import "@stylexjs/open-props/lib/colors.stylex";
import * as stylex from '@stylexjs/stylex';
import { spacing, text, globalTokens as $ } from './globalTokens.stylex';
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { useState } from 'react';
export default function Counter() {
    const [count, setCount] = useState(0);
    return <div {...{
        className: "Page__styles.container x78zum5 x6s0dn4 xl56j7k x1q0g3np xkorlav xmkeg23 x1y0btm7 xzj82u7 xhcr65l x1byiw6p x1l7lfc5"
    }}>
      <button {...{
        className: "Page__styles.button x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1lod3q0 xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...stylex.props(styles.count, styles.size(count), Math.abs(count) > 99 && styles.largeNumber)}>
        {count}
      </div>
      <button {...{
        className: "Page__styles.button x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1lod3q0 xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
_inject2(".x6zurak{font-size:var(--fontSize)}", 3000);
_inject2(".x78zum5{display:flex}", 3000);
_inject2(".x6s0dn4{align-items:center}", 3000);
_inject2(".xl56j7k{justify-content:center}", 3000);
_inject2(".x1q0g3np{flex-direction:row}", 3000);
_inject2(".xkorlav{border-radius:var(--x120tmbh)}", 2000);
_inject2(".xmkeg23{border-width:1px}", 2000);
_inject2(".x1y0btm7{border-style:solid}", 2000);
_inject2(".xzj82u7{border-color:var(--x1g16e7s)}", 2000);
_inject2(".xhcr65l{padding:var(--xk88l2w)}", 1000);
_inject2(".x1byiw6p{font-family:var(--x6ywdb8)}", 3000);
_inject2(".x1l7lfc5{gap:var(--xvp50ho)}", 2000);
_inject2(".x17frcva{height:6rem}", 4000);
_inject2(".x1plog1{aspect-ratio:1}", 3000);
_inject2(".x1ynku2j{color:var(--x1g16e7s)}", 3000);
_inject2(".xij5jp{background-color:var(--x1wnl0mb)}", 3000);
_inject2(".x6lnu34:hover{background-color:var(--x1987uwy)}", 3130);
_inject2("@media (prefers-color-scheme: dark){.xd84qqf.xd84qqf{background-color:var(--xv11w9p)}}", 3200);
_inject2("@media (prefers-color-scheme: dark){.x1lod3q0.x1lod3q0:hover{background-color:var(--xd0alct)}}", 3330);
_inject2(".xc342km{border-width:0}", 2000);
_inject2(".xng3xce{border-style:none}", 2000);
_inject2(".x12ugs8o{border-radius:var(--xvp50ho)}", 2000);
_inject2(".x1kopudh{padding:var(--xvp50ho)}", 1000);
_inject2(".xp822f4{margin:var(--xvp50ho)}", 1000);
_inject2(".x1ypdohk{cursor:pointer}", 3000);
_inject2(".xf8wwq{font-size:var(--x1nryaqe)}", 3000);
_inject2(".x1u4xmye:hover{transform:scale(1.025)}", 3130);
_inject2(".xglsxx3:active{transform:scale(.975)}", 3170);
_inject2(".x3stwaq{font-weight:100}", 3000);
_inject2(".x1fk3gbn{color:var(--x146xnew)}", 3000);
_inject2(".x1843ork{min-width:6rem}", 4000);
_inject2(".x2b8uid{text-align:center}", 3000);
_inject2(".x1nlbcxq{font-family:var(--xur0yta)}", 3000);
_inject2(".x8c9cfh{font-size:var(--x1yfd0fu)}", 3000);
const styles = {
    size: (size)=>[
            {
                "Page__styles.size": "Page__styles.size",
                fontSize: 8 * size + 'px' == null ? null : "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        "Page__styles.count": "Page__styles.count",
        fontSize: "xf8wwq",
        fontWeight: "x3stwaq",
        color: "x1fk3gbn",
        minWidth: "x1843ork",
        textAlign: "x2b8uid",
        fontFamily: "x1nlbcxq",
        $$css: true
    },
    largeNumber: {
        "Page__styles.largeNumber": "Page__styles.largeNumber",
        fontSize: "x8c9cfh",
        $$css: true
    }
};
