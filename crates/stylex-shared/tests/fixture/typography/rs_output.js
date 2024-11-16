import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "../../../styles/typography/typography.stylex";
import "../../../styles/color/colors.stylex";
import * as stylex from "@stylexjs/stylex";
import { TYPOGRAPHY_VARIANTS } from '../../../components/data-display/typography/Typography.constants';
import { colors } from '../../../styles/color/colors.stylex';
import { fontSizes, fonts, typographyBase } from '../../../styles/typography/typography.stylex';
_inject2(".margin-x1ghz6dp{margin:0}", 1000);
_inject2(".overflowWrap-xj0a0fe{overflow-wrap:anywhere}", 3000);
_inject2(".fontFamily-x1ggk1up{font-family:var(--x1b4t4tc)}", 3000);
_inject2(".lineHeight-x1sfvy2s{line-height:var(--x1ca06q3)}", 3000);
_inject2(".fontSize-x1cefvvz{font-size:var(--x6k3sde)}", 3000);
_inject2(".fontSize-xvbwt3i{font-size:var(--x16j1jf9)}", 3000);
_inject2(".fontSize-x1eku5rm{font-size:var(--x2h82yr)}", 3000);
_inject2(".fontSize-x8yxt08{font-size:var(--x1ea9yuv)}", 3000);
_inject2(".fontSize-xr4ztqd{font-size:var(--x1u3i2xq)}", 3000);
_inject2(".fontSize-xx8g5au{font-size:var(--xzd4ije)}", 3000);
_inject2(".fontSize-x1t2qx1t{font-size:var(--x1ii2xx6)}", 3000);
_inject2(".lineHeight-x59x7jn{line-height:var(--x1id4nr)}", 3000);
_inject2(".whiteSpace-xuxw1ft{white-space:nowrap}", 3000);
_inject2(".textOverflow-xlyipyv{text-overflow:ellipsis}", 3000);
_inject2(".overflow-xb3r6kr{overflow:hidden}", 2000);
_inject2(".fontWeight-x117nqv4{font-weight:bold}", 3000);
_inject2(".fontStyle-x1k4tb9n{font-style:italic}", 3000);
_inject2(".textTransform-x1gdvv3m{text-transform:unset}", 3000);
_inject2(".textTransform-xtvhhri{text-transform:uppercase}", 3000);
_inject2(".textTransform-x1kyqaxf{text-transform:lowercase}", 3000);
_inject2(".textTransform-xn80e1m{text-transform:capitalize}", 3000);
_inject2(".textDecoration-x1bvjpef{text-decoration:underline}", 2000);
_inject2(".color-xsxyzkr{color:var(--x382uzg)}", 3000);
_inject2(".color-xmw8ie6{color:var(--xz5qkih)}", 3000);
_inject2(".color-x1tz0j69{color:var(--xwdpitn)}", 3000);
_inject2(".color-xdjdtrs{color:var(--x1td75ug)}", 3000);
_inject2(".color-x1kncsn5{color:var(--x1p714ct)}", 3000);
_inject2(".color-xadi7i7{color:var(--x1pjdko1)}", 3000);
const styles = {
    text: {
        "Page__styles.text": "Page__styles.text",
        margin: "margin-x1ghz6dp",
        marginInline: null,
        marginInlineStart: null,
        marginLeft: null,
        marginInlineEnd: null,
        marginRight: null,
        marginBlock: null,
        marginTop: null,
        marginBottom: null,
        overflowWrap: "overflowWrap-xj0a0fe",
        fontFamily: "fontFamily-x1ggk1up",
        $$css: true
    },
    textXxxl: {
        "Page__styles.textXxxl": "Page__styles.textXxxl",
        lineHeight: "lineHeight-x1sfvy2s",
        fontSize: "fontSize-x1cefvvz",
        $$css: true
    },
    textXxl: {
        "Page__styles.textXxl": "Page__styles.textXxl",
        lineHeight: "lineHeight-x1sfvy2s",
        fontSize: "fontSize-xvbwt3i",
        $$css: true
    },
    textXl: {
        "Page__styles.textXl": "Page__styles.textXl",
        lineHeight: "lineHeight-x1sfvy2s",
        fontSize: "fontSize-x1eku5rm",
        $$css: true
    },
    textLg: {
        "Page__styles.textLg": "Page__styles.textLg",
        lineHeight: "lineHeight-x1sfvy2s",
        fontSize: "fontSize-x8yxt08",
        $$css: true
    },
    textMd: {
        "Page__styles.textMd": "Page__styles.textMd",
        lineHeight: "lineHeight-x1sfvy2s",
        fontSize: "fontSize-xr4ztqd",
        $$css: true
    },
    textSm: {
        "Page__styles.textSm": "Page__styles.textSm",
        lineHeight: "lineHeight-x1sfvy2s",
        fontSize: "fontSize-xx8g5au",
        $$css: true
    },
    body: {
        "Page__styles.body": "Page__styles.body",
        fontSize: "fontSize-x1t2qx1t",
        lineHeight: "lineHeight-x59x7jn",
        $$css: true
    },
    bodySm: {
        "Page__styles.bodySm": "Page__styles.bodySm",
        fontSize: "fontSize-xx8g5au",
        lineHeight: "lineHeight-x59x7jn",
        $$css: true
    },
    bodyMd: {
        "Page__styles.bodyMd": "Page__styles.bodyMd",
        fontSize: "fontSize-xr4ztqd",
        lineHeight: "lineHeight-x59x7jn",
        $$css: true
    },
    truncate: {
        "Page__styles.truncate": "Page__styles.truncate",
        whiteSpace: "whiteSpace-xuxw1ft",
        textOverflow: "textOverflow-xlyipyv",
        overflow: "overflow-xb3r6kr",
        overflowX: null,
        overflowY: null,
        $$css: true
    },
    bold: {
        "Page__styles.bold": "Page__styles.bold",
        fontWeight: "fontWeight-x117nqv4",
        $$css: true
    },
    italic: {
        "Page__styles.italic": "Page__styles.italic",
        fontStyle: "fontStyle-x1k4tb9n",
        $$css: true
    },
    textTransform_unset: {
        "Page__styles.textTransform_unset": "Page__styles.textTransform_unset",
        textTransform: "textTransform-x1gdvv3m",
        $$css: true
    },
    textTransform_uppercase: {
        "Page__styles.textTransform_uppercase": "Page__styles.textTransform_uppercase",
        textTransform: "textTransform-xtvhhri",
        $$css: true
    },
    textTransform_lowercase: {
        "Page__styles.textTransform_lowercase": "Page__styles.textTransform_lowercase",
        textTransform: "textTransform-x1kyqaxf",
        $$css: true
    },
    textTransform_capitalize: {
        "Page__styles.textTransform_capitalize": "Page__styles.textTransform_capitalize",
        textTransform: "textTransform-xn80e1m",
        $$css: true
    },
    underline: {
        "Page__styles.underline": "Page__styles.underline",
        textDecoration: "textDecoration-x1bvjpef",
        textDecorationColor: null,
        textDecorationLine: null,
        textDecorationStyle: null,
        textDecorationThickness: null,
        $$css: true
    },
    colorSuccess: {
        "Page__styles.colorSuccess": "Page__styles.colorSuccess",
        color: "color-xsxyzkr",
        $$css: true
    },
    colorError: {
        "Page__styles.colorError": "Page__styles.colorError",
        color: "color-xmw8ie6",
        $$css: true
    },
    colorInfo: {
        "Page__styles.colorInfo": "Page__styles.colorInfo",
        color: "color-x1tz0j69",
        $$css: true
    },
    colorWarning: {
        "Page__styles.colorWarning": "Page__styles.colorWarning",
        color: "color-xdjdtrs",
        $$css: true
    },
    colorPrimary: {
        "Page__styles.colorPrimary": "Page__styles.colorPrimary",
        color: "color-x1kncsn5",
        $$css: true
    },
    color_primary: {
        "Page__styles.color_primary": "Page__styles.color_primary",
        color: "color-x1kncsn5",
        $$css: true
    },
    colorSecondary: {
        "Page__styles.colorSecondary": "Page__styles.colorSecondary",
        color: "color-xadi7i7",
        $$css: true
    }
};
const DEFAULT_VARIANT_MAPPING = {
    headingXxxl: "h1",
    headingXxl: "h2",
    headingXl: "h3",
    headingLg: "h4",
    headingMd: "h5",
    headingSm: "h6",
    bodyMd: "p",
    body: "p",
    bodySm: "p"
} as const;
const Typography = ({ id, color, style, variant = "body", isBold = ![
    "body",
    "bodySm",
    "bodyMd"
].includes(variant), isItalic = false, isUnderlined = false, children, variantMapping = DEFAULT_VARIANT_MAPPING, isTruncated = ![
    "body",
    "bodySm",
    "bodyMd"
].includes(variant), as, textTransform = "unset" })=>{
    const Component = as || variantMapping[variant] || DEFAULT_VARIANT_MAPPING[variant] || "span";
    return <Component id={id} {...stylex.props(styles.text, styles['color_primary'], color && styles[`color_${color as keyof typeof colors}`], color === "success" && styles.colorSuccess, color === "error" && styles.colorError, color === "info" && styles.colorInfo, color === "warning" && styles.colorWarning, variant === TYPOGRAPHY_VARIANTS.headingXxxl && styles.textXxxl, variant === TYPOGRAPHY_VARIANTS.headingXxl && styles.textXxl, variant === TYPOGRAPHY_VARIANTS.headingXl && styles.textXl, variant === TYPOGRAPHY_VARIANTS.headingLg && styles.textLg, variant === TYPOGRAPHY_VARIANTS.headingMd && styles.textMd, variant === TYPOGRAPHY_VARIANTS.headingSm && styles.textSm, variant === TYPOGRAPHY_VARIANTS.body && styles.body, variant === TYPOGRAPHY_VARIANTS.bodySm && styles.bodySm, variant === TYPOGRAPHY_VARIANTS.bodyMd && styles.bodyMd, isTruncated && styles.truncate, isBold && styles.bold, isItalic && styles.italic, isUnderlined && styles.underline, styles[`textTransform_${textTransform}`], style)}>
      {children}
    </Component>;
};
export default Typography;
