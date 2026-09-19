import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./globalTokens.stylex";
/**
 * Lead Comment
 */ import React from 'react';
import * as stylex from '@stylexjs/stylex';
import Card from '@/components/Card';
import { globalTokens as $, spacing, text, scales } from './globalTokens.stylex';
import Counter from './Counter';
const HOMEPAGE = 'https://stylexjs.com';
export default function Home() {
    return(// @ts-expect-error - sx is not correctly typed
    <main className="input__style.main x78zum5 xdt5ytf x6s0dn4 x1qughib xg6iff7 x5zw9ho x1jlena x1hs85sq" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:75">
      <div className="input__style.description x1jfb8zj xarpa2k x1h91t0o xlql8t6 xh8yej3 xhtitgo xum72dy" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:121">
        <p className="input__style.descP xjg0vao x1n2onr6 x15f3dyk xo5s888 xu8adaz x1v68ji2 x1ghz6dp x2jueht xx1ocoh xzfjryi x1xwseyw x1hs85sq xhxofkm x1rgf71s xmkeg23 x1m60m6i x1y0btm7 xbc1r81 x1e6d3oi xvm41bv xd22jv x1los6se" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:138">
          Get started by editing&nbsp;
          <code className="input__style.code x1xlr1w8 xum72dy" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:178">app/page.tsx</code>
        </p>
      </div>
      <div className="input__style.hero x1iyjqo2 x78zum5 xdt5ytf x6s0dn4 xl56j7k x1fvhq8d" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:87">
        <h1 className="input__style.h1 x1kg2sfr xo5v014 x1alyrvt xo1l8bm x2b8uid x78zum5 xecefrz xuxw1ft x1q0g3np xwlf911" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:95">
          Next.js App Dir<span className="input__style.emoji x1n2onr6 x6icuqf x13vifvy xe3njyp x1c74tu6 xa4qsjk x1esw782" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:109">♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div className="input__style.grid xrvj5dj xtp8ymz xx3cr9d xtffbmy x1xhi074 x193iq5w xl858mc x15hltav" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:182">
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>);
}
_inject2({
    ltr: "@keyframes xvkw4ej-B{0%{transform:var(--x1m1o2d4);}10%{transform:var(--x13z98d3);}20%{transform:var(--x1m1o2d4);}30%{transform:var(--x13z98d3);}40%{transform:var(--x1m1o2d4);}90%{transform:var(--xrkhmu4);}100%{transform:var(--x1m1o2d4);}}",
    priority: 0
});
_inject2({
    ltr: ".x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".xdt5ytf{flex-direction:column}",
    priority: 3000
});
_inject2({
    ltr: ".x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".x1qughib{justify-content:space-between}",
    priority: 3000
});
_inject2({
    ltr: ".xg6iff7{min-height:100vh}",
    priority: 4000
});
_inject2({
    ltr: ".x5zw9ho{padding-top:var(--x1jb9yn9)}",
    priority: 4000
});
_inject2({
    ltr: ".x1jlena{padding-bottom:var(--x1jb9yn9)}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.x1hs85sq.x1hs85sq{padding-bottom:var(--xz85zqu)}}",
    priority: 4200
});
_inject2({
    ltr: ".x1iyjqo2{flex-grow:1}",
    priority: 3000
});
_inject2({
    ltr: ".xl56j7k{justify-content:center}",
    priority: 3000
});
_inject2({
    ltr: ".x1fvhq8d{gap:var(--x1btcnwp)}",
    priority: 2000
});
_inject2({
    ltr: ".x1kg2sfr{font-size:var(--x6bx092)}",
    priority: 3000
});
_inject2({
    ltr: ".xo5v014{line-height:1}",
    priority: 3000
});
_inject2({
    ltr: ".x1alyrvt{font-family:var(--x1v0ot8g)}",
    priority: 3000
});
_inject2({
    ltr: ".xo1l8bm{font-weight:400}",
    priority: 3000
});
_inject2({
    ltr: ".x2b8uid{text-align:center}",
    priority: 3000
});
_inject2({
    ltr: ".xecefrz{gap:var(--xz85zqu)}",
    priority: 2000
});
_inject2({
    ltr: ".xuxw1ft{white-space:nowrap}",
    priority: 3000
});
_inject2({
    ltr: ".x1q0g3np{flex-direction:row}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 700px){.xwlf911.xwlf911{flex-direction:column}}",
    priority: 3200
});
_inject2({
    ltr: ".x1n2onr6{position:relative}",
    priority: 3000
});
_inject2({
    ltr: ".x6icuqf{font-family:sans-serif}",
    priority: 3000
});
_inject2({
    ltr: ".x13vifvy{top:0}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.xe3njyp.xe3njyp{top:var(--x1jgrv4s)}}",
    priority: 4200
});
_inject2({
    ltr: ".x1c74tu6{animation-duration:2s}",
    priority: 3000
});
_inject2({
    ltr: ".xa4qsjk{animation-iteration-count:infinite}",
    priority: 3000
});
_inject2({
    ltr: ".x1esw782{animation-timing-function:linear}",
    priority: 3000
});
_inject2({
    ltr: ".x1jfb8zj{display:inherit}",
    priority: 3000
});
_inject2({
    ltr: ".xarpa2k{justify-content:inherit}",
    priority: 3000
});
_inject2({
    ltr: ".x1h91t0o{align-items:inherit}",
    priority: 3000
});
_inject2({
    ltr: ".xlql8t6{max-width:var(--x110of7g)}",
    priority: 4000
});
_inject2({
    ltr: ".xh8yej3{width:100%}",
    priority: 4000
});
_inject2({
    ltr: ".xhtitgo{z-index:2}",
    priority: 3000
});
_inject2({
    ltr: ".xum72dy{font-family:var(--xgc26q9)}",
    priority: 3000
});
_inject2({
    ltr: ".x168cpw9{gap:var(--xtt9l4u)}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.x1bnx9ql.x1bnx9ql{padding:var(--x1k0pbdz)}}",
    priority: 1200
});
_inject2({
    ltr: "@media (max-width: 700px){.xjg0vao.xjg0vao{display:flex}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.x15f3dyk.x15f3dyk{position:fixed}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.xo5s888.xo5s888{justify-content:center}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.xu8adaz.xu8adaz{align-items:center}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.x1v68ji2.x1v68ji2{width:100%}}",
    priority: 4200
});
_inject2({
    ltr: ".x1ghz6dp{margin:0}",
    priority: 1000
});
_inject2({
    ltr: ".x2jueht{padding-inline:var(--x1k0pbdz)}",
    priority: 2000
});
_inject2({
    ltr: ".xx1ocoh{padding-top:var(--x1k0pbdz)}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.xzfjryi.xzfjryi{padding-top:var(--xu29097)}}",
    priority: 4200
});
_inject2({
    ltr: ".x1xwseyw{padding-bottom:var(--x1k0pbdz)}",
    priority: 4000
});
_inject2({
    ltr: ".xhxofkm{background-color:var(--x11lpu6b)}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 700px){.x1rgf71s.x1rgf71s{background-image:linear-gradient(to bottom,var(--x1txk845),var(--x11lpu6b))}}",
    priority: 3200
});
_inject2({
    ltr: ".xmkeg23{border-width:1px}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.x1m60m6i.x1m60m6i{border-width:0}}",
    priority: 2200
});
_inject2({
    ltr: ".x1y0btm7{border-style:solid}",
    priority: 2000
});
_inject2({
    ltr: ".xbc1r81{border-color:rgba(var(--x1tfbujh),var(--x1eglwg0),var(--xgpbt7a),.3)}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.x1e6d3oi.x1e6d3oi{border-bottom-color:rgba(var(--x1tfbujh),var(--x1eglwg0),var(--xgpbt7a),.25)}}",
    priority: 4200
});
_inject2({
    ltr: ".xvm41bv{border-radius:var(--x1yemeo2)}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.xd22jv.xd22jv{border-radius:0}}",
    priority: 2200
});
_inject2({
    ltr: "@media (max-width: 700px){.x1los6se.x1los6se{inset:0 0 auto}}",
    priority: 1200
});
_inject2({
    ltr: ".x1xlr1w8{font-weight:700}",
    priority: 3000
});
_inject2({
    ltr: ".xrvj5dj{display:grid}",
    priority: 3000
});
_inject2({
    ltr: ".xtp8ymz{grid-template-columns:repeat(4,minmax(25%,auto))}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 700px){.xx3cr9d.xx3cr9d{grid-template-columns:1fr}}",
    priority: 3200
});
_inject2({
    ltr: "@media (min-width: 701px) and (max-width: 1120px){.xtffbmy.xtffbmy{grid-template-columns:repeat(2,50%)}}",
    priority: 3200
});
_inject2({
    ltr: ".x1xhi074{width:var(--x110of7g)}",
    priority: 4000
});
_inject2({
    ltr: ".x193iq5w{max-width:100%}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.xl858mc.xl858mc{max-width:320px}}",
    priority: 4200
});
_inject2({
    ltr: "@media (max-width: 700px){.x15hltav.x15hltav{text-align:center}}",
    priority: 3200
});
