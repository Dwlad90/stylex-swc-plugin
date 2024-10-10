'use client';
import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "@stylexjs/open-props/lib/colors.stylex";
import "./globalTokens.stylex";
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
        className: "Page__styles.button x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1ap9xfb xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...stylex.props(styles.count, styles.size(count), Math.abs(count) > 99 && styles.largeNumber)}>
        {count}
      </div>
      <button {...{
        className: "Page__styles.button x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1ap9xfb xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
const DARK = '@media (prefers-color-scheme: dark)';
_inject2(".x13jbg0v{font-size:var(--fontSize,revert)}", 3000);
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
_inject2("@media (prefers-color-scheme: dark){.x1ap9xfb.x1ap9xfb:hover{background-color:var(--xd0alct)}}", 3330);
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
                fontSize: "x13jbg0v",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : "initial")(8 * size + 'px')
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
