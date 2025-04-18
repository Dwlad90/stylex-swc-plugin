import * as stylex from "@stylexjs/stylex";
import { TYPOGRAPHY_VARIANTS } from '../../../components/data-display/typography/Typography.constants';
import { colors } from '../../../styles/color/colors.stylex';
import { fontSizes, fonts, typographyBase } from '../../../styles/typography/typography.stylex';
const styles = {
    text: {
        kogj98: "x1ghz6dp",
        kUOVxO: null,
        keTefX: null,
        koQZXg: null,
        k71WvV: null,
        km5ZXQ: null,
        kqGvvJ: null,
        keoZOQ: null,
        k1K539: null,
        kHjlTd: "xj0a0fe",
        kMv6JI: "xxtxygr",
        $$css: true
    },
    textXxxl: {
        kLWn49: "x1xmc3ak",
        kGuDYH: "x1q754lq",
        $$css: true
    },
    textXxl: {
        kLWn49: "x1xmc3ak",
        kGuDYH: "x1ikoez",
        $$css: true
    },
    textXl: {
        kLWn49: "x1xmc3ak",
        kGuDYH: "x1hr3gxt",
        $$css: true
    },
    textLg: {
        kLWn49: "x1xmc3ak",
        kGuDYH: "x1p5zybj",
        $$css: true
    },
    textMd: {
        kLWn49: "x1xmc3ak",
        kGuDYH: "x1uv8d7c",
        $$css: true
    },
    textSm: {
        kLWn49: "x1xmc3ak",
        kGuDYH: "x1dds9ib",
        $$css: true
    },
    body: {
        kGuDYH: "xcyum62",
        kLWn49: "x10zqc4",
        $$css: true
    },
    bodySm: {
        kGuDYH: "x1dds9ib",
        kLWn49: "x10zqc4",
        $$css: true
    },
    bodyMd: {
        kGuDYH: "x1uv8d7c",
        kLWn49: "x10zqc4",
        $$css: true
    },
    truncate: {
        khDVqt: "xuxw1ft",
        kg5iWk: "xlyipyv",
        kVQacm: "xb3r6kr",
        kXHlph: null,
        kORKVm: null,
        $$css: true
    },
    bold: {
        k63SB2: "x117nqv4",
        $$css: true
    },
    italic: {
        kKX8nH: "x1k4tb9n",
        $$css: true
    },
    textTransform_unset: {
        kP9fke: "x1gdvv3m",
        $$css: true
    },
    textTransform_uppercase: {
        kP9fke: "xtvhhri",
        $$css: true
    },
    textTransform_lowercase: {
        kP9fke: "x1kyqaxf",
        $$css: true
    },
    textTransform_capitalize: {
        kP9fke: "xn80e1m",
        $$css: true
    },
    underline: {
        kybGjl: "x1bvjpef",
        k1TLXF: null,
        kMnn75: null,
        kmVMDM: null,
        kNySMw: null,
        $$css: true
    },
    colorSuccess: {
        kMwMTN: "x1le1rw7",
        $$css: true
    },
    colorError: {
        kMwMTN: "x1dtcyqj",
        $$css: true
    },
    colorInfo: {
        kMwMTN: "x1ymlcqv",
        $$css: true
    },
    colorWarning: {
        kMwMTN: "xasdade",
        $$css: true
    },
    colorPrimary: {
        kMwMTN: "x2d0hfl",
        $$css: true
    },
    color_primary: {
        kMwMTN: "x2d0hfl",
        $$css: true
    },
    colorSecondary: {
        kMwMTN: "xxq4n0w",
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
