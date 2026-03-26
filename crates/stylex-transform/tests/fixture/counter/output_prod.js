'use client';
import * as stylex from '@stylexjs/stylex';
import { spacing, text, globalTokens as $ } from './globalTokens.stylex';
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { useState } from 'react';
export default function Counter() {
    const [count, setCount] = useState(0);
    return <div className="x78zum5 x6s0dn4 xl56j7k x1q0g3np x12xgqvu xmkeg23 x1y0btm7 xqpy6nh x87erls x1alyrvt x1749g51">
      <button className="x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 xgopyf5 xpotius x1gk0e8 x10vqmf9 xnha941 xc342km xng3xce xvm41bv xsbzlvg x1i5nj67 x1ypdohk xtqx43c x1u4xmye xglsxx3" onClick={()=>setCount((c)=>c - 1)}>
        -
      </button>
      <div {...{
        0: {
            className: "xtqx43c x3stwaq xxzazoc x1843ork x2b8uid xum72dy"
        },
        1: {
            className: "x3stwaq xxzazoc x1843ork x2b8uid xum72dy xs6c6ls"
        }
    }[!!(Math.abs(count) > 99) << 0]}>
        {count}
      </div>
      <button className="x78zum5 x6s0dn4 xl56j7k x17frcva x1plog1 xgopyf5 xpotius x1gk0e8 x10vqmf9 xnha941 xc342km xng3xce xvm41bv xsbzlvg x1i5nj67 x1ypdohk xtqx43c x1u4xmye xglsxx3" onClick={()=>setCount((c)=>c + 1)}>
        +
      </button>
    </div>;
}
