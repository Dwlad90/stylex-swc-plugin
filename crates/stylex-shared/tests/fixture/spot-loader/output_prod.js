import { COMMON_SIZES } from "@/app/components/Test";
import * as stylex from "@stylexjs/stylex";
const styles = {
    root: {
        k1xSpc: "xrvj5dj",
        kprqdN: "x1mt1orb",
        kumcoG: "xernuvs",
        kGNEyG: "x6s0dn4",
        kjj79g: "xl56j7k",
        kOIVth: "xmjcfx9",
        $$css: true
    },
    rect: {
        kWkggS: "x1mdjlir",
        kaIpWk: "x12oqio5",
        kY2c9j: "x11uqc5h",
        kKVMdj: "x13cdbti",
        k44tkh: "x1m9vv7p",
        ko0y90: "xa4qsjk",
        kyAemX: "x4hg4is",
        $$css: true
    },
    rect1: {
        kZKoxP: "x10buj8t",
        $$css: true
    },
    rect2: {
        kKxzle: "x1qdon1m",
        kZKoxP: "x1lnynta",
        $$css: true
    },
    rect3: {
        kKxzle: "x123bg45",
        kZKoxP: "x5yr21d",
        $$css: true
    },
    rect4: {
        kKxzle: "x1olj69",
        kWkggS: "xb4ade6",
        kZKoxP: "x1lnynta",
        $$css: true
    },
    rect5: {
        kKxzle: "x1ryhrx7",
        kZKoxP: "x10buj8t",
        $$css: true
    },
    sizeSmall: {
        kZKoxP: "xettwda",
        kzqmXN: "xs5h3dt",
        $$css: true
    },
    size_small: {
        kZKoxP: "xettwda",
        kzqmXN: "xs5h3dt",
        $$css: true
    },
    size_normal: {
        kZKoxP: "x1sh0tsm",
        kzqmXN: "xekueh",
        $$css: true
    },
    size_large: {
        kZKoxP: "x17frcva",
        kzqmXN: "xdvn7xf",
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
