//__stylex_metadata_start__[{"class_name":"x78zum5","style":{"rtl":null,"ltr":".x78zum5{display:flex}"},"priority":3000},{"class_name":"x6s0dn4","style":{"rtl":null,"ltr":".x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"xl56j7k","style":{"rtl":null,"ltr":".xl56j7k{justify-content:center}"},"priority":3000},{"class_name":"x1q0g3np","style":{"rtl":null,"ltr":".x1q0g3np{flex-direction:row}"},"priority":3000},{"class_name":"xkorlav","style":{"rtl":null,"ltr":".xkorlav{border-radius:var(--x120tmbh)}"},"priority":2000},{"class_name":"xmkeg23","style":{"rtl":null,"ltr":".xmkeg23{border-width:1px}"},"priority":2000},{"class_name":"x1y0btm7","style":{"rtl":null,"ltr":".x1y0btm7{border-style:solid}"},"priority":2000},{"class_name":"xzj82u7","style":{"rtl":null,"ltr":".xzj82u7{border-color:var(--x1g16e7s)}"},"priority":2000},{"class_name":"xhcr65l","style":{"rtl":null,"ltr":".xhcr65l{padding:var(--xk88l2w)}"},"priority":1000},{"class_name":"x1byiw6p","style":{"rtl":null,"ltr":".x1byiw6p{font-family:var(--x6ywdb8)}"},"priority":3000},{"class_name":"x1l7lfc5","style":{"rtl":null,"ltr":".x1l7lfc5{gap:var(--xvp50ho)}"},"priority":2000},{"class_name":"x17frcva","style":{"rtl":null,"ltr":".x17frcva{height:6rem}"},"priority":4000},{"class_name":"x1plog1","style":{"rtl":null,"ltr":".x1plog1{aspect-ratio:1}"},"priority":3000},{"class_name":"x1ynku2j","style":{"rtl":null,"ltr":".x1ynku2j{color:var(--x1g16e7s)}"},"priority":3000},{"class_name":"xij5jp","style":{"rtl":null,"ltr":".xij5jp{background-color:var(--x1wnl0mb)}"},"priority":3000},{"class_name":"x6lnu34","style":{"rtl":null,"ltr":".x6lnu34:hover{background-color:var(--x1987uwy)}"},"priority":3130},{"class_name":"xd84qqf","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){.xd84qqf.xd84qqf{background-color:var(--xv11w9p)}}"},"priority":3200},{"class_name":"x1lod3q0","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){.x1lod3q0.x1lod3q0:hover{background-color:var(--xd0alct)}}"},"priority":3330},{"class_name":"xc342km","style":{"rtl":null,"ltr":".xc342km{border-width:0}"},"priority":2000},{"class_name":"xng3xce","style":{"rtl":null,"ltr":".xng3xce{border-style:none}"},"priority":2000},{"class_name":"x12ugs8o","style":{"rtl":null,"ltr":".x12ugs8o{border-radius:var(--xvp50ho)}"},"priority":2000},{"class_name":"x1kopudh","style":{"rtl":null,"ltr":".x1kopudh{padding:var(--xvp50ho)}"},"priority":1000},{"class_name":"xp822f4","style":{"rtl":null,"ltr":".xp822f4{margin:var(--xvp50ho)}"},"priority":1000},{"class_name":"x1ypdohk","style":{"rtl":null,"ltr":".x1ypdohk{cursor:pointer}"},"priority":3000},{"class_name":"xf8wwq","style":{"rtl":null,"ltr":".xf8wwq{font-size:var(--x1nryaqe)}"},"priority":3000},{"class_name":"x1u4xmye","style":{"rtl":null,"ltr":".x1u4xmye:hover{transform:scale(1.025)}"},"priority":3130},{"class_name":"xglsxx3","style":{"rtl":null,"ltr":".xglsxx3:active{transform:scale(.975)}"},"priority":3170},{"class_name":"x3stwaq","style":{"rtl":null,"ltr":".x3stwaq{font-weight:100}"},"priority":3000},{"class_name":"x1fk3gbn","style":{"rtl":null,"ltr":".x1fk3gbn{color:var(--x146xnew)}"},"priority":3000},{"class_name":"x1843ork","style":{"rtl":null,"ltr":".x1843ork{min-width:6rem}"},"priority":4000},{"class_name":"x2b8uid","style":{"rtl":null,"ltr":".x2b8uid{text-align:center}"},"priority":3000},{"class_name":"x1nlbcxq","style":{"rtl":null,"ltr":".x1nlbcxq{font-family:var(--xur0yta)}"},"priority":3000},{"class_name":"x8c9cfh","style":{"rtl":null,"ltr":".x8c9cfh{font-size:var(--x1yfd0fu)}"},"priority":3000}]__stylex_metadata_end__
'use client';
import * as stylex from '@stylexjs/stylex';
import { spacing, text, globalTokens as $ } from './globalTokens.stylex';
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { useState } from 'react';
export default function Counter() {
    const [count, setCount] = useState(0);
    return <div {...{
        className: "x78zum5 x6s0dn4 xl56j7k x1q0g3np xkorlav xmkeg23 x1y0btm7 xzj82u7 xhcr65l x1byiw6p x1l7lfc5"
    }}>
      <button {...{
        className: "x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1lod3q0 xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...{
        0: {
            className: "xf8wwq x3stwaq x1fk3gbn x1843ork x2b8uid x1nlbcxq"
        },
        1: {
            className: "x3stwaq x1fk3gbn x1843ork x2b8uid x1nlbcxq x8c9cfh"
        }
    }[!!(Math.abs(count) > 99) << 0]}>
        {count}
      </div>
      <button {...{
        className: "x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1lod3q0 xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
