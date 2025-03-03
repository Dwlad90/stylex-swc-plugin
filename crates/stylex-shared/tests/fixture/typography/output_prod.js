import * as stylex from "@stylexjs/stylex";
import { TYPOGRAPHY_VARIANTS } from '../../../components/data-display/typography/Typography.constants';
import { colors } from '../../../styles/color/colors.stylex';
import { fontSizes, fonts, typographyBase } from '../../../styles/typography/typography.stylex';
const styles = {
    text: {
        margin: "x1ghz6dp",
        marginInline: null,
        marginInlineStart: null,
        marginLeft: null,
        marginInlineEnd: null,
        marginRight: null,
        marginBlock: null,
        marginTop: null,
        marginBottom: null,
        overflowWrap: "xj0a0fe",
        fontFamily: "xxtxygr",
        $$css: true
    },
    textXxxl: {
        lineHeight: "x1xmc3ak",
        fontSize: "x1q754lq",
        $$css: true
    },
    textXxl: {
        lineHeight: "x1xmc3ak",
        fontSize: "x1ikoez",
        $$css: true
    },
    textXl: {
        lineHeight: "x1xmc3ak",
        fontSize: "x1hr3gxt",
        $$css: true
    },
    textLg: {
        lineHeight: "x1xmc3ak",
        fontSize: "x1p5zybj",
        $$css: true
    },
    textMd: {
        lineHeight: "x1xmc3ak",
        fontSize: "x1uv8d7c",
        $$css: true
    },
    textSm: {
        lineHeight: "x1xmc3ak",
        fontSize: "x1dds9ib",
        $$css: true
    },
    body: {
        fontSize: "xcyum62",
        lineHeight: "x10zqc4",
        $$css: true
    },
    bodySm: {
        fontSize: "x1dds9ib",
        lineHeight: "x10zqc4",
        $$css: true
    },
    bodyMd: {
        fontSize: "x1uv8d7c",
        lineHeight: "x10zqc4",
        $$css: true
    },
    truncate: {
        whiteSpace: "xuxw1ft",
        textOverflow: "xlyipyv",
        overflow: "xb3r6kr",
        overflowX: null,
        overflowY: null,
        $$css: true
    },
    bold: {
        fontWeight: "x117nqv4",
        $$css: true
    },
    italic: {
        fontStyle: "x1k4tb9n",
        $$css: true
    },
    textTransform_unset: {
        textTransform: "x1gdvv3m",
        $$css: true
    },
    textTransform_uppercase: {
        textTransform: "xtvhhri",
        $$css: true
    },
    textTransform_lowercase: {
        textTransform: "x1kyqaxf",
        $$css: true
    },
    textTransform_capitalize: {
        textTransform: "xn80e1m",
        $$css: true
    },
    underline: {
        textDecoration: "x1bvjpef",
        textDecorationColor: null,
        textDecorationLine: null,
        textDecorationStyle: null,
        textDecorationThickness: null,
        $$css: true
    },
    colorSuccess: {
        color: "x1le1rw7",
        $$css: true
    },
    colorError: {
        color: "x1dtcyqj",
        $$css: true
    },
    colorInfo: {
        color: "x1ymlcqv",
        $$css: true
    },
    colorWarning: {
        color: "xasdade",
        $$css: true
    },
    colorPrimary: {
        color: "x2d0hfl",
        $$css: true
    },
    color_primary: {
        color: "x2d0hfl",
        $$css: true
    },
    colorSecondary: {
        color: "xxq4n0w",
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
