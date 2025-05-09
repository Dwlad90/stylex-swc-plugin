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
        className: "display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k flexDirection-x1q0g3np borderRadius-x18tt229 borderWidth-xmkeg23 borderStyle-x1y0btm7 borderColor-x1nasx6d padding-x1l67flk fontFamily-x1o4itb0 gap-x1mm2g2v",
        "data-style-src": "input.stylex.js:40"
    }}>
      <button {...{
        className: "display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-x194xsre backgroundColor-x1a2lmyf backgroundColor-x1oi2zhp backgroundColor-xseld47 backgroundColor-xjbufok borderWidth-xc342km borderStyle-xng3xce borderRadius-x1nklt0o padding-x1t29n93 margin-x4kdmvg cursor-x1ypdohk fontSize-xt4rhuc transform-x1u4xmye transform-xglsxx3",
        "data-style-src": "input.stylex.js:53"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...{
        0: {
            className: "fontSize-xt4rhuc fontWeight-x3stwaq color-xnu1ptm minWidth-x1843ork textAlign-x2b8uid fontFamily-xh1z4oz",
            "data-style-src": "input.stylex.js:81"
        },
        1: {
            className: "fontWeight-x3stwaq color-xnu1ptm minWidth-x1843ork textAlign-x2b8uid fontFamily-xh1z4oz fontSize-x1bb9vi5",
            "data-style-src": "input.stylex.js:81; input.stylex.js:89"
        }
    }[!!(Math.abs(count) > 99) << 0]}>
        {count}
      </div>
      <button {...{
        className: "display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-x194xsre backgroundColor-x1a2lmyf backgroundColor-x1oi2zhp backgroundColor-xseld47 backgroundColor-xjbufok borderWidth-xc342km borderStyle-xng3xce borderRadius-x1nklt0o padding-x1t29n93 margin-x4kdmvg cursor-x1ypdohk fontSize-xt4rhuc transform-x1u4xmye transform-xglsxx3",
        "data-style-src": "input.stylex.js:53"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".flexDirection-x1q0g3np{flex-direction:row}", 3000);
_inject2(".borderRadius-x18tt229{border-radius:var(--md-xz85zqu)}", 2000);
_inject2(".borderWidth-xmkeg23{border-width:1px}", 2000);
_inject2(".borderStyle-x1y0btm7{border-style:solid}", 2000);
_inject2(".borderColor-x1nasx6d{border-color:var(--blue7-x1kefbne)}", 2000);
_inject2(".padding-x1l67flk{padding:var(--xxxs-x1jgrv4s)}", 1000);
_inject2(".fontFamily-x1o4itb0{font-family:var(--fontSans-x1v0ot8g)}", 3000);
_inject2(".gap-x1mm2g2v{gap:var(--xs-x1yemeo2)}", 2000);
_inject2(".height-x17frcva{height:6rem}", 4000);
_inject2(".aspectRatio-x1plog1{aspect-ratio:1}", 3000);
_inject2(".color-x194xsre{color:var(--blue7-x1kefbne)}", 3000);
_inject2(".backgroundColor-x1a2lmyf{background-color:var(--gray3-x1h92w08)}", 3000);
_inject2(".backgroundColor-x1oi2zhp:hover{background-color:var(--gray4-xw5rm9m)}", 3130);
_inject2("@media (prefers-color-scheme: dark){.backgroundColor-xseld47.backgroundColor-xseld47{background-color:var(--gray9-x1k6ilpx)}}", 3200);
_inject2("@media (prefers-color-scheme: dark){.backgroundColor-xjbufok.backgroundColor-xjbufok:hover{background-color:var(--gray8-x1ru5ylq)}}", 3330);
_inject2(".borderWidth-xc342km{border-width:0}", 2000);
_inject2(".borderStyle-xng3xce{border-style:none}", 2000);
_inject2(".borderRadius-x1nklt0o{border-radius:var(--xs-x1yemeo2)}", 2000);
_inject2(".padding-x1t29n93{padding:var(--xs-x1yemeo2)}", 1000);
_inject2(".margin-x4kdmvg{margin:var(--xs-x1yemeo2)}", 1000);
_inject2(".cursor-x1ypdohk{cursor:pointer}", 3000);
_inject2(".fontSize-xt4rhuc{font-size:var(--h2-x1al5pe7)}", 3000);
_inject2(".transform-x1u4xmye:hover{transform:scale(1.025)}", 3130);
_inject2(".transform-xglsxx3:active{transform:scale(.975)}", 3170);
_inject2(".fontWeight-x3stwaq{font-weight:100}", 3000);
_inject2(".color-xnu1ptm{color:var(--lime7-x16pcyb)}", 3000);
_inject2(".minWidth-x1843ork{min-width:6rem}", 4000);
_inject2(".textAlign-x2b8uid{text-align:center}", 3000);
_inject2(".fontFamily-xh1z4oz{font-family:var(--fontMono-xgc26q9)}", 3000);
_inject2(".fontSize-x1bb9vi5{font-size:var(--h3-xbf52ah)}", 3000);
