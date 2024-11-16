import * as stylex from "@stylexjs/stylex";

import { TYPOGRAPHY_VARIANTS } from '../../../components/data-display/typography/Typography.constants';
import { colors } from '../../../styles/color/colors.stylex';
import { fontSizes, fonts, typographyBase } from '../../../styles/typography/typography.stylex';

const styles = stylex.create({
  text: {
    margin: 0,
    overflowWrap: "anywhere",
    fontFamily: fonts.fontPrimary,
  },
  textXxxl: {
    lineHeight: typographyBase.headingLineHeight,
    fontSize: fontSizes.textXxxl,
  },
  textXxl: {
    lineHeight: typographyBase.headingLineHeight,
    fontSize: fontSizes.textXxl,
  },
  textXl: {
    lineHeight: typographyBase.headingLineHeight,
    fontSize: fontSizes.textXl,
  },
  textLg: {
    lineHeight: typographyBase.headingLineHeight,
    fontSize: fontSizes.textLg,
  },
  textMd: {
    lineHeight: typographyBase.headingLineHeight,
    fontSize: fontSizes.textMd,
  },
  textSm: {
    lineHeight: typographyBase.headingLineHeight,
    fontSize: fontSizes.textSm,
  },
  body: {
    fontSize: fontSizes.textBase,
    lineHeight: typographyBase.bodyLineHeight,
  },
  bodySm: {
    fontSize: fontSizes.textSm,
    lineHeight: typographyBase.bodyLineHeight,
  },
  bodyMd: {
    fontSize: fontSizes.textMd,
    lineHeight: typographyBase.bodyLineHeight,
  },
  truncate: {
    whiteSpace: "nowrap",
    textOverflow: "ellipsis",
    overflow: "hidden",
  },
  bold: {
    fontWeight: "bold",
  },
  italic: {
    fontStyle: "italic",
  },
  textTransform_unset: {
    textTransform: "unset",
  },
  textTransform_uppercase: {
    textTransform: "uppercase",
  },
  textTransform_lowercase: {
    textTransform: "lowercase",
  },
  textTransform_capitalize: {
    textTransform: "capitalize",
  },
  underline: {
    textDecoration: "underline",
  },
  colorSuccess: {
    color: colors.success,
  },
  colorError: {
    color: colors.error,
  },
  colorInfo: {
    color: colors.info,
  },
  colorWarning: {
    color: colors.warning,
  },
  colorPrimary: {
    color: colors.primary,
  },
  ['color_primary']: {
    color: colors.primary,
  },
  colorSecondary: {
    color: colors.secondary,
  },
});
const DEFAULT_VARIANT_MAPPING = {
  headingXxxl: "h1",
  headingXxl: "h2",
  headingXl: "h3",
  headingLg: "h4",
  headingMd: "h5",
  headingSm: "h6",
  bodyMd: "p",
  body: "p",
  bodySm: "p",
} as const;
const Typography = ({
  id,
  color,
  style,
  variant = "body",
  isBold = !["body", "bodySm", "bodyMd"].includes(variant),
  isItalic = false,
  isUnderlined = false,
  children,
  variantMapping = DEFAULT_VARIANT_MAPPING,
  isTruncated = !["body", "bodySm", "bodyMd"].includes(variant),
  as,
  textTransform = "unset",
}) => {
  const Component =
    as || variantMapping[variant] || DEFAULT_VARIANT_MAPPING[variant] || "span";

  return (
    <Component
      id={id}
      {...stylex.props(
        styles.text,
        styles['color_primary'],
        color && styles[`color_${color as keyof typeof colors}`],
        color === "success" && styles.colorSuccess,
        color === "error" && styles.colorError,
        color === "info" && styles.colorInfo,
        color === "warning" && styles.colorWarning,
        variant === TYPOGRAPHY_VARIANTS.headingXxxl && styles.textXxxl,
        variant === TYPOGRAPHY_VARIANTS.headingXxl && styles.textXxl,
        variant === TYPOGRAPHY_VARIANTS.headingXl && styles.textXl,
        variant === TYPOGRAPHY_VARIANTS.headingLg && styles.textLg,
        variant === TYPOGRAPHY_VARIANTS.headingMd && styles.textMd,
        variant === TYPOGRAPHY_VARIANTS.headingSm && styles.textSm,
        variant === TYPOGRAPHY_VARIANTS.body && styles.body,
        variant === TYPOGRAPHY_VARIANTS.bodySm && styles.bodySm,
        variant === TYPOGRAPHY_VARIANTS.bodyMd && styles.bodyMd,
        isTruncated && styles.truncate,
        isBold && styles.bold,
        isItalic && styles.italic,
        isUnderlined && styles.underline,
        styles[`textTransform_${textTransform}`],
        style,
      )}
    >
      {children}
    </Component>
  );
};
export default Typography;
