'use client';
import * as stylex from '@stylexjs/stylex';
import { spacing, text, globalTokens as $ } from './globalTokens.stylex';
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { useState } from 'react';
const _temp2 = {
    kGuDYH: "xdmh292",
    $$css: true
};
export default function Counter() {
    const [count, setCount] = useState(0);
    return <div {...{
        className: "x78zum5 x6s0dn4 xl56j7k x1q0g3np x12xgqvu xmkeg23 x1y0btm7 xqpy6nh x87erls x1alyrvt x1749g51"
    }}>
      <button {...{
        className: "x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 xgopyf5 xpotius x1gk0e8 x10vqmf9 xnha941 xc342km xng3xce xvm41bv xsbzlvg x1i5nj67 x1ypdohk xtqx43c x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...stylex.props(styles.count, styles.size(count), Math.abs(count) > 99 && styles.largeNumber)}>
        {count}
      </div>
      <button {...{
        className: "x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 xgopyf5 xpotius x1gk0e8 x10vqmf9 xnha941 xc342km xng3xce xvm41bv xsbzlvg x1i5nj67 x1ypdohk xtqx43c x1u4xmye xglsxx3"
    }} onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
const styles = {
    size: (size)=>[
            _temp2,
            {
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(8 * size + 'px')
            }
        ],
    count: {
        kGuDYH: "xtqx43c",
        k63SB2: "x3stwaq",
        kMwMTN: "xxzazoc",
        k7Eaqz: "x1843ork",
        k9WMMc: "x2b8uid",
        kMv6JI: "xum72dy",
        $$css: true
    },
    largeNumber: {
        kGuDYH: "xs6c6ls",
        $$css: true
    }
};
