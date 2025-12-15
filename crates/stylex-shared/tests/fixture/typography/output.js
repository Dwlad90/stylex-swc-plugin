import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import "../../../styles/typography/typography.stylex";
import "../../../styles/color/colors.stylex";
import * as stylex from "@stylexjs/stylex";
import { TYPOGRAPHY_VARIANTS } from '../../../components/data-display/typography/Typography.constants';
import { colors } from '../../../styles/color/colors.stylex';
import { fontSizes, fonts, typographyBase } from '../../../styles/typography/typography.stylex';
_inject2({
    ltr: ".margin-x1ghz6dp{margin:0}",
    priority: 1000
});
_inject2({
    ltr: ".overflowWrap-xj0a0fe{overflow-wrap:anywhere}",
    priority: 3000
});
_inject2({
    ltr: ".fontFamily-xbwy7e6{font-family:var(--fontPrimary-x5f0q53)}",
    priority: 3000
});
_inject2({
    ltr: ".lineHeight-x48q9rv{line-height:var(--headingLineHeight-xp61pzj)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-x193ocya{font-size:var(--textXxxl-x1hz802z)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-xlj8byu{font-size:var(--textXxl-x9zonk2)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-xgc4vk5{font-size:var(--textXl-x1jjjagt)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-x17gblq1{font-size:var(--textLg-x1l0g4sg)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-xm7bc5f{font-size:var(--textMd-x1uw977o)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-x9bx2mk{font-size:var(--textSm-xxjny3m)}",
    priority: 3000
});
_inject2({
    ltr: ".fontSize-xr14wxu{font-size:var(--textBase-x1g1kq9w)}",
    priority: 3000
});
_inject2({
    ltr: ".lineHeight-x1sjzer8{line-height:var(--bodyLineHeight-xahfjtl)}",
    priority: 3000
});
_inject2({
    ltr: ".whiteSpace-xuxw1ft{white-space:nowrap}",
    priority: 3000
});
_inject2({
    ltr: ".textOverflow-xlyipyv{text-overflow:ellipsis}",
    priority: 3000
});
_inject2({
    ltr: ".overflow-xb3r6kr{overflow:hidden}",
    priority: 2000
});
_inject2({
    ltr: ".fontWeight-x117nqv4{font-weight:bold}",
    priority: 3000
});
_inject2({
    ltr: ".fontStyle-x1k4tb9n{font-style:italic}",
    priority: 3000
});
_inject2({
    ltr: ".textTransform-x1gdvv3m{text-transform:unset}",
    priority: 3000
});
_inject2({
    ltr: ".textTransform-xtvhhri{text-transform:uppercase}",
    priority: 3000
});
_inject2({
    ltr: ".textTransform-x1kyqaxf{text-transform:lowercase}",
    priority: 3000
});
_inject2({
    ltr: ".textTransform-xn80e1m{text-transform:capitalize}",
    priority: 3000
});
_inject2({
    ltr: ".textDecoration-x1bvjpef{text-decoration:underline}",
    priority: 2000
});
_inject2({
    ltr: ".color-x2i9qa9{color:var(--success-x1g3za88)}",
    priority: 3000
});
_inject2({
    ltr: ".color-x1wptp0d{color:var(--error-x1x1dixw)}",
    priority: 3000
});
_inject2({
    ltr: ".color-xt2mot5{color:var(--info-xkl3kbl)}",
    priority: 3000
});
_inject2({
    ltr: ".color-xe5xflh{color:var(--warning-x1mhy80l)}",
    priority: 3000
});
_inject2({
    ltr: ".color-xw3ogp8{color:var(--primary-xzstmg8)}",
    priority: 3000
});
_inject2({
    ltr: ".color-x10gd8tk{color:var(--secondary-x1pnddxr)}",
    priority: 3000
});
const styles = {
    text: {
        margin: "margin-x1ghz6dp",
        overflowWrap: "overflowWrap-xj0a0fe",
        fontFamily: "fontFamily-xbwy7e6",
        $$css: "tests/fixture/typography/input.stylex.js:6"
    },
    textXxxl: {
        lineHeight: "lineHeight-x48q9rv",
        fontSize: "fontSize-x193ocya",
        $$css: "tests/fixture/typography/input.stylex.js:11"
    },
    textXxl: {
        lineHeight: "lineHeight-x48q9rv",
        fontSize: "fontSize-xlj8byu",
        $$css: "tests/fixture/typography/input.stylex.js:15"
    },
    textXl: {
        lineHeight: "lineHeight-x48q9rv",
        fontSize: "fontSize-xgc4vk5",
        $$css: "tests/fixture/typography/input.stylex.js:19"
    },
    textLg: {
        lineHeight: "lineHeight-x48q9rv",
        fontSize: "fontSize-x17gblq1",
        $$css: "tests/fixture/typography/input.stylex.js:23"
    },
    textMd: {
        lineHeight: "lineHeight-x48q9rv",
        fontSize: "fontSize-xm7bc5f",
        $$css: "tests/fixture/typography/input.stylex.js:27"
    },
    textSm: {
        lineHeight: "lineHeight-x48q9rv",
        fontSize: "fontSize-x9bx2mk",
        $$css: "tests/fixture/typography/input.stylex.js:31"
    },
    body: {
        fontSize: "fontSize-xr14wxu",
        lineHeight: "lineHeight-x1sjzer8",
        $$css: "tests/fixture/typography/input.stylex.js:35"
    },
    bodySm: {
        fontSize: "fontSize-x9bx2mk",
        lineHeight: "lineHeight-x1sjzer8",
        $$css: "tests/fixture/typography/input.stylex.js:39"
    },
    bodyMd: {
        fontSize: "fontSize-xm7bc5f",
        lineHeight: "lineHeight-x1sjzer8",
        $$css: "tests/fixture/typography/input.stylex.js:43"
    },
    truncate: {
        whiteSpace: "whiteSpace-xuxw1ft",
        textOverflow: "textOverflow-xlyipyv",
        overflow: "overflow-xb3r6kr",
        $$css: "tests/fixture/typography/input.stylex.js:47"
    },
    bold: {
        fontWeight: "fontWeight-x117nqv4",
        $$css: "tests/fixture/typography/input.stylex.js:52"
    },
    italic: {
        fontStyle: "fontStyle-x1k4tb9n",
        $$css: "tests/fixture/typography/input.stylex.js:55"
    },
    textTransform_unset: {
        textTransform: "textTransform-x1gdvv3m",
        $$css: "tests/fixture/typography/input.stylex.js:58"
    },
    textTransform_uppercase: {
        textTransform: "textTransform-xtvhhri",
        $$css: "tests/fixture/typography/input.stylex.js:61"
    },
    textTransform_lowercase: {
        textTransform: "textTransform-x1kyqaxf",
        $$css: "tests/fixture/typography/input.stylex.js:64"
    },
    textTransform_capitalize: {
        textTransform: "textTransform-xn80e1m",
        $$css: "tests/fixture/typography/input.stylex.js:67"
    },
    underline: {
        textDecoration: "textDecoration-x1bvjpef",
        $$css: "tests/fixture/typography/input.stylex.js:70"
    },
    colorSuccess: {
        color: "color-x2i9qa9",
        $$css: "tests/fixture/typography/input.stylex.js:73"
    },
    colorError: {
        color: "color-x1wptp0d",
        $$css: "tests/fixture/typography/input.stylex.js:76"
    },
    colorInfo: {
        color: "color-xt2mot5",
        $$css: "tests/fixture/typography/input.stylex.js:79"
    },
    colorWarning: {
        color: "color-xe5xflh",
        $$css: "tests/fixture/typography/input.stylex.js:82"
    },
    colorPrimary: {
        color: "color-xw3ogp8",
        $$css: "tests/fixture/typography/input.stylex.js:85"
    },
    color_primary: {
        color: "color-xw3ogp8",
        $$css: "tests/fixture/typography/input.stylex.js:99"
    },
    colorSecondary: {
        color: "color-x10gd8tk",
        $$css: "tests/fixture/typography/input.stylex.js:91"
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
};
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
    return <Component id={id} {...stylex.props(styles.text, styles['color_primary'], color && styles[`color_${color}`], color === "success" && styles.colorSuccess, color === "error" && styles.colorError, color === "info" && styles.colorInfo, color === "warning" && styles.colorWarning, variant === TYPOGRAPHY_VARIANTS.headingXxxl && styles.textXxxl, variant === TYPOGRAPHY_VARIANTS.headingXxl && styles.textXxl, variant === TYPOGRAPHY_VARIANTS.headingXl && styles.textXl, variant === TYPOGRAPHY_VARIANTS.headingLg && styles.textLg, variant === TYPOGRAPHY_VARIANTS.headingMd && styles.textMd, variant === TYPOGRAPHY_VARIANTS.headingSm && styles.textSm, variant === TYPOGRAPHY_VARIANTS.body && styles.body, variant === TYPOGRAPHY_VARIANTS.bodySm && styles.bodySm, variant === TYPOGRAPHY_VARIANTS.bodyMd && styles.bodyMd, isTruncated && styles.truncate, isBold && styles.bold, isItalic && styles.italic, isUnderlined && styles.underline, styles[`textTransform_${textTransform}`], style)}>
      {children}
    </Component>;
};
export default Typography;
