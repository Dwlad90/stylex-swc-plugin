//__stylex_metadata_start__[{"class_name":"fontSize-x6zurak","style":{"rtl":null,"ltr":".fontSize-x6zurak{font-size:var(--fontSize)}"},"priority":3000},{"class_name":"display-x78zum5","style":{"rtl":null,"ltr":".display-x78zum5{display:flex}"},"priority":3000},{"class_name":"alignItems-x6s0dn4","style":{"rtl":null,"ltr":".alignItems-x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"justifyContent-xl56j7k","style":{"rtl":null,"ltr":".justifyContent-xl56j7k{justify-content:center}"},"priority":3000},{"class_name":"flexDirection-x1q0g3np","style":{"rtl":null,"ltr":".flexDirection-x1q0g3np{flex-direction:row}"},"priority":3000},{"class_name":"borderRadius-xkorlav","style":{"rtl":null,"ltr":".borderRadius-xkorlav{border-radius:var(--x120tmbh)}"},"priority":2000},{"class_name":"borderWidth-xmkeg23","style":{"rtl":null,"ltr":".borderWidth-xmkeg23{border-width:1px}"},"priority":2000},{"class_name":"borderStyle-x1y0btm7","style":{"rtl":null,"ltr":".borderStyle-x1y0btm7{border-style:solid}"},"priority":2000},{"class_name":"borderColor-xzj82u7","style":{"rtl":null,"ltr":".borderColor-xzj82u7{border-color:var(--x1g16e7s)}"},"priority":2000},{"class_name":"padding-xhcr65l","style":{"rtl":null,"ltr":".padding-xhcr65l{padding:var(--xk88l2w)}"},"priority":1000},{"class_name":"fontFamily-x1byiw6p","style":{"rtl":null,"ltr":".fontFamily-x1byiw6p{font-family:var(--x6ywdb8)}"},"priority":3000},{"class_name":"gap-x1l7lfc5","style":{"rtl":null,"ltr":".gap-x1l7lfc5{gap:var(--xvp50ho)}"},"priority":2000},{"class_name":"height-x17frcva","style":{"rtl":null,"ltr":".height-x17frcva{height:6rem}"},"priority":4000},{"class_name":"aspectRatio-x1plog1","style":{"rtl":null,"ltr":".aspectRatio-x1plog1{aspect-ratio:1}"},"priority":3000},{"class_name":"color-x1ynku2j","style":{"rtl":null,"ltr":".color-x1ynku2j{color:var(--x1g16e7s)}"},"priority":3000},{"class_name":"backgroundColor-xij5jp","style":{"rtl":null,"ltr":".backgroundColor-xij5jp{background-color:var(--x1wnl0mb)}"},"priority":3000},{"class_name":"backgroundColor-x6lnu34","style":{"rtl":null,"ltr":".backgroundColor-x6lnu34:hover{background-color:var(--x1987uwy)}"},"priority":3130},{"class_name":"backgroundColor-xd84qqf","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){.backgroundColor-xd84qqf.backgroundColor-xd84qqf{background-color:var(--xv11w9p)}}"},"priority":3200},{"class_name":"backgroundColor-x1lod3q0","style":{"rtl":null,"ltr":"@media (prefers-color-scheme: dark){.backgroundColor-x1lod3q0.backgroundColor-x1lod3q0:hover{background-color:var(--xd0alct)}}"},"priority":3330},{"class_name":"borderWidth-xc342km","style":{"rtl":null,"ltr":".borderWidth-xc342km{border-width:0}"},"priority":2000},{"class_name":"borderStyle-xng3xce","style":{"rtl":null,"ltr":".borderStyle-xng3xce{border-style:none}"},"priority":2000},{"class_name":"borderRadius-x12ugs8o","style":{"rtl":null,"ltr":".borderRadius-x12ugs8o{border-radius:var(--xvp50ho)}"},"priority":2000},{"class_name":"padding-x1kopudh","style":{"rtl":null,"ltr":".padding-x1kopudh{padding:var(--xvp50ho)}"},"priority":1000},{"class_name":"margin-xp822f4","style":{"rtl":null,"ltr":".margin-xp822f4{margin:var(--xvp50ho)}"},"priority":1000},{"class_name":"cursor-x1ypdohk","style":{"rtl":null,"ltr":".cursor-x1ypdohk{cursor:pointer}"},"priority":3000},{"class_name":"fontSize-xf8wwq","style":{"rtl":null,"ltr":".fontSize-xf8wwq{font-size:var(--x1nryaqe)}"},"priority":3000},{"class_name":"transform-x1u4xmye","style":{"rtl":null,"ltr":".transform-x1u4xmye:hover{transform:scale(1.025)}"},"priority":3130},{"class_name":"transform-xglsxx3","style":{"rtl":null,"ltr":".transform-xglsxx3:active{transform:scale(.975)}"},"priority":3170},{"class_name":"fontWeight-x3stwaq","style":{"rtl":null,"ltr":".fontWeight-x3stwaq{font-weight:100}"},"priority":3000},{"class_name":"color-x1fk3gbn","style":{"rtl":null,"ltr":".color-x1fk3gbn{color:var(--x146xnew)}"},"priority":3000},{"class_name":"minWidth-x1843ork","style":{"rtl":null,"ltr":".minWidth-x1843ork{min-width:6rem}"},"priority":4000},{"class_name":"textAlign-x2b8uid","style":{"rtl":null,"ltr":".textAlign-x2b8uid{text-align:center}"},"priority":3000},{"class_name":"fontFamily-x1nlbcxq","style":{"rtl":null,"ltr":".fontFamily-x1nlbcxq{font-family:var(--xur0yta)}"},"priority":3000},{"class_name":"fontSize-x8c9cfh","style":{"rtl":null,"ltr":".fontSize-x8c9cfh{font-size:var(--x1yfd0fu)}"},"priority":3000}]__stylex_metadata_end__
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
      <div {...stylex.props(styles.count, styles.size(count), Math.abs(count) > 99 && styles.largeNumber)}>
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
_inject2(".fontSize-x6zurak{font-size:var(--fontSize)}", 3000);
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
const styles = {
    size: (size)=>[
            {
                "Page__styles.size": "Page__styles.size",
                fontSize: 8 * size + 'px' == null ? null : "fontSize-x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        "Page__styles.count": "Page__styles.count",
        fontSize: "fontSize-xf8wwq",
        fontWeight: "fontWeight-x3stwaq",
        color: "color-x1fk3gbn",
        minWidth: "minWidth-x1843ork",
        textAlign: "textAlign-x2b8uid",
        fontFamily: "fontFamily-x1nlbcxq",
        $$css: true
    },
    largeNumber: {
        "Page__styles.largeNumber": "Page__styles.largeNumber",
        fontSize: "fontSize-x8c9cfh",
        $$css: true
    }
};
