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
        className: "Page__styles.container display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k flexDirection-x1q0g3np borderRadius-xkorlav borderWidth-xmkeg23 borderStyle-x1y0btm7 borderColor-xzj82u7 padding-xhcr65l fontFamily-x1byiw6p gap-x1l7lfc5"
    }}>
      <button {...{
        className: "Page__styles.button display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-x1ynku2j backgroundColor-xij5jp backgroundColor-x6lnu34 backgroundColor-xd84qqf backgroundColor-x1lod3q0 borderWidth-xc342km borderStyle-xng3xce borderRadius-x12ugs8o padding-x1kopudh margin-xp822f4 cursor-x1ypdohk fontSize-xf8wwq transform-x1u4xmye transform-xglsxx3"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...{
        0: {
            className: "Page__styles.count fontSize-xf8wwq fontWeight-x3stwaq color-x1fk3gbn minWidth-x1843ork textAlign-x2b8uid fontFamily-x1nlbcxq"
        },
        1: {
            className: "Page__styles.count fontWeight-x3stwaq color-x1fk3gbn minWidth-x1843ork textAlign-x2b8uid fontFamily-x1nlbcxq Page__styles.largeNumber fontSize-x8c9cfh"
        }
    }[!!(Math.abs(count) > 99) << 0]}>
        {count}
      </div>
      <button {...{
        className: "Page__styles.button display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-x1ynku2j backgroundColor-xij5jp backgroundColor-x6lnu34 backgroundColor-xd84qqf backgroundColor-x1lod3q0 borderWidth-xc342km borderStyle-xng3xce borderRadius-x12ugs8o padding-x1kopudh margin-xp822f4 cursor-x1ypdohk fontSize-xf8wwq transform-x1u4xmye transform-xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
const DARK = '@media (prefers-color-scheme: dark)';
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".flexDirection-x1q0g3np{flex-direction:row}", 3000);
_inject2(".borderRadius-xkorlav{border-radius:var(--x120tmbh)}", 2000);
_inject2(".borderWidth-xmkeg23{border-width:1px}", 2000);
_inject2(".borderStyle-x1y0btm7{border-style:solid}", 2000);
_inject2(".borderColor-xzj82u7{border-color:var(--x1g16e7s)}", 2000);
_inject2(".padding-xhcr65l{padding:var(--xk88l2w)}", 1000);
_inject2(".fontFamily-x1byiw6p{font-family:var(--x6ywdb8)}", 3000);
_inject2(".gap-x1l7lfc5{gap:var(--xvp50ho)}", 2000);
_inject2(".height-x17frcva{height:6rem}", 4000);
_inject2(".aspectRatio-x1plog1{aspect-ratio:1}", 3000);
_inject2(".color-x1ynku2j{color:var(--x1g16e7s)}", 3000);
_inject2(".backgroundColor-xij5jp{background-color:var(--x1wnl0mb)}", 3000);
_inject2(".backgroundColor-x6lnu34:hover{background-color:var(--x1987uwy)}", 3130);
_inject2("@media (prefers-color-scheme: dark){.backgroundColor-xd84qqf.backgroundColor-xd84qqf{background-color:var(--xv11w9p)}}", 3200);
_inject2("@media (prefers-color-scheme: dark){.backgroundColor-x1lod3q0.backgroundColor-x1lod3q0:hover{background-color:var(--xd0alct)}}", 3330);
_inject2(".borderWidth-xc342km{border-width:0}", 2000);
_inject2(".borderStyle-xng3xce{border-style:none}", 2000);
_inject2(".borderRadius-x12ugs8o{border-radius:var(--xvp50ho)}", 2000);
_inject2(".padding-x1kopudh{padding:var(--xvp50ho)}", 1000);
_inject2(".margin-xp822f4{margin:var(--xvp50ho)}", 1000);
_inject2(".cursor-x1ypdohk{cursor:pointer}", 3000);
_inject2(".fontSize-xf8wwq{font-size:var(--x1nryaqe)}", 3000);
_inject2(".transform-x1u4xmye:hover{transform:scale(1.025)}", 3130);
_inject2(".transform-xglsxx3:active{transform:scale(.975)}", 3170);
_inject2(".fontWeight-x3stwaq{font-weight:100}", 3000);
_inject2(".color-x1fk3gbn{color:var(--x146xnew)}", 3000);
_inject2(".minWidth-x1843ork{min-width:6rem}", 4000);
_inject2(".textAlign-x2b8uid{text-align:center}", 3000);
_inject2(".fontFamily-x1nlbcxq{font-family:var(--xur0yta)}", 3000);
_inject2(".fontSize-x8c9cfh{font-size:var(--x1yfd0fu)}", 3000);
