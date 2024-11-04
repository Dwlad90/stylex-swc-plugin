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
      <div {...stylex.props(styles.count, styles.size(count), Math.abs(count) > 99 && styles.largeNumber)}>
        {count}
      </div>
      <button {...{
        className: "x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 x1ynku2j xij5jp x6lnu34 xd84qqf x1lod3q0 xc342km xng3xce x12ugs8o x1kopudh xp822f4 x1ypdohk xf8wwq x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
const styles = {
    size: (size)=>[
            {
                fontSize: 8 * size + 'px' == null ? null : "x6zurak",
                $$css: true
            },
            {
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        fontSize: "xf8wwq",
        fontWeight: "x3stwaq",
        color: "x1fk3gbn",
        minWidth: "x1843ork",
        textAlign: "x2b8uid",
        fontFamily: "x1nlbcxq",
        $$css: true
    },
    largeNumber: {
        fontSize: "x8c9cfh",
        $$css: true
    }
};
