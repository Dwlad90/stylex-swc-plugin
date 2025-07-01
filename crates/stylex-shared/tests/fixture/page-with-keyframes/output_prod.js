/**
 * Lead Comment
 */ import React from 'react';
import * as stylex from '@stylexjs/stylex';
import Card from '@/components/Card';
import { globalTokens as $, spacing, text, scales } from './globalTokens.stylex';
import Counter from './Counter';
const HOMEPAGE = 'https://stylexjs.com';
export default function Home() {
    return <main {...{
        className: "x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 x5zw9ho x1jlena x1hs85sq"
    }}>
      <div {...{
        className: "x1jfb8zj xarpa2k x1h91t0o xlql8t6 xh8yej3 xhtitgo xum72dy"
    }}>
        <p {...{
        className: "xjg0vao x1n2onr6 x15f3dyk xo5s888 xu8adaz x1v68ji2 x1ghz6dp x2jueht xx1ocoh xzfjryi x1xwseyw x1hs85sq xhxofkm x1rgf71s xmkeg23 x1m60m6i x1y0btm7 xbc1r81 x1e6d3oi xvm41bv xd22jv x1los6se"
    }}>
          Get started by editing&nbsp;
          <code {...{
        className: "x1xlr1w8 xum72dy"
    }}>app/page.tsx</code>
        </p>
      </div>
      <div {...{
        className: "x1iyjqo2 x78zum5 xdt5ytf x6s0dn4 xl56j7k x1fvhq8d"
    }}>
        <h1 {...{
        className: "x1kg2sfr xo5v014 x1alyrvt xo1l8bm x2b8uid x78zum5 xecefrz xuxw1ft x1q0g3np xwlf911"
    }}>
          Next.js App Dir<span {...{
        className: "x1n2onr6 x6icuqf x13vifvy xe3njyp x1c74tu6 xa4qsjk x1esw782"
    }}>♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div {...{
        className: "xrvj5dj xtp8ymz xx3cr9d xtffbmy x1xhi074 x193iq5w xl858mc x15hltav"
    }}>
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>;
}
