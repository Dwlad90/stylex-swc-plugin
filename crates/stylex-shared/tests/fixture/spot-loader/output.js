import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import { COMMON_SIZES } from "@/app/components/Test";
import * as stylex from "@stylexjs/stylex";
_inject2("@keyframes xn4wiy6-B{from{transform:perspective(120px) rotatex(0deg) rotatey(0deg);}to{transform:perspective(120px) rotatex(-180.1deg) rotatey(0deg);}}", 0);
_inject2(".display-xrvj5dj{display:grid}", 3000);
_inject2(".gridAutoFlow-x1mt1orb{grid-auto-flow:column}", 3000);
_inject2(".gridTemplateColumns-xernuvs{grid-template-columns:repeat(5,12%)}", 3000);
_inject2(".alignItems-x6s0dn4{align-items:center}", 3000);
_inject2(".justifyContent-xl56j7k{justify-content:center}", 3000);
_inject2(".gap-xmjcfx9{gap:8%}", 2000);
_inject2(".backgroundColor-x1mdjlir{background-color:#2c3e50}", 3000);
_inject2(".borderRadius-x12oqio5{border-radius:4px}", 2000);
_inject2(".zIndex-x11uqc5h{z-index:100}", 3000);
_inject2(".animationName-x13cdbti{animation-name:xn4wiy6-B}", 3000);
_inject2(".animationDuration-x1m9vv7p{animation-duration:1.2s}", 3000);
_inject2(".animationIterationCount-xa4qsjk{animation-iteration-count:infinite}", 3000);
_inject2(".animationTimingFunction-x4hg4is{animation-timing-function:ease-in-out}", 3000);
_inject2(".height-x10buj8t{height:24%}", 4000);
_inject2(".animationDelay-x1qdon1m{animation-delay:-1.1s}", 3000);
_inject2(".height-x1lnynta{height:62%}", 4000);
_inject2(".animationDelay-x123bg45{animation-delay:-1s}", 3000);
_inject2(".height-x5yr21d{height:100%}", 4000);
_inject2(".animationDelay-x1olj69{animation-delay:-.9s}", 3000);
_inject2(".backgroundColor-xb4ade6{background-color:#ff4500}", 3000);
_inject2(".animationDelay-x1ryhrx7{animation-delay:-.8s}", 3000);
_inject2(".height-xettwda{height:2rem}", 4000);
_inject2(".width-xs5h3dt{width:2rem}", 4000);
_inject2(".height-x1sh0tsm{height:4rem}", 4000);
_inject2(".width-xekueh{width:4rem}", 4000);
_inject2(".height-x17frcva{height:6rem}", 4000);
_inject2(".width-xdvn7xf{width:6rem}", 4000);
const styles = {
    root: {
        display: "display-xrvj5dj",
        gridAutoFlow: "gridAutoFlow-x1mt1orb",
        gridTemplateColumns: "gridTemplateColumns-xernuvs",
        alignItems: "alignItems-x6s0dn4",
        justifyContent: "justifyContent-xl56j7k",
        gap: "gap-xmjcfx9",
        $$css: "tests/fixture/spot-loader/input.stylex.js:12"
    },
    rect: {
        backgroundColor: "backgroundColor-x1mdjlir",
        borderRadius: "borderRadius-x12oqio5",
        zIndex: "zIndex-x11uqc5h",
        animationName: "animationName-x13cdbti",
        animationDuration: "animationDuration-x1m9vv7p",
        animationIterationCount: "animationIterationCount-xa4qsjk",
        animationTimingFunction: "animationTimingFunction-x4hg4is",
        $$css: "tests/fixture/spot-loader/input.stylex.js:20"
    },
    rect1: {
        height: "height-x10buj8t",
        $$css: "tests/fixture/spot-loader/input.stylex.js:29"
    },
    rect2: {
        animationDelay: "animationDelay-x1qdon1m",
        height: "height-x1lnynta",
        $$css: "tests/fixture/spot-loader/input.stylex.js:32"
    },
    rect3: {
        animationDelay: "animationDelay-x123bg45",
        height: "height-x5yr21d",
        $$css: "tests/fixture/spot-loader/input.stylex.js:36"
    },
    rect4: {
        animationDelay: "animationDelay-x1olj69",
        backgroundColor: "backgroundColor-xb4ade6",
        height: "height-x1lnynta",
        $$css: "tests/fixture/spot-loader/input.stylex.js:40"
    },
    rect5: {
        animationDelay: "animationDelay-x1ryhrx7",
        height: "height-x10buj8t",
        $$css: "tests/fixture/spot-loader/input.stylex.js:45"
    },
    sizeSmall: {
        height: "height-xettwda",
        width: "width-xs5h3dt",
        $$css: "tests/fixture/spot-loader/input.stylex.js:49"
    },
    size_small: {
        height: "height-xettwda",
        width: "width-xs5h3dt",
        $$css: "tests/fixture/spot-loader/input.stylex.js:61"
    },
    size_normal: {
        height: "height-x1sh0tsm",
        width: "width-xekueh",
        $$css: "tests/fixture/spot-loader/input.stylex.js:57"
    },
    size_large: {
        height: "height-x17frcva",
        width: "width-xdvn7xf",
        $$css: "tests/fixture/spot-loader/input.stylex.js:61"
    }
};
const SpotLoader = ({ isLoading = true, style, size = COMMON_SIZES.normal })=>{
    return isLoading && <>
        <div {...stylex.props(styles[size])}>{size}</div>
        <div className="display-xrvj5dj gridAutoFlow-x1mt1orb gridTemplateColumns-xernuvs alignItems-x6s0dn4 justifyContent-xl56j7k gap-xmjcfx9 height-xettwda width-xs5h3dt" data-style-src="tests/fixture/spot-loader/input.stylex.js:12; tests/fixture/spot-loader/input.stylex.js:49">styles.sizeSmall</div>
        <div {...stylex.props(styles.root, styles.sizeSmall, style)}>styles.sizeSmall with styles</div>
      </>;
};
export default SpotLoader;
