/**
 * Lead Comment
 */ import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./globalTokens.stylex";
import React from 'react';
import stylex from '@stylexjs/stylex';
import Card from '@/components/Card';
import { globalTokens as $, spacing, text, scales } from './globalTokens.stylex';
import Counter from './Counter';
const HOMEPAGE = 'https://stylexjs.com';
export default function Home() {
    return <main {...{
        className: "Page__style.main x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 xey12qk xpywc49 x14kqxm4"
    }}>
      <div {...{
        className: "Page__style.description x1jfb8zj xarpa2k x1h91t0o xv4mccy xh8yej3 xhtitgo x1nlbcxq"
    }}>
        <p {...{
        className: "Page__style.descP xjg0vao x1n2onr6 x15f3dyk xo5s888 xu8adaz x1v68ji2 x1ghz6dp x13ekbdn xqxyaa3 x12h1x1l x5b8z1 x14kqxm4 x19g2c9c xa7o7q9 xmkeg23 x1m60m6i         x1y0btm7 x15t7hjr xgepp9j x12ugs8o xd22jv         x1los6se"
    }}>
          Get started by editing 
          <code {...{
        className: "Page__style.code x1xlr1w8 x1nlbcxq"
    }}>app/page.tsx</code>
        </p>
      </div>
      <div {...{
        className: "Page__style.hero x1iyjqo2 x78zum5 xdt5ytf x6s0dn4 xl56j7k xod9s3o"
    }}>
        <h1 {...{
        className: "Page__style.h1 xn39edi xo5v014 x1byiw6p xo1l8bm x2b8uid x78zum5 xmju1pe xuxw1ft x1q0g3np xwlf911"
    }}>
          Next.js App Dir<span {...{
        className: "Page__style.emoji x1n2onr6 x6icuqf x13vifvy x1e1ljn3 x1c74tu6 xa4qsjk x1esw782"
    }}>♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div {...{
        className: "Page__style.grid xrvj5dj xtp8ymz xx3cr9d xtffbmy x15jn8ho x193iq5w xl858mc x15hltav"
    }}>
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>;
}
_inject2("@keyframes x1b14oj3-B{0%{transform:var(--x8qjy7n);}10%{transform:var(--x16qhacm);}20%{transform:var(--x8qjy7n);}30%{transform:var(--x16qhacm);}40%{transform:var(--x8qjy7n);}90%{transform:var(--xg58vmv);}100%{transform:var(--x8qjy7n);}}", 1);
_inject2(".x78zum5{display:flex}", 3000);
_inject2(".xdt5ytf{flex-direction:column}", 3000);
_inject2(".x6s0dn4{align-items:center}", 3000);
_inject2(".x1qughib{justify-content:space-between}", 3000);
_inject2(".xg6iff7{min-height:100vh}", 4000);
_inject2(".xey12qk{padding-top:var(--xrreorb)}", 4000);
_inject2(".xpywc49{padding-bottom:var(--xrreorb)}", 4000);
_inject2("@media (max-width: 700px){.x14kqxm4.x14kqxm4{padding-bottom:var(--x120tmbh)}}", 4200);
_inject2(".x1iyjqo2{flex-grow:1}", 3000);
_inject2(".xl56j7k{justify-content:center}", 3000);
_inject2(".xod9s3o{gap:var(--xqbuwcu)}", 2000);
_inject2(".xn39edi{font-size:var(--x1fnzu0q)}", 3000);
_inject2(".xo5v014{line-height:1}", 3000);
_inject2(".x1byiw6p{font-family:var(--x6ywdb8)}", 3000);
_inject2(".xo1l8bm{font-weight:400}", 3000);
_inject2(".x2b8uid{text-align:center}", 3000);
_inject2(".xmju1pe{gap:var(--x120tmbh)}", 2000);
_inject2(".xuxw1ft{white-space:nowrap}", 3000);
_inject2(".x1q0g3np{flex-direction:row}", 3000);
_inject2("@media (max-width: 700px){.xwlf911.xwlf911{flex-direction:column}}", 3200);
_inject2(".x1n2onr6{position:relative}", 3000);
_inject2(".x6icuqf{font-family:sans-serif}", 3000);
_inject2(".x13vifvy{top:0}", 4000);
_inject2("@media (max-width: 700px){.x1e1ljn3.x1e1ljn3{top:var(--xk88l2w)}}", 4200);
_inject2(".x1c74tu6{animation-duration:2s}", 3000);
_inject2(".xa4qsjk{animation-iteration-count:infinite}", 3000);
_inject2(".x1esw782{animation-timing-function:linear}", 3000);
_inject2(".x1jfb8zj{display:inherit}", 3000);
_inject2(".xarpa2k{justify-content:inherit}", 3000);
_inject2(".x1h91t0o{align-items:inherit}", 3000);
_inject2(".xv4mccy{max-width:var(--xt7qi6)}", 4000);
_inject2(".xh8yej3{width:100%}", 4000);
_inject2(".xhtitgo{z-index:2}", 3000);
_inject2(".x1nlbcxq{font-family:var(--xur0yta)}", 3000);
_inject2(".xnp4naa{gap:var(--xmf2usz)}", 2000);
_inject2("@media (max-width: 700px){.x1ivusqq.x1ivusqq{padding:var(--x1bfynh1)}}", 1200);
_inject2("@media (max-width: 700px){.xjg0vao.xjg0vao{display:flex}}", 3200);
_inject2("@media (max-width: 700px){.x15f3dyk.x15f3dyk{position:fixed}}", 3200);
_inject2("@media (max-width: 700px){.xo5s888.xo5s888{justify-content:center}}", 3200);
_inject2("@media (max-width: 700px){.xu8adaz.xu8adaz{align-items:center}}", 3200);
_inject2("@media (max-width: 700px){.x1v68ji2.x1v68ji2{width:100%}}", 4200);
_inject2(".x1ghz6dp{margin:0}", 1000);
_inject2(".x13ekbdn{padding-inline:var(--x1bfynh1)}", 2000);
_inject2(".xqxyaa3{padding-top:var(--x1bfynh1)}", 4000);
_inject2("@media (max-width: 700px){.x12h1x1l.x12h1x1l{padding-top:var(--x83l8dq)}}", 4200);
_inject2(".x5b8z1{padding-bottom:var(--x1bfynh1)}", 4000);
_inject2(".x19g2c9c{background-color:var(--x1xmsgwt)}", 3000);
_inject2("@media (max-width: 700px){.xa7o7q9.xa7o7q9{background-image:linear-gradient(to bottom,var(--x1i5pq9l),var(--x1xmsgwt))}}", 3200);
_inject2(".xmkeg23{border-width:1px}", 2000);
_inject2("@media (max-width: 700px){.x1m60m6i.x1m60m6i{border-width:0}}", 2200);
_inject2(".x1y0btm7{border-style:solid}", 2000);
_inject2(".x15t7hjr{border-color:rgba(var(--xodl1w7),var(--x2p453m),var(--x140pla3),.3)}", 2000);
_inject2("@media (max-width: 700px){.xgepp9j.xgepp9j{border-bottom-color:rgba(var(--xodl1w7),var(--x2p453m),var(--x140pla3),.25)}}", 4200);
_inject2(".x12ugs8o{border-radius:var(--xvp50ho)}", 2000);
_inject2("@media (max-width: 700px){.xd22jv.xd22jv{border-radius:0}}", 2200);
_inject2("@media (max-width: 700px){.x1los6se.x1los6se{inset:0 0 auto}}", 1200);
_inject2(".x1xlr1w8{font-weight:700}", 3000);
_inject2(".xrvj5dj{display:grid}", 3000);
_inject2(".xtp8ymz{grid-template-columns:repeat(4,minmax(25%,auto))}", 3000);
_inject2("@media (max-width: 700px){.xx3cr9d.xx3cr9d{grid-template-columns:1fr}}", 3200);
_inject2("@media (min-width: 701px) and (max-width: 1120px){.xtffbmy.xtffbmy{grid-template-columns:repeat(2,50%)}}", 3200);
_inject2(".x15jn8ho{width:var(--xt7qi6)}", 4000);
_inject2(".x193iq5w{max-width:100%}", 4000);
_inject2("@media (max-width: 700px){.xl858mc.xl858mc{max-width:320px}}", 4200);
_inject2("@media (max-width: 700px){.x15hltav.x15hltav{text-align:center}}", 3200);
