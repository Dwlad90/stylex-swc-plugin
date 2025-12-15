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
    return <div className="display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k flexDirection-x1q0g3np borderRadius-x18tt229 borderWidth-xmkeg23 borderStyle-x1y0btm7 borderColor-x1nasx6d padding-x1l67flk fontFamily-x1o4itb0 gap-x1mm2g2v" data-style-src="tests/fixture/counter/input.stylex.js:22">
      <button className="display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-x194xsre backgroundColor-x1a2lmyf backgroundColor-x1oi2zhp backgroundColor-xseld47 backgroundColor-xjbufok borderWidth-xc342km borderStyle-xng3xce borderRadius-x1nklt0o padding-x1t29n93 margin-x4kdmvg cursor-x1ypdohk fontSize-xt4rhuc transform-x1u4xmye transform-xglsxx3" data-style-src="tests/fixture/counter/input.stylex.js:35" onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...{
        0: {
            className: "fontSize-xt4rhuc fontWeight-x3stwaq color-xnu1ptm minWidth-x1843ork textAlign-x2b8uid fontFamily-xh1z4oz",
            "data-style-src": "tests/fixture/counter/input.stylex.js:63"
        },
        1: {
            className: "fontWeight-x3stwaq color-xnu1ptm minWidth-x1843ork textAlign-x2b8uid fontFamily-xh1z4oz fontSize-x1bb9vi5",
            "data-style-src": "tests/fixture/counter/input.stylex.js:63; tests/fixture/counter/input.stylex.js:71"
        }
    }[!!(Math.abs(count) > 99) << 0]}>
        {count}
      </div>
      <button className="display-x78zum5 alignItems-x6s0dn4 justifyContent-xl56j7k height-x17frcva aspectRatio-x1plog1 color-x194xsre backgroundColor-x1a2lmyf backgroundColor-x1oi2zhp backgroundColor-xseld47 backgroundColor-xjbufok borderWidth-xc342km borderStyle-xng3xce borderRadius-x1nklt0o padding-x1t29n93 margin-x4kdmvg cursor-x1ypdohk fontSize-xt4rhuc transform-x1u4xmye transform-xglsxx3" data-style-src="tests/fixture/counter/input.stylex.js:35" onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
_inject2({
    ltr: ".display-x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".alignItems-x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".justifyContent-xl56j7k{justify-content:center}",
    priority: 3000
});
_inject2({
    ltr: ".flexDirection-x1q0g3np{flex-direction:row}",
    priority: 3000
});
_inject2({
    ltr: ".borderRadius-x18tt229{border-radius:var(--md-xz85zqu)}",
    priority: 2000
});
_inject2({
    ltr: ".borderWidth-xmkeg23{border-width:1px}",
    priority: 2000
});
_inject2({
    ltr: ".borderStyle-x1y0btm7{border-style:solid}",
    priority: 2000
});
_inject2({
    ltr: ".borderColor-x1nasx6d{border-color:var(--blue7-x1kefbne)}",
    priority: 2000
});
_inject2({
    ltr: ".padding-x1l67flk{padding:var(--xxxs-x1jgrv4s)}",
    priority: 1000
});
_inject2({
    ltr: ".fontFamily-x1o4itb0{font-family:var(--fontSans-x1v0ot8g)}",
    priority: 3000
});
_inject2({
    ltr: ".gap-x1mm2g2v{gap:var(--xs-x1yemeo2)}",
    priority: 2000
});
_inject2({
    ltr: ".height-x17frcva{height:6rem}",
    priority: 4000
});
_inject2({
    ltr: ".aspectRatio-x1plog1{aspect-ratio:1}",
    priority: 3000
});
_inject2({
    ltr: ".color-x194xsre{color:var(--blue7-x1kefbne)}",
    priority: 3000
});
_inject2({
    ltr: ".backgroundColor-x1a2lmyf{background-color:var(--gray3-x1h92w08)}",
    priority: 3000
});
_inject2({
    ltr: ".backgroundColor-x1oi2zhp:hover{background-color:var(--gray4-xw5rm9m)}",
    priority: 3130
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){.backgroundColor-xseld47.backgroundColor-xseld47{background-color:var(--gray9-x1k6ilpx)}}",
    priority: 3200
});
_inject2({
    ltr: "@media (prefers-color-scheme: dark){.backgroundColor-xjbufok.backgroundColor-xjbufok:hover{background-color:var(--gray8-x1ru5ylq)}}",
    priority: 3330
});
_inject2({
    ltr: ".borderWidth-xc342km{border-width:0}",
    priority: 2000
});
_inject2({
    ltr: ".borderStyle-xng3xce{border-style:none}",
    priority: 2000
});
_inject2({
    ltr: ".borderRadius-x1nklt0o{border-radius:var(--xs-x1yemeo2)}",
    priority: 2000
});
_inject2({
    ltr: ".padding-x1t29n93{padding:var(--xs-x1yemeo2)}",
    priority: 1000
});
_inject2({
    ltr: ".margin-x4kdmvg{margin:var(--xs-x1yemeo2)}",
    priority: 1000
});
_inject2({
    ltr: ".cursor-x1ypdohk{cursor:pointer}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-xt4rhuc{font-size:var(--h2-x1al5pe7)}",
    priority: 3000
});
_inject2({
    ltr: ".transform-x1u4xmye:hover{transform:scale(1.025)}",
    priority: 3130
});
_inject2({
    ltr: ".transform-xglsxx3:active{transform:scale(.975)}",
    priority: 3170
});
_inject2({
    ltr: ".fontWeight-x3stwaq{font-weight:100}",
    priority: 3000
});
_inject2({
    ltr: ".color-xnu1ptm{color:var(--lime7-x16pcyb)}",
    priority: 3000
});
_inject2({
    ltr: ".minWidth-x1843ork{min-width:6rem}",
    priority: 4000
});
_inject2({
    ltr: ".textAlign-x2b8uid{text-align:center}",
    priority: 3000
});
_inject2({
    ltr: ".fontFamily-xh1z4oz{font-family:var(--fontMono-xgc26q9)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-x1bb9vi5{font-size:var(--h3-xbf52ah)}",
    priority: 3000
});
