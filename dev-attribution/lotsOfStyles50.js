'use strict';

import * as stylex from '@stylexjs/stylex';

export const lotsOfStyles = [
  stylex.create({
    bottom12: {
      paddingBottom: 12,
    },
    bottom4: {
      paddingBottom: 4,
    },
    imageWidthFull: {
      width: '100%',
    },
  }),
  stylex.create({
    addButton: {
      position: 'absolute',
      start: 16,
      top: 16,
      zIndex: 2,
    },
    dropZone: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      minHeight: 254,
      width: '100%',
    },
    dropZoneBorder: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 4,
    },
    dropZoneDrag: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
      borderColor: 'var(--accent)',
    },
    imagenameWrapper: {
      bottom: 16,
      end: 16,
      maxWidth: '100%',
      position: 'absolute',
      start: 16,
      textAlign: 'start',
      zIndex: 2,
    },
    imageSizeLimits: {
      maxHeight: 254,
      maxWidth: '100%',
    },
    noPointerEvents: {
      pointerEvents: 'none',
    },
    overlay: {
      backgroundColor: 'var(--shadow-2)',
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 1,
    },
    removeButton: {
      end: 16,
      position: 'absolute',
      top: 16,
      zIndex: 2,
    },
    unviewablePreview: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 4,
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
    wrapper: {
      alignItems: 'center',
      borderRadius: 4,
      display: 'flex',
      height: 254,
      justifyContent: 'center',
      minHeight: 100,
      overflow: 'hidden',
      position: 'relative',
      textAlign: 'center',
      width: '100%',
    },
  }),
  stylex.create({
    fileRemoveButton: {
      paddingInlineEnd: 16,
    },
    imageFileRemoveButton: {
      end: 16,
      position: 'absolute',
      top: 16,
      zIndex: 2,
    },
    imagenameWrapper: {
      bottom: 16,
      end: 16,
      maxWidth: '100%',
      position: 'absolute',
      start: 16,
      textAlign: 'start',
      zIndex: 2,
    },
    imageSizeLimits: {
      maxHeight: 254,
      maxWidth: '100%',
    },
    overlay: {
      backgroundColor: 'var(--shadow-2)',
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 1,
    },
    wrapper: {
      alignItems: 'center',
      borderRadius: 4,
      display: 'flex',
      height: 254,
      justifyContent: 'center',
      minHeight: 100,
      overflow: 'hidden',
      position: 'relative',
      textAlign: 'center',
      width: '100%',
    },
  }),
  stylex.create({
    padding: {
      paddingBottom: 'var(--p-space-4)',
      paddingInline: 'var(--p-space-4)',
      paddingTop: 'var(--p-space-2)',
    },
  }),
  stylex.create({
    vert16: {
      paddingBlock: 16,
    },
  }),
  stylex.create({
    vert128: {
      paddingBlock: 128,
    },
    vert16: {
      paddingBlock: 16,
    },
  }),
  stylex.create({
    item: {
      listStyleType: 'disc',
    },
    list: {
      marginInlineStart: 24,
    },
  }),
  stylex.create({
    loadingParent: {
      alignItems: 'center',
      display: 'flex',
      height: '100%',
      paddingInlineEnd: 8,
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 7,
      height: 14,
      marginBottom: 14,
    },
    bodyGlimmerContainer: {
      padding: '16px 16px 32px 16px',
    },
    bodyGlimmerFirst: {
      width: '80%',
    },
    bodyGlimmerSecond: {
      width: '40%',
    },
    header: {
      alignItems: 'center',
      display: 'flex',
      height: 60,
      padding: '0 16px',
    },
    headerGlimmer: {
      borderRadius: 7,
      height: 14,
      width: 100,
    },
  }),
  stylex.create({
    card: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    marginInline: {
      marginInlineEnd: 8,
      marginInlineStart: 8,
    },
  }),
  stylex.create({
    fontWeightNormal: {
      fontWeight: 'normal',
    },
  }),
  stylex.create({
    container: {
      wordBreak: 'break-word',
    },
  }),
  stylex.create({
    backgroundOpacity: {
      opacity: 0.7,
    },
    cardContainer: {
      borderRadius: 16,
      display: 'flex',
      justifyContent: 'center',
      margin: '0px auto',
      overflow: 'hidden',
    },
    contentWrapper: {
      alignItems: 'center',
      backgroundClip: 'padding-box',
      borderRadius: 16,
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'flex-start',
      overflow: 'hidden',
      paddingBottom: 20,
      paddingInline: 20,
      zIndex: 1000,
    },
    fallbackImageBackground: {
      backgroundColor: 'var(--card-background)',
    },
    imageContainer: {
      borderRadius: 16,
      end: 0,
      height: '100%',
      overflow: 'hidden',
      position: 'absolute',
      textAlign: 'end',
      width: '100%',
    },
    shadow: {
      boxShadow: '0 2px 12px var(--shadow-2)',
    },
  }),
  stylex.create({
    cardContainer: {
      display: 'flex',
      justifyContent: 'center',
      margin: '0px auto',
    },
  }),
  stylex.create({
    buttonWrapper: {
      marginBottom: 16,
      marginInline: 12,
    },
    satpBackgroundWrapper: {
      borderRadius: 16,
      height: 576,
      margin: '32px auto 32px auto',
      overflow: 'hidden',
      width: 432,
    },
    statusAreaWrapper: {
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      paddingInline: 9,
      width: '100%',
    },
  }),
  stylex.create({
    activeBadge: {
      backgroundColor: 'var(--notification-badge)',
      borderRadius: '4px',
      position: 'absolute',
    },
    badgeDefault: {
      bottom: -10,
      end: 55,
      padding: 8,
    },
    badgeScaled: {
      bottom: -8,
      end: 43,
      padding: 6,
    },
    photoWrapper: {
      alignSelf: 'center',
      paddingBlock: 24,
      position: 'relative',
    },
  }),
  stylex.create({
    buttonWrapper: {
      width: '100%',
    },
    iconContainer: {
      borderWidth: 4,
      borderStyle: 'solid',
      borderColor: 'var(--always-white)',
      borderRadius: 360,
      maxHeight: 64,
      maxWidth: 64,
      padding: 16,
    },
    mainContent: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      paddingBottom: 20,
      width: '100%',
    },
    textContainer: {
      justifyContent: 'flex-start',
      marginTop: 12,
      paddingInline: 10,
      wordBreak: 'break-word',
    },
  }),
  stylex.create({
    circle: {
      backgroundColor: 'var(--positive)',
      borderRadius: 360,
      padding: '0px 4px 2.5px 4px',
    },
    container: {
      alignItems: 'center',
      color: 'var(--primary-text)',
      display: 'flex',
      flexDirection: 'row',
      paddingInlineStart: 4,
    },
    textContainer: {
      padding: '4px 2px 2px 2px',
      whiteSpace: 'nowrap',
    },
  }),
  stylex.create({
    actionLinksContainer: {
      alignContent: 'center',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'center',
      lineHeight: 1,
      paddingTop: '24px',
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      padding: '0 0 40px 0',
      width: '100%',
    },
    menuContainer: {
      alignSelf: 'flex-end',
      paddingTop: 20,
    },
    middotContainer: {
      fontWeight: 'bold',
      padding: '0 5px 0 5px',
    },
    textContainer: {
      wordBreak: 'break-word',
    },
    timestampContainer: {
      fontWeight: 'normal',
    },
    translateContainer: {
      fontWeight: 'bold',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      paddingBottom: 20,
      width: '100%',
    },
    textContainer: {
      justifyContent: 'flex-start',
      paddingInline: 5,
      wordBreak: 'break-word',
    },
  }),
  stylex.create({
    buttonWrapper: {
      width: '100%',
    },
  }),
  stylex.create({
    authorContainer: {
      display: 'flex',
      flexDirection: 'row',
      marginBottom: '4px',
    },
    authorNameContainer: {
      alignItems: 'center',
      display: 'flex',
      flexGrow: 1,
      marginInlineStart: 6,
    },
    container: {
      alignItems: 'start',
      borderRadius: 18,
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 2,
      padding: '10px 16px 12px 16px',
    },
    darkContainer: {
      backgroundColor: 'var(--attachment-footer-background)',
      color: 'var(--always-white)',
    },
    lightContainer: {
      backgroundColor: 'var(--surface-background)',
      color: 'var(--primary-text)',
    },
    questionContainer: {
      fontSize: '18px',
      fontWeight: 'bold',
      width: '100%',
      wordBreak: 'break-word',
    },
    verifiedBadge: {
      alignItems: 'center',
      display: 'flex',
      marginInlineStart: 4,
    },
  }),
  stylex.create({
    answerContainer: {
      alignItems: 'center',
      display: 'flex',
      flexGrow: 1,
      padding: '0 0 40px 0',
    },
    questionContainer: {
      alignSelf: 'flex-start',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'space-between',
      padding: '20px 20px 0 0',
      width: '100%',
    },
  }),
  stylex.create({
    buttonWrapper: {
      display: 'flex',
    },
    container: {
      alignItems: 'center',
      backgroundColor: 'var(--card-background)',
      display: 'flex',
      justifyContent: 'space-between',
      marginTop: 8,
      padding: '10px 10px 20px 10px',
    },
    footer: {
      bottom: '0',
      marginBottom: '-10px',
      position: 'absolute',
      width: '100%',
    },
    iconWrapper: {
      marginInlineEnd: 4,
      marginInlineStart: 4,
    },
    replyShareButton: {
      alignItems: 'center',
      borderRadius: 4,
      display: 'flex',
      padding: 4,
    },
    ufiActions: {
      display: 'flex',
      justifyContent: 'space-between',
      paddingInlineStart: 2,
    },
    ufiSummary: {
      display: 'flex',
      flexDirection: 'row',
    },
  }),
  stylex.create({
    container: {
      maxWidth: 300,
      padding: 16,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      padding: '0px 16px',
    },
    ufiGlimmer: {
      borderRadius: 8,
      height: 35,
      margin: '0 0 16px',
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--surface-background)',
      borderRadius: 20,
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      padding: 16,
    },
  }),
  stylex.create({
    backgroundOpacity: {
      opacity: 0.7,
    },
    colorBackground: {
      height: '100%',
      position: 'absolute',
      top: 0,
      width: '100%',
    },
    contentWrapper: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      overflow: 'hidden',
      zIndex: 1000,
    },
    imageContainer: {
      end: 0,
      height: '100%',
      position: 'absolute',
      textAlign: 'end',
      width: '100%',
    },
    satpBackground: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      width: '100%',
    },
    textWrapper: {
      display: 'flex',
      maxWidth: 'calc(430px - 40px)',
      padding: 20,
    },
  }),
  stylex.create({
    seeMore: {
      opacity: 0.7,
    },
  }),
  stylex.create({
    content: {
      marginInlineEnd: -16,
      marginInlineStart: -16,
    },
    root: {
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      padding: 16,
    },
    root: {
      backgroundColor: 'var(--comment-background)',
      position: 'relative',
    },
  }),
  stylex.create({
    attachmentPhoto: {
      alignItems: 'center',
      alignSelf: 'stretch',
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
    },
    backgroundImage: {
      height: 252,
      position: 'absolute',
      top: 0,
      width: '100%',
    },
    root: {
      overflow: 'auto',
      position: 'relative',
    },
  }),
  stylex.create({
    contentContainer: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 'calc(-100vh +  var(--header-height))',
      minHeight: 'inherit',
      position: 'relative',
      zIndex: 0,
    },
    contentContainerContainment: {
      contain: 'style layout paint',
    },
    contentContainerHidden: {
      display: 'none',
    },
    contentContainerHiddenContentVisibility: {
      contentVisibility: 'hidden',
      position: 'absolute',
      start: '-100000px',
    },
    contentContainerVisibilityHidden: {
      visibility: 'hidden',
    },
  }),
  stylex.create({
    base: {
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
    },
    innerHiddenTopNav: {
      minHeight: '100vh',
      top: 0,
    },
    innerHiddenTopNavDvh: {
      '@supports (min-height: 100dvh)': {
        minHeight: '100dvh',
      },
    },
    innerWithTopNav: {
      minHeight: 'calc(100vh - var(--header-height))',
      top: 'var(--header-height)',
    },
    innerWithTopNavDvh: {
      '@supports (min-height: 100dvh)': {
        minHeight: 'calc(100dvh - var(--header-height))',
      },
    },
    outerWithExpandedOnLargeScreensGlobalPanel: {
      start: 'var(--global-panel-width-expanded)',
      width: 'calc(100% - var(--global-panel-width-expanded))',
      '@media (max-width: 1159px)': {
        start: 'var(--global-panel-width)',
        width: 'calc(100% - var(--global-panel-width))',
      },
    },
    outerWithGlobalPanel: {
      start: 'var(--global-panel-width)',
      width: 'calc(100% - var(--global-panel-width))',
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
      zIndex: 0,
    },
  }),
  stylex.create({
    offscreenAccessibilityElement: {
      clip: 'rect(0, 0, 0, 0)',
      clipPath: 'inset(50%)',
      height: 1,
      overflow: 'hidden',
      position: 'absolute',
      width: 1,
    },
  }),
  stylex.create({
    wordmark: {
      borderRadius: 8,
      marginBottom: 4,
      marginTop: 6,
      padding: 4,
      paddingBottom: 2,
    },
  }),
  stylex.create({
    badgeContainer: {
      position: 'absolute',
      zIndex: 1,
    },
  }),
  stylex.create({
    badgeOffset: {
      start: 26,
      top: -6,
    },
  }),
  stylex.create({
    personalProfilePageAdminSwitcherTooltip: {
      maxWidth: 300,
    },
    profileName: {
      fontWeight: 'bold',
    },
    tooltip: {
      maxWidth: 250,
    },
  }),
  stylex.create({
    badge: {
      end: 0,
      position: 'absolute',
      top: 0,
    },
    horizontalOffset: {
      paddingInline: 12,
    },
    wrapper: {
      display: 'flex',
      position: 'relative',
    },
  }),
  stylex.create({
    iconDisabled: {
      alignItems: 'center',
      backgroundColor: 'var(--primary-deemphasized-button-background)',
      borderRadius: '50%',
      display: 'flex',
      height: 40,
      justifyContent: 'center',
      width: 40,
    },
    pressableOverlayPressed: {
      backgroundColor: 'var(--press-overlay)',
    },
  }),
  stylex.create({
    actions: {
      height: 22,
      width: 24,
    },
    card: {
      width: 360,
    },
    cardFullHeight: {
      height: 'calc(100vh - var(--header-height) - 16px)',
      maxWidth: 'calc(100vw - 24px)',
    },
    heading: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'space-between',
      minHeight: 32,
      padding: '12px 16px 4px',
    },
    root: {
      marginInlineEnd: 8,
      marginTop: 5,
    },
  }),
  stylex.create({
    root: {
      marginInlineEnd: 8,
      marginTop: 5,
    },
  }),
  stylex.create({
    card: {
      display: 'flex',
      flexDirection: 'column',
      maxWidth: 'calc(100vw - 24px)',
      minHeight: 'inherit',
    },
    cardFullHeight: {
      minHeight: 'calc(100vh - var(--header-height) - 16px)',
    },
    cardMaxHeight: {
      maxHeight: 'calc(100vh - var(--header-height) - 16px)',
    },
    cardPanelHeight: {
      height: 'calc(100vh - var(--header-height))',
    },
    cardWidth: {
      width: 360,
    },
  }),
  stylex.create({
    'base-wash': {
      backgroundColor: 'var(--wash)',
    },
    'card-flat': {
      backgroundColor: 'var(--card-background-flat)',
    },
    'dark-wash': {
      backgroundColor: 'var(--shadow-5)',
    },
    error: {
      backgroundColor: 'var(--negative)',
    },
    highlight: {
      backgroundColor: 'var(--accent)',
    },
    'light-wash': {
      backgroundColor: 'var(--web-wash)',
    },
    transparent: {
      backgroundColor: 'transparent',
    },
    white: {
      backgroundColor: 'var(--surface-background)',
    },
  }),
  stylex.create({
    heading: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'space-between',
      minHeight: 32,
      padding: '12px 16px 4px',
    },
    headingOffsetWithGlobalPanel: {
      paddingTop: 20,
    },
  }),
  stylex.create({
    profileName: {
      fontWeight: 'bold',
    },
  }),
  stylex.create({
    paddingAll: {
      paddingTop: 10,
    },
    paddingDefault: {
      paddingBottom: 20,
    },
  }),
];
