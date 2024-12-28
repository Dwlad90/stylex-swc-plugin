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
        className: "Page__styles.container display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k flexDirection-x1q0g3np borderRadius-x1hc0yr1 borderWidth-xmkeg23 borderStyle-x1y0btm7 borderColor-xd7ip7t padding-x1j3i37e fontFamily-x9388is gap-xqp6ha3"
    }}>
      <button {...{
        className: "Page__styles.button display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-xufi7tb backgroundColor-x4o57bs backgroundColor-x9hm0p backgroundColor-xxl4oju backgroundColor-xag9bkp borderWidth-xc342km borderStyle-xng3xce borderRadius-xezyu2j padding-xpa3c2p margin-x1ots716 cursor-x1ypdohk fontSize-x1tq7rpn transform-x1u4xmye transform-xglsxx3"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...{
        0: {
            className: "Page__styles.count fontSize-x1tq7rpn fontWeight-x3stwaq color-x18c051v minWidth-x1843ork textAlign-x2b8uid fontFamily-xovafh8"
        },
        1: {
            className: "Page__styles.count fontWeight-x3stwaq color-x18c051v minWidth-x1843ork textAlign-x2b8uid fontFamily-xovafh8 Page__styles.largeNumber fontSize-x1c27omx"
        }
    }[!!(Math.abs(count) > 99) << 0]}>
        {count}
      </div>
      <button {...{
        className: "Page__styles.button display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-xufi7tb backgroundColor-x4o57bs backgroundColor-x9hm0p backgroundColor-xxl4oju backgroundColor-xag9bkp borderWidth-xc342km borderStyle-xng3xce borderRadius-xezyu2j padding-xpa3c2p margin-x1ots716 cursor-x1ypdohk fontSize-x1tq7rpn transform-x1u4xmye transform-xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".flexDirection-x1q0g3np{flex-direction:row}", 3000);
_inject2(".borderRadius-x1hc0yr1{border-radius:var(--md-x120tmbh)}", 2000);
_inject2(".borderWidth-xmkeg23{border-width:1px}", 2000);
_inject2(".borderStyle-x1y0btm7{border-style:solid}", 2000);
_inject2(".borderColor-xd7ip7t{border-color:var(--blue7-x1g16e7s)}", 2000);
_inject2(".padding-x1j3i37e{padding:var(--xxxs-xk88l2w)}", 1000);
_inject2(".fontFamily-x9388is{font-family:var(--fontSans-x6ywdb8)}", 3000);
_inject2(".gap-xqp6ha3{gap:var(--xs-xvp50ho)}", 2000);
_inject2(".height-x17frcva{height:6rem}", 4000);
_inject2(".aspectRatio-x1plog1{aspect-ratio:1}", 3000);
_inject2(".color-xufi7tb{color:var(--blue7-x1g16e7s)}", 3000);
_inject2(".backgroundColor-x4o57bs{background-color:var(--gray3-x1wnl0mb)}", 3000);
_inject2(".backgroundColor-x9hm0p:hover{background-color:var(--gray4-x1987uwy)}", 3130);
_inject2("@media (prefers-color-scheme: dark){.backgroundColor-xxl4oju.backgroundColor-xxl4oju{background-color:var(--gray9-xv11w9p)}}", 3200);
_inject2("@media (prefers-color-scheme: dark){.backgroundColor-xag9bkp.backgroundColor-xag9bkp:hover{background-color:var(--gray8-xd0alct)}}", 3330);
_inject2(".borderWidth-xc342km{border-width:0}", 2000);
_inject2(".borderStyle-xng3xce{border-style:none}", 2000);
_inject2(".borderRadius-xezyu2j{border-radius:var(--xs-xvp50ho)}", 2000);
_inject2(".padding-xpa3c2p{padding:var(--xs-xvp50ho)}", 1000);
_inject2(".margin-x1ots716{margin:var(--xs-xvp50ho)}", 1000);
_inject2(".cursor-x1ypdohk{cursor:pointer}", 3000);
_inject2(".fontSize-x1tq7rpn{font-size:var(--h2-x1nryaqe)}", 3000);
_inject2(".transform-x1u4xmye:hover{transform:scale(1.025)}", 3130);
_inject2(".transform-xglsxx3:active{transform:scale(.975)}", 3170);
_inject2(".fontWeight-x3stwaq{font-weight:100}", 3000);
_inject2(".color-x18c051v{color:var(--lime7-x146xnew)}", 3000);
_inject2(".minWidth-x1843ork{min-width:6rem}", 4000);
_inject2(".textAlign-x2b8uid{text-align:center}", 3000);
_inject2(".fontFamily-xovafh8{font-family:var(--fontMono-xur0yta)}", 3000);
_inject2(".fontSize-x1c27omx{font-size:var(--h3-x1yfd0fu)}", 3000);
