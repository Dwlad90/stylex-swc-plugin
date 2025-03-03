/**
 * Lead Comment
 */ import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "./globalTokens.stylex";
import React from 'react';
import * as stylex from '@stylexjs/stylex';
import Card from '@/components/Card';
import { globalTokens as $, spacing, text, scales } from './globalTokens.stylex';
import Counter from './Counter';
const HOMEPAGE = 'https://stylexjs.com';
export default function Home() {
    return <main {...{
        className: "display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-x1qughib minHeight-xg6iff7 paddingTop-x1llwu7x paddingBottom-xjfnvzm paddingBottom-x191vjuz",
        "data-style-src": "input.stylex.js:74"
    }}>
      <div {...{
        className: "display-x1jfb8zj justifyContent-xarpa2k alignItems-x1h91t0o maxWidth-xmrzitl width-xh8yej3 zIndex-xhtitgo fontFamily-xh1z4oz",
        "data-style-src": "input.stylex.js:120"
    }}>
        <p {...{
        className: "display-xjg0vao position-x1n2onr6 position-x15f3dyk justifyContent-xo5s888 alignItems-xu8adaz width-x1v68ji2 margin-x1ghz6dp paddingInline-x1fvqwet paddingTop-x1eq7djj paddingTop-xrmelco paddingBottom-x1hsyo9t paddingBottom-x191vjuz backgroundColor-x1lz9bv1 backgroundImage-x1n7lvf9 borderWidth-xmkeg23 borderWidth-x1m60m6i         borderStyle-x1y0btm7 borderColor-x1hydj5d borderBottomColor-xslp3sd borderRadius-x1nklt0o borderRadius-xd22jv         inset-x1los6se",
        "data-style-src": "input.stylex.js:137"
    }}>
          Get started by editing&nbsp;
          <code {...{
        className: "fontWeight-x1xlr1w8 fontFamily-xh1z4oz",
        "data-style-src": "input.stylex.js:177"
    }}>app/page.tsx</code>
        </p>
      </div>
      <div {...{
        className: "flexGrow-x1iyjqo2 display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-xl56j7k gap-x1irrqrq",
        "data-style-src": "input.stylex.js:86"
    }}>
        <h1 {...{
        className: "fontSize-x1u631ky lineHeight-xo5v014 fontFamily-x1o4itb0 fontWeight-xo1l8bm textAlign-x2b8uid display-x78zum5 gap-x643tzn whiteSpace-xuxw1ft flexDirection-x1q0g3np flexDirection-xwlf911",
        "data-style-src": "input.stylex.js:94"
    }}>
          Next.js App Dir<span {...{
        className: "position-x1n2onr6 fontFamily-x6icuqf top-x13vifvy top-x1dgnge0 animationDuration-x1c74tu6 animationIterationCount-xa4qsjk animationTimingFunction-x1esw782",
        "data-style-src": "input.stylex.js:108"
    }}>♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div {...{
        className: "display-xrvj5dj gridTemplateColumns-xtp8ymz gridTemplateColumns-xx3cr9d gridTemplateColumns-xtffbmy width-xcqkx85 maxWidth-x193iq5w maxWidth-xl858mc textAlign-x15hltav",
        "data-style-src": "input.stylex.js:181"
    }}>
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>;
}
_inject2("@keyframes xhab9pt-B{0%{transform:var(--medium-x1m1o2d4);}10%{transform:var(--large-x13z98d3);}20%{transform:var(--medium-x1m1o2d4);}30%{transform:var(--large-x13z98d3);}40%{transform:var(--medium-x1m1o2d4);}90%{transform:var(--small-xrkhmu4);}100%{transform:var(--medium-x1m1o2d4);}}", 1);
_inject2(".display-x78zum5{display:flex}", 3000);
_inject2(".flexDirection-xdt5ytf{flex-direction:column}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-x1qughib{justify-content:space-between}", 3000);
_inject2(".minHeight-xg6iff7{min-height:100vh}", 4000);
_inject2(".paddingTop-x1llwu7x{padding-top:var(--xxl-x1jb9yn9)}", 4000);
_inject2(".paddingBottom-xjfnvzm{padding-bottom:var(--xxl-x1jb9yn9)}", 4000);
_inject2("@media (max-width: 700px){.paddingBottom-x191vjuz.paddingBottom-x191vjuz{padding-bottom:var(--md-xz85zqu)}}", 4200);
_inject2(".flexGrow-x1iyjqo2{flex-grow:1}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".gap-x1irrqrq{gap:var(--xl-x1btcnwp)}", 2000);
_inject2(".fontSize-x1u631ky{font-size:var(--h1-x6bx092)}", 3000);
_inject2(".lineHeight-xo5v014{line-height:1}", 3000);
_inject2(".fontFamily-x1o4itb0{font-family:var(--fontSans-x1v0ot8g)}", 3000);
_inject2(".fontWeight-xo1l8bm{font-weight:400}", 3000);
_inject2(".textAlign-x2b8uid{text-align:center}", 3000);
_inject2(".gap-x643tzn{gap:var(--md-xz85zqu)}", 2000);
_inject2(".whiteSpace-xuxw1ft{white-space:nowrap}", 3000);
_inject2(".flexDirection-x1q0g3np{flex-direction:row}", 3000);
_inject2("@media (max-width: 700px){.flexDirection-xwlf911.flexDirection-xwlf911{flex-direction:column}}", 3200);
_inject2(".position-x1n2onr6{position:relative}", 3000);
_inject2(".fontFamily-x6icuqf{font-family:sans-serif}", 3000);
_inject2(".top-x13vifvy{top:0}", 4000);
_inject2("@media (max-width: 700px){.top-x1dgnge0.top-x1dgnge0{top:var(--xxxs-x1jgrv4s)}}", 4200);
_inject2(".animationDuration-x1c74tu6{animation-duration:2s}", 3000);
_inject2(".animationIterationCount-xa4qsjk{animation-iteration-count:infinite}", 3000);
_inject2(".animationTimingFunction-x1esw782{animation-timing-function:linear}", 3000);
_inject2(".display-x1jfb8zj{display:inherit}", 3000);
_inject2(".justifyContent-xarpa2k{justify-content:inherit}", 3000);
_inject2(".alignItems-x1h91t0o{align-items:inherit}", 3000);
_inject2(".maxWidth-xmrzitl{max-width:var(--maxWidth-x110of7g)}", 4000);
_inject2(".width-xh8yej3{width:100%}", 4000);
_inject2(".zIndex-xhtitgo{z-index:2}", 3000);
_inject2(".fontFamily-xh1z4oz{font-family:var(--fontMono-xgc26q9)}", 3000);
_inject2(".gap-x1kznko5{gap:var(--xxs-xtt9l4u)}", 2000);
_inject2("@media (max-width: 700px){.padding-xywpsvr.padding-xywpsvr{padding:var(--sm-x1k0pbdz)}}", 1200);
_inject2("@media (max-width: 700px){.display-xjg0vao.display-xjg0vao{display:flex}}", 3200);
_inject2("@media (max-width: 700px){.position-x15f3dyk.position-x15f3dyk{position:fixed}}", 3200);
_inject2("@media (max-width: 700px){.justifyContent-xo5s888.justifyContent-xo5s888{justify-content:center}}", 3200);
_inject2("@media (max-width: 700px){.alignItems-xu8adaz.alignItems-xu8adaz{align-items:center}}", 3200);
_inject2("@media (max-width: 700px){.width-x1v68ji2.width-x1v68ji2{width:100%}}", 4200);
_inject2(".margin-x1ghz6dp{margin:0}", 1000);
_inject2(".paddingInline-x1fvqwet{padding-inline:var(--sm-x1k0pbdz)}", 2000);
_inject2(".paddingTop-x1eq7djj{padding-top:var(--sm-x1k0pbdz)}", 4000);
_inject2("@media (max-width: 700px){.paddingTop-xrmelco.paddingTop-xrmelco{padding-top:var(--lg-xu29097)}}", 4200);
_inject2(".paddingBottom-x1hsyo9t{padding-bottom:var(--sm-x1k0pbdz)}", 4000);
_inject2(".backgroundColor-x1lz9bv1{background-color:var(--calloutRGB50-x11lpu6b)}", 3000);
_inject2("@media (max-width: 700px){.backgroundImage-x1n7lvf9.backgroundImage-x1n7lvf9{background-image:linear-gradient(to bottom,var(--bgStartRGB-x1txk845),var(--calloutRGB50-x11lpu6b))}}", 3200);
_inject2(".borderWidth-xmkeg23{border-width:1px}", 2000);
_inject2("@media (max-width: 700px){.borderWidth-x1m60m6i.borderWidth-x1m60m6i{border-width:0}}", 2200);
_inject2(".borderStyle-x1y0btm7{border-style:solid}", 2000);
_inject2(".borderColor-x1hydj5d{border-color:rgba(var(--calloutBorderR-x1tfbujh),var(--calloutBorderG-x1eglwg0),var(--calloutBorderB-xgpbt7a),.3)}", 2000);
_inject2("@media (max-width: 700px){.borderBottomColor-xslp3sd.borderBottomColor-xslp3sd{border-bottom-color:rgba(var(--calloutBorderR-x1tfbujh),var(--calloutBorderG-x1eglwg0),var(--calloutBorderB-xgpbt7a),.25)}}", 4200);
_inject2(".borderRadius-x1nklt0o{border-radius:var(--xs-x1yemeo2)}", 2000);
_inject2("@media (max-width: 700px){.borderRadius-xd22jv.borderRadius-xd22jv{border-radius:0}}", 2200);
_inject2("@media (max-width: 700px){.inset-x1los6se.inset-x1los6se{inset:0 0 auto}}", 1200);
_inject2(".fontWeight-x1xlr1w8{font-weight:700}", 3000);
_inject2(".display-xrvj5dj{display:grid}", 3000);
_inject2(".gridTemplateColumns-xtp8ymz{grid-template-columns:repeat(4,minmax(25%,auto))}", 3000);
_inject2("@media (max-width: 700px){.gridTemplateColumns-xx3cr9d.gridTemplateColumns-xx3cr9d{grid-template-columns:1fr}}", 3200);
_inject2("@media (min-width: 701px) and (max-width: 1120px){.gridTemplateColumns-xtffbmy.gridTemplateColumns-xtffbmy{grid-template-columns:repeat(2,50%)}}", 3200);
_inject2(".width-xcqkx85{width:var(--maxWidth-x110of7g)}", 4000);
_inject2(".maxWidth-x193iq5w{max-width:100%}", 4000);
_inject2("@media (max-width: 700px){.maxWidth-xl858mc.maxWidth-xl858mc{max-width:320px}}", 4200);
_inject2("@media (max-width: 700px){.textAlign-x15hltav.textAlign-x15hltav{text-align:center}}", 3200);
