'use strict';

import * as stylex from '@stylexjs/stylex';

export const lotsOfStylesDynamic = [
  stylex.create({
    dynamicHeight: (height) => ({
      height,
    }),
    dynamicPadding: (paddingTop, paddingBottom) => ({
      paddingTop,
      paddingBottom,
    }),
    dynamicTextColor: (textColor) => ({
      color: textColor,
    }),
  }),
  stylex.create({
    dynamicHeightWithStatic: (height) => ({
      height,
      backgroundColor: 'var(--background-color)',
    }),
    dynamicPaddingWithStatic: (paddingTop, paddingBottom) => ({
      paddingTop,
      paddingBottom,
      margin: '8px',
    }),
    dynamicTextColorWithStatic: (textColor) => ({
      color: textColor,
      fontSize: '16px',
    }),
  }),
  stylex.create({
    dynamicHeight: (height) => ({ height }),
    dynamicPadding: (paddingTop, paddingBottom) => ({
      paddingTop,
      paddingBottom,
    }),
    dynamicTextColor: (textColor) => ({ color: textColor }),
    dynamicFontSize: (fontSize) => ({ fontSize }),
    dynamicFontWeight: (fontWeight) => ({ fontWeight }),
    dynamicLineHeight: (lineHeight) => ({ lineHeight }),
    dynamicLetterSpacing: (letterSpacing) => ({ letterSpacing }),
    dynamicTextTransform: (textTransform) => ({ textTransform }),
    dynamicTextDecoration: (textDecoration) => ({ textDecoration }),
  }),
  stylex.create({
    dynamicFontSizeWithStatic: (fontSize) => ({
      fontSize,
      fontFamily: 'Arial, sans-serif',
    }),
    dynamicFontWeightWithStatic: (fontWeight) => ({
      fontWeight,
      fontStyle: 'normal',
    }),
    dynamicLineHeightWithStatic: (lineHeight) => ({
      lineHeight,
      textAlign: 'left',
    }),
    dynamicLetterSpacingWithStatic: (letterSpacing) => ({
      letterSpacing,
      wordWrap: 'break-word',
    }),
    dynamicTextTransformWithStatic: (textTransform) => ({
      textTransform,
      textShadow: 'none',
    }),
    dynamicTextDecorationWithStatic: (textDecoration) => ({
      textDecoration,
      boxSizing: 'border-box',
    }),
  }),
  stylex.create({
    dynamicWidth: (width) => ({ width }),
    dynamicMinWidth: (minWidth) => ({ minWidth }),
    dynamicMaxWidth: (maxWidth) => ({ maxWidth }),
    dynamicHeight2: (height) => ({ height }),
    dynamicMinHeight: (minHeight) => ({ minHeight }),
    dynamicMaxHeight: (maxHeight) => ({ maxHeight }),
    dynamicBorderRadius: (borderRadius) => ({ borderRadius }),
    dynamicBoxShadow: (boxShadow) => ({ boxShadow }),
    dynamicBackgroundImage: (backgroundImage) => ({ backgroundImage }),
    dynamicBackgroundSize: (backgroundSize) => ({ backgroundSize }),
    dynamicBackgroundPosition: (backgroundPosition) => ({ backgroundPosition }),
    dynamicBackgroundRepeat: (backgroundRepeat) => ({ backgroundRepeat }),
  }),
  stylex.create({
    dynamicOpacity: (opacity) => ({ opacity }),
    dynamicVisibility: (visibility) => ({ visibility }),
    dynamicDisplay: (display) => ({ display }),
    dynamicPosition: (position) => ({ position }),
    dynamicTop: (top) => ({ top }),
    dynamicRight: (right) => ({ right }),
    dynamicBottom: (bottom) => ({ bottom }),
    dynamicLeft: (left) => ({ left }),
    dynamicZIndex: (zIndex) => ({ zIndex }),
    dynamicOverflow: (overflow) => ({ overflow }),
    dynamicOverflowX: (overflowX) => ({ overflowX }),
    dynamicOverflowY: (overflowY) => ({ overflowY }),
  }),
  stylex.create({
    dynamicCursor: (cursor) => ({ cursor }),
    dynamicOutline: (outline) => ({ outline }),
    dynamicOutlineWidth: (outlineWidth) => ({ outlineWidth }),
    dynamicOutlineStyle: (outlineStyle) => ({ outlineStyle }),
    dynamicOutlineColor: (outlineColor) => ({ outlineColor }),
    dynamicListStyle: (listStyle) => ({ listStyle }),
    dynamicListStyleType: (listStyleType) => ({ listStyleType }),
    dynamicListStylePosition: (listStylePosition) => ({ listStylePosition }),
    dynamicListStyleImage: (listStyleImage) => ({ listStyleImage }),
  }),
  stylex.create({
    dynamicInput: (caretColor) => ({
      'caret-color': caretColor,
    }),
    dynamicDisplayInherit: (display) => ({
      display,
    }),
    dynamicInherit: (
      alignContent,
      alignItems,
      flexDirection,
      flexGrow,
      flexShrink,
      height,
      justifyContent,
      maxHeight,
      maxWidth,
      minHeight,
      minWidth,
      position,
      width,
    ) => ({
      alignContent,
      alignItems,
      flexDirection,
      flexGrow,
      flexShrink,
      height,
      justifyContent,
      maxHeight,
      maxWidth,
      minHeight,
      minWidth,
      position,
      width,
    }),
  }),
  stylex.create({
    dynamicRailContent: (fontSize, margin) => ({
      fontSize,
      margin,
    }),
    dynamicRailItem: (marginBottom) => ({
      marginBottom,
    }),
    dynamicRoot: (flexGrow, listStyleType, margin) => ({
      flexGrow,
      listStyleType,
      margin,
    }),
    dynamicWidgetSet: (display, marginTop) => ({
      display,
      marginTop,
    }),
  }),
  stylex.create({
    dynamicContainer: (marginInlineEnd) => ({
      marginInlineEnd,
    }),
    dynamicKeyInfo: (
      backgroundColor,
      borderWidth,
      borderStyle,
      borderColor,
      borderRadius,
      marginInlineEnd,
      padding,
    ) => ({
      backgroundColor,
      borderWidth,
      borderStyle,
      borderColor,
      borderRadius,
      marginInlineEnd,
      padding,
    }),
    dynamicKeyInfoItem: (marginTop) => ({
      marginTop,
    }),
  }),
  stylex.create({
    dynamicBlueBackground: (backgroundColor, color, padding) => ({
      backgroundColor,
      color,
      padding,
    }),
    dynamicRedBackground: (backgroundColor, color, padding) => ({
      backgroundColor,
      color,
      padding,
    }),
    dynamicWhiteBackground: (backgroundColor, color, padding) => ({
      backgroundColor,
      color,
      padding,
    }),
    dynamicContainer: (borderWidth, borderStyle, marginInlineEnd) => ({
      borderWidth,
      borderStyle,
      marginInlineEnd,
    }),
    dynamicInputWrapper: (marginTop) => ({
      marginTop,
    }),
  }),
  stylex.create({
    dynamicGreenBackground: (backgroundColor, color, padding) => ({
      backgroundColor,
      color,
      padding,
    }),
    dynamicSection: (marginBottom) => ({
      marginBottom,
    }),
  }),
  stylex.create({
    dynamicKeyInfo: (
      borderWidth,
      borderStyle,
      borderColor,
      borderRadius,
      display,
      lineHeight,
      margin,
      minWidth,
      padding,
      paddingInlineEnd,
      paddingInlineStart,
      textAlign,
    ) => ({
      borderWidth,
      borderStyle,
      borderColor,
      borderRadius,
      display,
      lineHeight,
      margin,
      minWidth,
      padding,
      paddingInlineEnd,
      paddingInlineStart,
      textAlign,
    }),
  }),
  stylex.create({
    dynamicList: (paddingBottom, paddingTop) => ({
      paddingBottom,
      paddingTop,
    }),
    dynamicListItem: (paddingTop) => ({
      paddingTop,
    }),
    dynamicPlus: (marginInline) => ({
      marginInline,
    }),
  }),
  stylex.create({
    dynamicWrapperFocusable: (outline) => ({
      outline,
    }),
  }),
  stylex.create({
    dynamicHeader: (display, flexGrow, flexShrink, flexBasis) => ({
      display,
      flexGrow,
      flexShrink,
      flexBasis,
    }),
  }),
  stylex.create({
    dynamicDialog: (backgroundColor, height, width) => ({
      backgroundColor,
      height,
      width,
    }),
  }),
  stylex.create({
    dynamicRoot: (backgroundColor, height, width, overflowY) => ({
      backgroundColor,
      height,
      width,
      overflowY,
    }),
  }),
  stylex.create({
    dynamicContainer: (
      backgroundColor,
      display,
      flexDirection,
      height,
      width,
    ) => ({
      backgroundColor,
      display,
      flexDirection,
      height,
      width,
    }),
    dynamicColumnLayout: (display, flexGrow, flexShrink, minHeight) => ({
      display,
      flexGrow,
      flexShrink,
      minHeight,
    }),
    dynamicBodyContainer: (width, flexGrow, display, justifyContent) => ({
      width,
      flexGrow,
      display,
      justifyContent,
    }),
    dynamicCardWrapper: (
      display,
      flexDirection,
      flexShrink,
      flexGrow,
      minWidth,
      margin,
    ) => ({
      display,
      flexDirection,
      flexShrink,
      flexGrow,
      minWidth,
      margin,
    }),
    dynamicRoot: (
      backgroundColor,
      borderRadius,
      boxShadow,
      display,
      marginBlock,
      minHeight,
      maxWidth,
      width,
      flexGrow,
      alignSelf,
      flexDirection,
      overflow,
    ) => ({
      backgroundColor,
      borderRadius,
      boxShadow,
      display,
      marginBlock,
      minHeight,
      maxWidth,
      width,
      flexGrow,
      alignSelf,
      flexDirection,
      overflow,
    }),
  }),
  stylex.create({
    dynamicButton: (width, height, marginInlineEnd, borderRadius) => ({
      width,
      height,
      marginInlineEnd,
      borderRadius,
    }),
    dynamicBody: (width, height, borderRadius, marginBlock) => ({
      width,
      height,
      borderRadius,
      marginBlock,
    }),
    dynamicMeta: (width, height, borderRadius) => ({
      width,
      height,
      borderRadius,
    }),
  }),
  stylex.create({
    dynamicHeader: (display, flexGrow, flexShrink, flexBasis) => ({
      display,
      flexGrow,
      flexShrink,
      flexBasis,
    }),
    dynamicHeaderContainer: (
      backgroundColor,
      borderBottomWidth,
      borderBottomStyle,
      borderBottomColor,
      textAlign,
      padding,
      width,
    ) => ({
      backgroundColor,
      borderBottomWidth,
      borderBottomStyle,
      borderBottomColor,
      textAlign,
      padding,
      width,
    }),
    dynamicPlaceholder: (marginInlineStart, marginTop) => ({
      marginInlineStart,
      marginTop,
    }),
    dynamicPlaceholderWithBugNub: (marginInlineStart, marginTop) => ({
      marginInlineStart,
      marginTop,
    }),
    dynamicTitle: (maxWidth) => ({
      maxWidth,
    }),
  }),
  stylex.create({
    dynamicShareFeedbackSticky: (position, end, bottom, zIndex) => ({
      position,
      end,
      bottom,
      zIndex,
    }),
    dynamicRoot: (height) => ({
      height,
    }),
    dynamicScrollable: (overflowY, padding, height) => ({
      overflowY,
      padding,
      height,
    }),
    dynamicContent: (display, flexDirection, width, margin) => ({
      display,
      flexDirection,
      width,
      margin,
    }),
    dynamicEditor: (
      boxSizing,
      backgroundColor,
      borderRadius,
      boxShadow,
      display,
      marginTop,
      marginBottom,
      maxWidth,
      width,
      minHeight,
      flexGrow,
      alignSelf,
      flexDirection,
      padding,
    ) => ({
      boxSizing,
      backgroundColor,
      borderRadius,
      boxShadow,
      display,
      marginTop,
      marginBottom,
      maxWidth,
      width,
      minHeight,
      flexGrow,
      alignSelf,
      flexDirection,
      padding,
    }),
    dynamicHeader: (mediaPrintDisplay) => ({
      '@media print': {
        display: mediaPrintDisplay,
      },
    }),
    dynamicEditorContainer: (display, justifyContent, height) => ({
      display,
      justifyContent,
      height,
    }),
    dynamicEditorInnerContainer: (width, zIndex) => ({
      width,
      zIndex,
    }),
    dynamicSidebarContainer: (
      width,
      flexShrink,
      paddingTop,
      backgroundColor,
      overflowY,
      height,
    ) => ({
      width,
      flexShrink,
      paddingTop,
      backgroundColor,
      overflowY,
      height,
    }),
    dynamicInnerContainer: (marginTop, height) => ({
      marginTop,
      height,
    }),
    dynamicNoteContainer: (display, flexDirection, height) => ({
      display,
      flexDirection,
      height,
    }),
  }),
  stylex.create({
    dynamicPhotoStyle: (paddingTop, height) => ({
      paddingTop,
      height,
    }),
  }),
  stylex.create({
    dynamicMetricCardContent: (
      borderTopStyle,
      borderInlineStartStyle,
      borderInlineEndStyle,
      borderBottomStyle,
      borderTopWidth,
      borderInlineStartWidth,
      borderInlineEndWidth,
      borderBottomWidth,
      boxSizing,
      display,
      flexGrow,
      flexShrink,
      marginTop,
      marginInlineEnd,
      marginBottom,
      marginInlineStart,
      minHeight,
      minWidth,
      paddingTop,
      paddingInlineEnd,
      paddingBottom,
      paddingInlineStart,
      position,
      zIndex,
      flexDirection,
      justifyContent,
      alignItems,
      height,
    ) => ({
      borderTopStyle,
      borderInlineStartStyle,
      borderInlineEndStyle,
      borderBottomStyle,
      borderTopWidth,
      borderInlineStartWidth,
      borderInlineEndWidth,
      borderBottomWidth,
      boxSizing,
      display,
      flexGrow,
      flexShrink,
      marginTop,
      marginInlineEnd,
      marginBottom,
      marginInlineStart,
      minHeight,
      minWidth,
      paddingTop,
      paddingInlineEnd,
      paddingBottom,
      paddingInlineStart,
      position,
      zIndex,
      flexDirection,
      justifyContent,
      alignItems,
      height,
    }),
    dynamicReactionRoot: (
      display,
      alignItems,
      paddingInlineStart,
      marginInlineEnd,
    ) => ({
      display,
      alignItems,
      paddingInlineStart,
      marginInlineEnd,
    }),
    dynamicReactionContainer: (
      width,
      height,
      borderColor,
      borderRadius,
      borderStyle,
      borderWidth,
      marginInlineStart,
      position,
    ) => ({
      width,
      height,
      borderColor,
      borderRadius,
      borderStyle,
      borderWidth,
      marginInlineStart,
      position,
    }),
    dynamicIconContainer: (
      marginInlineEnd,
      width,
      height,
      alignItems,
      borderRadius,
      borderWidth,
      boxSizing,
      display,
      justifyContent,
      padding,
      position,
    ) => ({
      marginInlineEnd,
      width,
      height,
      alignItems,
      borderRadius,
      borderWidth,
      boxSizing,
      display,
      justifyContent,
      padding,
      position,
    }),
    dynamicIconColorViewers: (backgroundColor) => ({ backgroundColor }),
    dynamicIconColorComments: (backgroundColor) => ({ backgroundColor }),
    dynamicIconColorQuestions: (backgroundColor) => ({ backgroundColor }),
    dynamicIconColorLikeReaction: (backgroundColor) => ({ backgroundColor }),
  }),
  stylex.create({
    dynamicContainer: (backgroundColor, height) => ({
      backgroundColor,
      height,
    }),
  }),
  stylex.create({
    dynamicRoot: (backgroundColor, width, height, maxHeight) => ({
      backgroundColor,
      width,
      height,
      maxHeight,
    }),
  }),
  stylex.create({
    dynamicContainer: (backgroundColor, display, height, margin, width) => ({
      backgroundColor,
      display,
      height,
      margin,
      width,
    }),
    dynamicBackgroundTeams: (backgroundColor) => ({ backgroundColor }),
  }),
  stylex.create({
    dynamicTitle: (width, borderRadius, height) => ({
      width,
      borderRadius,
      height,
    }),
    dynamicSubtitle: (width, borderRadius, height) => ({
      width,
      borderRadius,
      height,
    }),
  }),
  stylex.create({
    dynamicDefaultResponsiveWidth: (width, minWidth, maxWidth) => ({
      width,
      minWidth,
      maxWidth,
    }),
    dynamicContentScroll: (overflowY, paddingInline, height) => ({
      overflowY,
      paddingInline,
      height,
    }),
    dynamicMsteams: (marginInlineStart) => ({ marginInlineStart }),
  }),
  stylex.create({
    dynamicGlimmer: (height) => ({ height }),
    dynamicIcon: (marginInlineEnd, position, top) => ({
      marginInlineEnd,
      position,
      top,
    }),
    dynamicOuterCard: (backgroundColor, height, borderRadius) => ({
      backgroundColor,
      height,
      borderRadius,
    }),
    dynamicBackgroundTeams: (backgroundColor) => ({ backgroundColor }),
  }),
  stylex.create({
    dynamicBody: (marginTop) => ({ marginTop }),
  }),
  stylex.create({
    dynamicVideoOptionsCard: (
      padding,
      marginBottom,
      borderRadius,
      boxShadow,
    ) => ({
      padding,
      marginBottom,
      borderRadius,
      boxShadow,
    }),
  }),
  stylex.create({
    padding: {
      paddingBottom: 'var(--p-space-4)',
      paddingInline: 'var(--p-space-4)',
      paddingTop: 'var(--p-space-2)',
    },
    dynamicPadding: (padding) => ({
      padding,
    }),
  }),
  stylex.create({
    vert16: {
      paddingBlock: 16,
    },
    dynamicVert: (paddingBlock) => ({
      paddingBlock,
    }),
  }),
  stylex.create({
    item: {
      listStyleType: 'disc',
    },
    dynamicItem: (listStyleType) => ({
      listStyleType,
    }),
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      padding: '0px 16px',
    },
    dynamicContainer: (
      display,
      flexDirection,
      height,
      justifyContent,
      padding,
    ) => ({
      display,
      flexDirection,
      height,
      justifyContent,
      padding,
    }),
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--surface-background)',
      borderRadius: 8,
      boxShadow: '0 2px 12px var(--shadow-2)',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      padding: '16px',
      width: '100%',
    },
    dynamicRoot: (
      backgroundColor,
      borderRadius,
      boxShadow,
      boxSizing,
      display,
      flexDirection,
      height,
      justifyContent,
      padding,
      width,
    ) => ({
      backgroundColor,
      borderRadius,
      boxShadow,
      boxSizing,
      display,
      flexDirection,
      height,
      justifyContent,
      padding,
      width,
    }),
  }),
];