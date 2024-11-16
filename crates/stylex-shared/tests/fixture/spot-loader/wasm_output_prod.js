//__stylex_metadata_start__[{"class_name":"xn4wiy6-B","style":{"rtl":null,"ltr":"@keyframes xn4wiy6-B{from{transform:perspective(120px) rotatex(0deg) rotatey(0deg);}to{transform:perspective(120px) rotatex(-180.1deg) rotatey(0deg);}}"},"priority":1},{"class_name":"xrvj5dj","style":{"rtl":null,"ltr":".xrvj5dj{display:grid}"},"priority":3000},{"class_name":"x1mt1orb","style":{"rtl":null,"ltr":".x1mt1orb{grid-auto-flow:column}"},"priority":3000},{"class_name":"xernuvs","style":{"rtl":null,"ltr":".xernuvs{grid-template-columns:repeat(5,12%)}"},"priority":3000},{"class_name":"x6s0dn4","style":{"rtl":null,"ltr":".x6s0dn4{align-items:center}"},"priority":3000},{"class_name":"xl56j7k","style":{"rtl":null,"ltr":".xl56j7k{justify-content:center}"},"priority":3000},{"class_name":"xmjcfx9","style":{"rtl":null,"ltr":".xmjcfx9{gap:8%}"},"priority":2000},{"class_name":"x1mdjlir","style":{"rtl":null,"ltr":".x1mdjlir{background-color:#2c3e50}"},"priority":3000},{"class_name":"x12oqio5","style":{"rtl":null,"ltr":".x12oqio5{border-radius:4px}"},"priority":2000},{"class_name":"x11uqc5h","style":{"rtl":null,"ltr":".x11uqc5h{z-index:100}"},"priority":3000},{"class_name":"x13cdbti","style":{"rtl":null,"ltr":".x13cdbti{animation-name:xn4wiy6-B}"},"priority":3000},{"class_name":"x1m9vv7p","style":{"rtl":null,"ltr":".x1m9vv7p{animation-duration:1.2s}"},"priority":3000},{"class_name":"xa4qsjk","style":{"rtl":null,"ltr":".xa4qsjk{animation-iteration-count:infinite}"},"priority":3000},{"class_name":"x4hg4is","style":{"rtl":null,"ltr":".x4hg4is{animation-timing-function:ease-in-out}"},"priority":3000},{"class_name":"x10buj8t","style":{"rtl":null,"ltr":".x10buj8t{height:24%}"},"priority":4000},{"class_name":"x1qdon1m","style":{"rtl":null,"ltr":".x1qdon1m{animation-delay:-1.1s}"},"priority":3000},{"class_name":"x1lnynta","style":{"rtl":null,"ltr":".x1lnynta{height:62%}"},"priority":4000},{"class_name":"x123bg45","style":{"rtl":null,"ltr":".x123bg45{animation-delay:-1s}"},"priority":3000},{"class_name":"x5yr21d","style":{"rtl":null,"ltr":".x5yr21d{height:100%}"},"priority":4000},{"class_name":"x1olj69","style":{"rtl":null,"ltr":".x1olj69{animation-delay:-.9s}"},"priority":3000},{"class_name":"xb4ade6","style":{"rtl":null,"ltr":".xb4ade6{background-color:#ff4500}"},"priority":3000},{"class_name":"x1ryhrx7","style":{"rtl":null,"ltr":".x1ryhrx7{animation-delay:-.8s}"},"priority":3000},{"class_name":"xettwda","style":{"rtl":null,"ltr":".xettwda{height:2rem}"},"priority":4000},{"class_name":"xs5h3dt","style":{"rtl":null,"ltr":".xs5h3dt{width:2rem}"},"priority":4000},{"class_name":"x1sh0tsm","style":{"rtl":null,"ltr":".x1sh0tsm{height:4rem}"},"priority":4000},{"class_name":"xekueh","style":{"rtl":null,"ltr":".xekueh{width:4rem}"},"priority":4000},{"class_name":"x17frcva","style":{"rtl":null,"ltr":".x17frcva{height:6rem}"},"priority":4000},{"class_name":"xdvn7xf","style":{"rtl":null,"ltr":".xdvn7xf{width:6rem}"},"priority":4000}]__stylex_metadata_end__
import { COMMON_SIZES } from "@/app/components/Test";
import * as stylex from "@stylexjs/stylex";
const styles = {
    root: {
        display: "xrvj5dj",
        gridAutoFlow: "x1mt1orb",
        gridTemplateColumns: "xernuvs",
        alignItems: "x6s0dn4",
        justifyContent: "xl56j7k",
        gap: "xmjcfx9",
        rowGap: null,
        columnGap: null,
        $$css: true
    },
    rect: {
        backgroundColor: "x1mdjlir",
        borderRadius: "x12oqio5",
        borderStartStartRadius: null,
        borderStartEndRadius: null,
        borderEndStartRadius: null,
        borderEndEndRadius: null,
        borderTopLeftRadius: null,
        borderTopRightRadius: null,
        borderBottomLeftRadius: null,
        borderBottomRightRadius: null,
        zIndex: "x11uqc5h",
        animationName: "x13cdbti",
        animationDuration: "x1m9vv7p",
        animationIterationCount: "xa4qsjk",
        animationTimingFunction: "x4hg4is",
        $$css: true
    },
    rect1: {
        height: "x10buj8t",
        $$css: true
    },
    rect2: {
        animationDelay: "x1qdon1m",
        height: "x1lnynta",
        $$css: true
    },
    rect3: {
        animationDelay: "x123bg45",
        height: "x5yr21d",
        $$css: true
    },
    rect4: {
        animationDelay: "x1olj69",
        backgroundColor: "xb4ade6",
        height: "x1lnynta",
        $$css: true
    },
    rect5: {
        animationDelay: "x1ryhrx7",
        height: "x10buj8t",
        $$css: true
    },
    sizeSmall: {
        height: "xettwda",
        width: "xs5h3dt",
        $$css: true
    },
    size_small: {
        height: "xettwda",
        width: "xs5h3dt",
        $$css: true
    },
    size_normal: {
        height: "x1sh0tsm",
        width: "xekueh",
        $$css: true
    },
    size_large: {
        height: "x17frcva",
        width: "xdvn7xf",
        $$css: true
    }
};
const SpotLoader = ({ isLoading = true, style, size = COMMON_SIZES.normal })=>{
    return isLoading && <>
        <div {...stylex.props(styles[size])}>{size}</div>
        <div {...{
        className: "xrvj5dj x1mt1orb xernuvs x6s0dn4 xl56j7k xmjcfx9 xettwda xs5h3dt"
    }}>styles.sizeSmall</div>
        <div {...stylex.props(styles.root, styles.sizeSmall, style)}>styles.sizeSmall with styles</div>
      </>;
};
export default SpotLoader;
