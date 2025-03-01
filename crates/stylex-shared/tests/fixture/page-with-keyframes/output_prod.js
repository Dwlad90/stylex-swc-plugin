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
        className: "x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 xey12qk xpywc49 x14kqxm4"
    }}>
      <div {...{
        className: "x1jfb8zj xarpa2k x1h91t0o xv4mccy xh8yej3 xhtitgo x1nlbcxq"
    }}>
        <p {...{
        className: "xjg0vao x1n2onr6 x15f3dyk xo5s888 xu8adaz x1v68ji2 x1ghz6dp x13ekbdn xqxyaa3 x12h1x1l x5b8z1 x14kqxm4 x19g2c9c xa7o7q9 xmkeg23 x1m60m6i         x1y0btm7 x15t7hjr xgepp9j x12ugs8o xd22jv         x1los6se"
    }}>
          Get started by editing&nbsp;
          <code {...{
        className: "x1xlr1w8 x1nlbcxq"
    }}>app/page.tsx</code>
        </p>
      </div>
      <div {...{
        className: "x1iyjqo2 x78zum5 xdt5ytf x6s0dn4 xl56j7k xod9s3o"
    }}>
        <h1 {...{
        className: "xn39edi xo5v014 x1byiw6p xo1l8bm x2b8uid x78zum5 xmju1pe xuxw1ft x1q0g3np xwlf911"
    }}>
          Next.js App Dir<span {...{
        className: "x1n2onr6 x6icuqf x13vifvy x1e1ljn3 x1c74tu6 xa4qsjk x1esw782"
    }}>♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div {...{
        className: "xrvj5dj xtp8ymz xx3cr9d xtffbmy x15jn8ho x193iq5w xl858mc x15hltav"
    }}>
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>;
}
