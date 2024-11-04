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
        className: "Page__style.main display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-x1qughib minHeight-xg6iff7 paddingTop-xey12qk paddingBottom-xpywc49 paddingBottom-x14kqxm4"
    }}>
      <div {...{
        className: "Page__style.description display-x1jfb8zj justifyContent-xarpa2k alignItems-x1h91t0o maxWidth-xv4mccy width-xh8yej3 zIndex-xhtitgo fontFamily-x1nlbcxq"
    }}>
        <p {...{
        className: "Page__style.descP display-xjg0vao position-x1n2onr6 position-x15f3dyk justifyContent-xo5s888 alignItems-xu8adaz width-x1v68ji2 margin-x1ghz6dp paddingInline-x13ekbdn paddingTop-xqxyaa3 paddingTop-x12h1x1l paddingBottom-x5b8z1 paddingBottom-x14kqxm4 backgroundColor-x19g2c9c backgroundImage-xa7o7q9 borderWidth-xmkeg23 borderWidth-x1m60m6i         borderStyle-x1y0btm7 borderColor-x15t7hjr borderBottomColor-xgepp9j borderRadius-x12ugs8o borderRadius-xd22jv         inset-x1los6se"
    }}>
          Get started by editing 
          <code {...{
        className: "Page__style.code fontWeight-x1xlr1w8 fontFamily-x1nlbcxq"
    }}>app/page.tsx</code>
        </p>
      </div>
      <div {...{
        className: "Page__style.hero flexGrow-x1iyjqo2 display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-xl56j7k gap-xod9s3o"
    }}>
        <h1 {...{
        className: "Page__style.h1 fontSize-xn39edi lineHeight-xo5v014 fontFamily-x1byiw6p fontWeight-xo1l8bm textAlign-x2b8uid display-x78zum5 gap-xmju1pe whiteSpace-xuxw1ft flexDirection-x1q0g3np flexDirection-xwlf911"
    }}>
          Next.js App Dir<span {...{
        className: "Page__style.emoji position-x1n2onr6 fontFamily-x6icuqf top-x13vifvy top-x1e1ljn3 animationDuration-x1c74tu6 animationIterationCount-xa4qsjk animationTimingFunction-x1esw782"
    }}>♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div {...{
        className: "Page__style.grid display-xrvj5dj gridTemplateColumns-xtp8ymz gridTemplateColumns-xx3cr9d gridTemplateColumns-xtffbmy width-x15jn8ho maxWidth-x193iq5w maxWidth-xl858mc textAlign-x15hltav"
    }}>
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>;
}
_inject2("@keyframes x1b14oj3-B{0%{transform:var(--x8qjy7n);}10%{transform:var(--x16qhacm);}20%{transform:var(--x8qjy7n);}30%{transform:var(--x16qhacm);}40%{transform:var(--x8qjy7n);}90%{transform:var(--xg58vmv);}100%{transform:var(--x8qjy7n);}}", 1);
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".flexDirection-xdt5ytf{flex-direction:column}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-x1qughib{justify-content:space-between}", 3000);
_inject2(".minHeight-xg6iff7{min-height:100vh}", 4000);
_inject2(".paddingTop-xey12qk{padding-top:var(--xrreorb)}", 4000);
_inject2(".paddingBottom-xpywc49{padding-bottom:var(--xrreorb)}", 4000);
_inject2("@media (max-width: 700px){.paddingBottom-x14kqxm4.paddingBottom-x14kqxm4{padding-bottom:var(--x120tmbh)}}", 4200);
_inject2(".flexGrow-x1iyjqo2{flex-grow:1}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".gap-xod9s3o{gap:var(--xqbuwcu)}", 2000);
_inject2(".fontSize-xn39edi{font-size:var(--x1fnzu0q)}", 3000);
_inject2(".lineHeight-xo5v014{line-height:1}", 3000);
_inject2(".fontFamily-x1byiw6p{font-family:var(--x6ywdb8)}", 3000);
_inject2(".fontWeight-xo1l8bm{font-weight:400}", 3000);
_inject2(".textAlign-x2b8uid{text-align:center}", 3000);
_inject2(".gap-xmju1pe{gap:var(--x120tmbh)}", 2000);
_inject2(".whiteSpace-xuxw1ft{white-space:nowrap}", 3000);
_inject2(".flexDirection-x1q0g3np{flex-direction:row}", 3000);
_inject2("@media (max-width: 700px){.flexDirection-xwlf911.flexDirection-xwlf911{flex-direction:column}}", 3200);
_inject2(".position-x1n2onr6{position:relative}", 3000);
_inject2(".fontFamily-x6icuqf{font-family:sans-serif}", 3000);
_inject2(".top-x13vifvy{top:0}", 4000);
_inject2("@media (max-width: 700px){.top-x1e1ljn3.top-x1e1ljn3{top:var(--xk88l2w)}}", 4200);
_inject2(".animationDuration-x1c74tu6{animation-duration:2s}", 3000);
_inject2(".animationIterationCount-xa4qsjk{animation-iteration-count:infinite}", 3000);
_inject2(".animationTimingFunction-x1esw782{animation-timing-function:linear}", 3000);
_inject2(".display-x1jfb8zj{display:inherit}", 3000);
_inject2(".justifyContent-xarpa2k{justify-content:inherit}", 3000);
_inject2(".alignItems-x1h91t0o{align-items:inherit}", 3000);
_inject2(".maxWidth-xv4mccy{max-width:var(--xt7qi6)}", 4000);
_inject2(".width-xh8yej3{width:100%}", 4000);
_inject2(".zIndex-xhtitgo{z-index:2}", 3000);
_inject2(".fontFamily-x1nlbcxq{font-family:var(--xur0yta)}", 3000);
_inject2(".gap-xnp4naa{gap:var(--xmf2usz)}", 2000);
_inject2("@media (max-width: 700px){.padding-x1ivusqq.padding-x1ivusqq{padding:var(--x1bfynh1)}}", 1200);
_inject2("@media (max-width: 700px){.display-xjg0vao.display-xjg0vao{display:flex}}", 3200);
_inject2("@media (max-width: 700px){.position-x15f3dyk.position-x15f3dyk{position:fixed}}", 3200);
_inject2("@media (max-width: 700px){.justifyContent-xo5s888.justifyContent-xo5s888{justify-content:center}}", 3200);
_inject2("@media (max-width: 700px){.alignItems-xu8adaz.alignItems-xu8adaz{align-items:center}}", 3200);
_inject2("@media (max-width: 700px){.width-x1v68ji2.width-x1v68ji2{width:100%}}", 4200);
_inject2(".margin-x1ghz6dp{margin:0}", 1000);
_inject2(".paddingInline-x13ekbdn{padding-inline:var(--x1bfynh1)}", 2000);
_inject2(".paddingTop-xqxyaa3{padding-top:var(--x1bfynh1)}", 4000);
_inject2("@media (max-width: 700px){.paddingTop-x12h1x1l.paddingTop-x12h1x1l{padding-top:var(--x83l8dq)}}", 4200);
_inject2(".paddingBottom-x5b8z1{padding-bottom:var(--x1bfynh1)}", 4000);
_inject2(".backgroundColor-x19g2c9c{background-color:var(--x1xmsgwt)}", 3000);
_inject2("@media (max-width: 700px){.backgroundImage-xa7o7q9.backgroundImage-xa7o7q9{background-image:linear-gradient(to bottom,var(--x1i5pq9l),var(--x1xmsgwt))}}", 3200);
_inject2(".borderWidth-xmkeg23{border-width:1px}", 2000);
_inject2("@media (max-width: 700px){.borderWidth-x1m60m6i.borderWidth-x1m60m6i{border-width:0}}", 2200);
_inject2(".borderStyle-x1y0btm7{border-style:solid}", 2000);
_inject2(".borderColor-x15t7hjr{border-color:rgba(var(--xodl1w7),var(--x2p453m),var(--x140pla3),.3)}", 2000);
_inject2("@media (max-width: 700px){.borderBottomColor-xgepp9j.borderBottomColor-xgepp9j{border-bottom-color:rgba(var(--xodl1w7),var(--x2p453m),var(--x140pla3),.25)}}", 4200);
_inject2(".borderRadius-x12ugs8o{border-radius:var(--xvp50ho)}", 2000);
_inject2("@media (max-width: 700px){.borderRadius-xd22jv.borderRadius-xd22jv{border-radius:0}}", 2200);
_inject2("@media (max-width: 700px){.inset-x1los6se.inset-x1los6se{inset:0 0 auto}}", 1200);
_inject2(".fontWeight-x1xlr1w8{font-weight:700}", 3000);
_inject2(".display-xrvj5dj{display:grid}", 3000);
_inject2(".gridTemplateColumns-xtp8ymz{grid-template-columns:repeat(4,minmax(25%,auto))}", 3000);
_inject2("@media (max-width: 700px){.gridTemplateColumns-xx3cr9d.gridTemplateColumns-xx3cr9d{grid-template-columns:1fr}}", 3200);
_inject2("@media (min-width: 701px) and (max-width: 1120px){.gridTemplateColumns-xtffbmy.gridTemplateColumns-xtffbmy{grid-template-columns:repeat(2,50%)}}", 3200);
_inject2(".width-x15jn8ho{width:var(--xt7qi6)}", 4000);
_inject2(".maxWidth-x193iq5w{max-width:100%}", 4000);
_inject2("@media (max-width: 700px){.maxWidth-xl858mc.maxWidth-xl858mc{max-width:320px}}", 4200);
_inject2("@media (max-width: 700px){.textAlign-x15hltav.textAlign-x15hltav{text-align:center}}", 3200);
