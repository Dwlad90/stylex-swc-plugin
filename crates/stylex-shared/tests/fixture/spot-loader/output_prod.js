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

const SpotLoader = ({ isLoading = true, style, size = COMMON_SIZES.normal }) => {
    return isLoading && <>
        <div {...stylex.props(styles[size])}>{size}</div>
        <div {...{
        className: "xrvj5dj x1mt1orb xernuvs x6s0dn4 xl56j7k xmjcfx9 xettwda xs5h3dt"
    }}>styles.sizeSmall</div>
        <div {...stylex.props(styles.root, styles.sizeSmall, style)}>styles.sizeSmall with styles</div>
      </>;
};
export default SpotLoader;
