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
_inject2(".fontFamily-xkb8tpn{font-family:var(--fontPrimary-x1b4t4tc)}", 3000);
_inject2(".lineHeight-x1idg4k0{line-height:var(--headingLineHeight-x1ca06q3)}", 3000);
_inject2(".fontSize-x1gidmzx{font-size:var(--textXxxl-x6k3sde)}", 3000);
_inject2(".fontSize-xrt1taa{font-size:var(--textXxl-x16j1jf9)}", 3000);
_inject2(".fontSize-xcym6mb{font-size:var(--textXl-x2h82yr)}", 3000);
_inject2(".fontSize-x1srbcfm{font-size:var(--textLg-x1ea9yuv)}", 3000);
_inject2(".fontSize-x15jeal9{font-size:var(--textMd-x1u3i2xq)}", 3000);
_inject2(".fontSize-xjjk0k8{font-size:var(--textSm-xzd4ije)}", 3000);
_inject2(".fontSize-x140zqe6{font-size:var(--textBase-x1ii2xx6)}", 3000);
_inject2(".lineHeight-xeic3y9{line-height:var(--bodyLineHeight-x1id4nr)}", 3000);
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
_inject2(".color-x1ljzc9k{color:var(--success-x382uzg)}", 3000);
_inject2(".color-x1v53zt7{color:var(--error-xz5qkih)}", 3000);
_inject2(".color-xluix4b{color:var(--info-xwdpitn)}", 3000);
_inject2(".color-xvm0r0y{color:var(--warning-x1td75ug)}", 3000);
_inject2(".color-x1usuvry{color:var(--primary-x1p714ct)}", 3000);
_inject2(".color-x11czq3c{color:var(--secondary-x1pjdko1)}", 3000);
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
        fontFamily: "fontFamily-xkb8tpn",
        $$css: true
    },
    textXxxl: {
        "Page__styles.textXxxl": "Page__styles.textXxxl",
        lineHeight: "lineHeight-x1idg4k0",
        fontSize: "fontSize-x1gidmzx",
        $$css: true
    },
    textXxl: {
        "Page__styles.textXxl": "Page__styles.textXxl",
        lineHeight: "lineHeight-x1idg4k0",
        fontSize: "fontSize-xrt1taa",
        $$css: true
    },
    textXl: {
        "Page__styles.textXl": "Page__styles.textXl",
        lineHeight: "lineHeight-x1idg4k0",
        fontSize: "fontSize-xcym6mb",
        $$css: true
    },
    textLg: {
        "Page__styles.textLg": "Page__styles.textLg",
        lineHeight: "lineHeight-x1idg4k0",
        fontSize: "fontSize-x1srbcfm",
        $$css: true
    },
    textMd: {
        "Page__styles.textMd": "Page__styles.textMd",
        lineHeight: "lineHeight-x1idg4k0",
        fontSize: "fontSize-x15jeal9",
        $$css: true
    },
    textSm: {
        "Page__styles.textSm": "Page__styles.textSm",
        lineHeight: "lineHeight-x1idg4k0",
        fontSize: "fontSize-xjjk0k8",
        $$css: true
    },
    body: {
        "Page__styles.body": "Page__styles.body",
        fontSize: "fontSize-x140zqe6",
        lineHeight: "lineHeight-xeic3y9",
        $$css: true
    },
    bodySm: {
        "Page__styles.bodySm": "Page__styles.bodySm",
        fontSize: "fontSize-xjjk0k8",
        lineHeight: "lineHeight-xeic3y9",
        $$css: true
    },
    bodyMd: {
        "Page__styles.bodyMd": "Page__styles.bodyMd",
        fontSize: "fontSize-x15jeal9",
        lineHeight: "lineHeight-xeic3y9",
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
        color: "color-x1ljzc9k",
        $$css: true
    },
    colorError: {
        "Page__styles.colorError": "Page__styles.colorError",
        color: "color-x1v53zt7",
        $$css: true
    },
    colorInfo: {
        "Page__styles.colorInfo": "Page__styles.colorInfo",
        color: "color-xluix4b",
        $$css: true
    },
    colorWarning: {
        "Page__styles.colorWarning": "Page__styles.colorWarning",
        color: "color-xvm0r0y",
        $$css: true
    },
    colorPrimary: {
        "Page__styles.colorPrimary": "Page__styles.colorPrimary",
        color: "color-x1usuvry",
        $$css: true
    },
    color_primary: {
        "Page__styles.color_primary": "Page__styles.color_primary",
        color: "color-x1usuvry",
        $$css: true
    },
    colorSecondary: {
        "Page__styles.colorSecondary": "Page__styles.colorSecondary",
        color: "color-x11czq3c",
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
