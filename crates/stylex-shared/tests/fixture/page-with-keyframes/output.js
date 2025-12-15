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
    return <main className="display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-x1qughib minHeight-xg6iff7 paddingTop-x1llwu7x paddingBottom-xjfnvzm paddingBottom-x191vjuz" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:56">
      <div className="display-x1jfb8zj justifyContent-xarpa2k alignItems-x1h91t0o maxWidth-xmrzitl width-xh8yej3 zIndex-xhtitgo fontFamily-xh1z4oz" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:101">
        <p className="display-xjg0vao position-x1n2onr6 position-x15f3dyk justifyContent-xo5s888 alignItems-xu8adaz width-x1v68ji2 margin-x1ghz6dp paddingInline-x1fvqwet paddingTop-x1eq7djj paddingTop-xrmelco paddingBottom-x1hsyo9t paddingBottom-x191vjuz backgroundColor-x1lz9bv1 backgroundImage-x1n7lvf9 borderWidth-xmkeg23 borderWidth-x1m60m6i borderStyle-x1y0btm7 borderColor-x1hydj5d borderBottomColor-xslp3sd borderRadius-x1nklt0o borderRadius-xd22jv inset-x1los6se" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:119">
          Get started by editing&nbsp;
          <code className="fontWeight-x1xlr1w8 fontFamily-xh1z4oz" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:169">app/page.tsx</code>
        </p>
      </div>
      <div className="flexGrow-x1iyjqo2 display-x78zum5 flexDirection-xdt5ytf alignItems-x6s0dn4 justifyContent-xl56j7k gap-x1irrqrq" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:68">
        <h1 className="fontSize-x1u631ky lineHeight-xo5v014 fontFamily-x1o4itb0 fontWeight-xo1l8bm textAlign-x2b8uid display-x78zum5 gap-x643tzn whiteSpace-xuxw1ft flexDirection-x1q0g3np flexDirection-xwlf911" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:76">
          Next.js App Dir<span className="position-x1n2onr6 fontFamily-x6icuqf top-x13vifvy top-x1dgnge0 animationDuration-x1c74tu6 animationIterationCount-xa4qsjk animationTimingFunction-x1esw782" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:90">♥️</span>️StyleX
        </h1>
        <Counter/>
      </div>

      <div className="display-xrvj5dj gridTemplateColumns-xtp8ymz gridTemplateColumns-xx3cr9d gridTemplateColumns-xtffbmy width-xcqkx85 maxWidth-x193iq5w maxWidth-xl858mc textAlign-x15hltav" data-style-src="tests/fixture/page-with-keyframes/input.stylex.js:173">
        <Card body="Learn how to use StyleX to build UIs" href={`${HOMEPAGE}/docs/learn/`} title="Docs"/>
        <Card body="Browse through the StyleX API reference" href={`${HOMEPAGE}/docs/api/`} title="API"/>
        <Card body="Play with StyleX and look at the compile outputs" href={`${HOMEPAGE}/playground/`} title="Playground"/>
        <Card body="Get started with a NextJS+StyleX project" href="https://github.com/nmn/nextjs-app-dir-stylex" title="Templates"/>
      </div>
    </main>;
}
_inject2({
    ltr: "@keyframes xhab9pt-B{0%{transform:var(--medium-x1m1o2d4);}10%{transform:var(--large-x13z98d3);}20%{transform:var(--medium-x1m1o2d4);}30%{transform:var(--large-x13z98d3);}40%{transform:var(--medium-x1m1o2d4);}90%{transform:var(--small-xrkhmu4);}100%{transform:var(--medium-x1m1o2d4);}}",
    priority: 0
});
_inject2({
    ltr: ".display-x78zum5{display:flex}",
    priority: 3000
});
_inject2({
    ltr: ".flexDirection-xdt5ytf{flex-direction:column}",
    priority: 3000
});
_inject2({
    ltr: ".alignItems-x6s0dn4{align-items:center}",
    priority: 3000
});
_inject2({
    ltr: ".justifyContent-x1qughib{justify-content:space-between}",
    priority: 3000
});
_inject2({
    ltr: ".minHeight-xg6iff7{min-height:100vh}",
    priority: 4000
});
_inject2({
    ltr: ".paddingTop-x1llwu7x{padding-top:var(--xxl-x1jb9yn9)}",
    priority: 4000
});
_inject2({
    ltr: ".paddingBottom-xjfnvzm{padding-bottom:var(--xxl-x1jb9yn9)}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.paddingBottom-x191vjuz.paddingBottom-x191vjuz{padding-bottom:var(--md-xz85zqu)}}",
    priority: 4200
});
_inject2({
    ltr: ".flexGrow-x1iyjqo2{flex-grow:1}",
    priority: 3000
});
_inject2({
    ltr: ".justifyContent-xl56j7k{justify-content:center}",
    priority: 3000
});
_inject2({
    ltr: ".gap-x1irrqrq{gap:var(--xl-x1btcnwp)}",
    priority: 2000
});
_inject2({
    ltr: ".fontSize-x1u631ky{font-size:var(--h1-x6bx092)}",
    priority: 3000
});
_inject2({
    ltr: ".lineHeight-xo5v014{line-height:1}",
    priority: 3000
});
_inject2({
    ltr: ".fontFamily-x1o4itb0{font-family:var(--fontSans-x1v0ot8g)}",
    priority: 3000
});
_inject2({
    ltr: ".fontWeight-xo1l8bm{font-weight:400}",
    priority: 3000
});
_inject2({
    ltr: ".textAlign-x2b8uid{text-align:center}",
    priority: 3000
});
_inject2({
    ltr: ".gap-x643tzn{gap:var(--md-xz85zqu)}",
    priority: 2000
});
_inject2({
    ltr: ".whiteSpace-xuxw1ft{white-space:nowrap}",
    priority: 3000
});
_inject2({
    ltr: ".flexDirection-x1q0g3np{flex-direction:row}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 700px){.flexDirection-xwlf911.flexDirection-xwlf911{flex-direction:column}}",
    priority: 3200
});
_inject2({
    ltr: ".position-x1n2onr6{position:relative}",
    priority: 3000
});
_inject2({
    ltr: ".fontFamily-x6icuqf{font-family:sans-serif}",
    priority: 3000
});
_inject2({
    ltr: ".top-x13vifvy{top:0}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.top-x1dgnge0.top-x1dgnge0{top:var(--xxxs-x1jgrv4s)}}",
    priority: 4200
});
_inject2({
    ltr: ".animationDuration-x1c74tu6{animation-duration:2s}",
    priority: 3000
});
_inject2({
    ltr: ".animationIterationCount-xa4qsjk{animation-iteration-count:infinite}",
    priority: 3000
});
_inject2({
    ltr: ".animationTimingFunction-x1esw782{animation-timing-function:linear}",
    priority: 3000
});
_inject2({
    ltr: ".display-x1jfb8zj{display:inherit}",
    priority: 3000
});
_inject2({
    ltr: ".justifyContent-xarpa2k{justify-content:inherit}",
    priority: 3000
});
_inject2({
    ltr: ".alignItems-x1h91t0o{align-items:inherit}",
    priority: 3000
});
_inject2({
    ltr: ".maxWidth-xmrzitl{max-width:var(--maxWidth-x110of7g)}",
    priority: 4000
});
_inject2({
    ltr: ".width-xh8yej3{width:100%}",
    priority: 4000
});
_inject2({
    ltr: ".zIndex-xhtitgo{z-index:2}",
    priority: 3000
});
_inject2({
    ltr: ".fontFamily-xh1z4oz{font-family:var(--fontMono-xgc26q9)}",
    priority: 3000
});
_inject2({
    ltr: ".gap-x1kznko5{gap:var(--xxs-xtt9l4u)}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.padding-xywpsvr.padding-xywpsvr{padding:var(--sm-x1k0pbdz)}}",
    priority: 1200
});
_inject2({
    ltr: "@media (max-width: 700px){.display-xjg0vao.display-xjg0vao{display:flex}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.position-x15f3dyk.position-x15f3dyk{position:fixed}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.justifyContent-xo5s888.justifyContent-xo5s888{justify-content:center}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.alignItems-xu8adaz.alignItems-xu8adaz{align-items:center}}",
    priority: 3200
});
_inject2({
    ltr: "@media (max-width: 700px){.width-x1v68ji2.width-x1v68ji2{width:100%}}",
    priority: 4200
});
_inject2({
    ltr: ".margin-x1ghz6dp{margin:0}",
    priority: 1000
});
_inject2({
    ltr: ".paddingInline-x1fvqwet{padding-inline:var(--sm-x1k0pbdz)}",
    priority: 2000
});
_inject2({
    ltr: ".paddingTop-x1eq7djj{padding-top:var(--sm-x1k0pbdz)}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.paddingTop-xrmelco.paddingTop-xrmelco{padding-top:var(--lg-xu29097)}}",
    priority: 4200
});
_inject2({
    ltr: ".paddingBottom-x1hsyo9t{padding-bottom:var(--sm-x1k0pbdz)}",
    priority: 4000
});
_inject2({
    ltr: ".backgroundColor-x1lz9bv1{background-color:var(--calloutRGB50-x11lpu6b)}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 700px){.backgroundImage-x1n7lvf9.backgroundImage-x1n7lvf9{background-image:linear-gradient(to bottom,var(--bgStartRGB-x1txk845),var(--calloutRGB50-x11lpu6b))}}",
    priority: 3200
});
_inject2({
    ltr: ".borderWidth-xmkeg23{border-width:1px}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.borderWidth-x1m60m6i.borderWidth-x1m60m6i{border-width:0}}",
    priority: 2200
});
_inject2({
    ltr: ".borderStyle-x1y0btm7{border-style:solid}",
    priority: 2000
});
_inject2({
    ltr: ".borderColor-x1hydj5d{border-color:rgba(var(--calloutBorderR-x1tfbujh),var(--calloutBorderG-x1eglwg0),var(--calloutBorderB-xgpbt7a),.3)}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.borderBottomColor-xslp3sd.borderBottomColor-xslp3sd{border-bottom-color:rgba(var(--calloutBorderR-x1tfbujh),var(--calloutBorderG-x1eglwg0),var(--calloutBorderB-xgpbt7a),.25)}}",
    priority: 4200
});
_inject2({
    ltr: ".borderRadius-x1nklt0o{border-radius:var(--xs-x1yemeo2)}",
    priority: 2000
});
_inject2({
    ltr: "@media (max-width: 700px){.borderRadius-xd22jv.borderRadius-xd22jv{border-radius:0}}",
    priority: 2200
});
_inject2({
    ltr: "@media (max-width: 700px){.inset-x1los6se.inset-x1los6se{inset:0 0 auto}}",
    priority: 1200
});
_inject2({
    ltr: ".fontWeight-x1xlr1w8{font-weight:700}",
    priority: 3000
});
_inject2({
    ltr: ".display-xrvj5dj{display:grid}",
    priority: 3000
});
_inject2({
    ltr: ".gridTemplateColumns-xtp8ymz{grid-template-columns:repeat(4,minmax(25%,auto))}",
    priority: 3000
});
_inject2({
    ltr: "@media (max-width: 700px){.gridTemplateColumns-xx3cr9d.gridTemplateColumns-xx3cr9d{grid-template-columns:1fr}}",
    priority: 3200
});
_inject2({
    ltr: "@media (min-width: 701px) and (max-width: 1120px){.gridTemplateColumns-xtffbmy.gridTemplateColumns-xtffbmy{grid-template-columns:repeat(2,50%)}}",
    priority: 3200
});
_inject2({
    ltr: ".width-xcqkx85{width:var(--maxWidth-x110of7g)}",
    priority: 4000
});
_inject2({
    ltr: ".maxWidth-x193iq5w{max-width:100%}",
    priority: 4000
});
_inject2({
    ltr: "@media (max-width: 700px){.maxWidth-xl858mc.maxWidth-xl858mc{max-width:320px}}",
    priority: 4200
});
_inject2({
    ltr: "@media (max-width: 700px){.textAlign-x15hltav.textAlign-x15hltav{text-align:center}}",
    priority: 3200
});
