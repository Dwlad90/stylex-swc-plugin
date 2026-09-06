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
  stylex.create({
    bulletedList: {
      listStyleType: 'disc',
      marginInlineStart: 16,
    },
    cta: {
      paddingTop: 40,
    },
    favicon: {
      marginInlineStart: -10,
      paddingBottom: 20,
    },
    pushPageRoot: {
      boxSizing: 'border-box',
      maxWidth: 500,
      padding: 20,
      width: '100vw',
    },
    title: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    cta: {
      padding: '24px 16px 12px',
    },
    root: {
      boxSizing: 'border-box',
      height: 'fit-content',
      maxWidth: 500,
      width: '100vw',
    },
    title: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    descrSectionContent: {
      paddingBottom: 15,
      paddingTop: 20,
    },
  }),
  stylex.create({
    overlay: {
      alignContent: 'center',
      backgroundColor: 'var(--overlay-on-media)',
      bottom: 0,
      display: 'flex',
      end: 0,
      justifyContent: 'center',
      position: 'fixed',
      start: 0,
      top: 0,
    },
  }),
  stylex.create({
    cookieBanner: {
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      maxWidth: 950,
      padding: 20,
    },
    cookieBannerCNIL: {
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      maxWidth: '75%',
      padding: 20,
    },
    cookieBannerContainer: {
      backgroundColor: 'var(--card-background-flat)',
      bottom: 0,
      height: 'auto',
      position: 'fixed',
      width: '100%',
    },
    hideOnSmallerScreen: {
      '@media (max-width: 768px)': {
        display: 'none',
      },
    },
    showOnSmallerScreen: {
      display: 'none',
      '@media (max-width: 768px)': {
        display: 'block',
      },
    },
  }),
  stylex.create({
    list: {
      listStyleType: 'disc',
      paddingInlineStart: 16,
    },
    listItem: {
      paddingTop: 8,
    },
    scrollable: {
      boxSizing: 'border-box',
      maxHeight: '85vh',
      paddingBottom: 24,
      paddingInlineEnd: 24,
      paddingInlineStart: 24,
      paddingTop: 50,
    },
  }),
  stylex.create({
    bulletIcons: {
      paddingTop: 20,
    },
    cta: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--media-inner-border)',
      paddingBottom: '16px',
      paddingInlineEnd: '16px',
      paddingInlineStart: '16px',
    },
    favicon: {
      paddingBottom: 20,
    },
    pageRoot: {
      boxSizing: 'border-box',
      maxWidth: 680,
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
      paddingTop: 20,
      width: '100vw',
    },
    scrollable: {
      maxHeight: '60vh',
    },
    textSection: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 20,
    },
    title: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    section: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 20,
    },
    textSection: {
      paddingTop: 20,
    },
    titleSection: {
      paddingTop: 30,
    },
  }),
  stylex.create({
    section: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 20,
    },
    titleSection: {
      paddingTop: 30,
    },
  }),
  stylex.create({
    cta: {
      paddingTop: 40,
    },
    favicon: {
      paddingBottom: 20,
    },
    pushPageRoot: {
      boxSizing: 'border-box',
      maxWidth: 500,
      padding: 20,
      width: '100vw',
    },
    title: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    bulletIcons: {
      paddingTop: 20,
    },
    cta: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--media-inner-border)',
      paddingBottom: '16px',
      paddingInlineEnd: '16px',
      paddingInlineStart: '16px',
    },
    favicon: {
      paddingBottom: 20,
    },
    pageRoot: {
      boxSizing: 'border-box',
      maxWidth: 680,
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
      paddingTop: 20,
      width: '100vw',
    },
    scrollable: {
      maxHeight: '60vh',
    },
    textSection: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 20,
    },
    title: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    image: {
      borderRadius: 8,
    },
  }),
  stylex.create({
    background: {
      backgroundColor: 'var(--always-white)',
    },
  }),
  stylex.create({
    cta: {
      padding: '24px 16px 12px',
    },
    descriptionRoot: {
      height: 350,
      overflowY: 'scroll',
      padding: 16,
    },
    pageRoot: {
      boxSizing: 'border-box',
      height: 515,
      maxWidth: 680,
      width: '100vw',
    },
    root: {
      boxSizing: 'border-box',
      height: 515,
      maxWidth: 500,
      width: '100vw',
    },
    title: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    root: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      paddingBottom: 20,
      paddingTop: 10,
    },
    section: {
      paddingTop: 30,
    },
    subtitle: {
      paddingBottom: 10,
    },
    text: {
      paddingTop: 20,
    },
  }),
  stylex.create({
    root: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
    },
  }),
  stylex.create({
    cardList: {
      color: 'var(--secondary-text)',
      listStyle: 'disc',
      marginInlineStart: 16,
    },
    cardPadding: {
      padding: 16,
    },
  }),
  stylex.create({
    defaultAnchor: {
      minHeight: 300,
    },
  }),
  stylex.create({
    paragraphPadding: {
      marginTop: 16,
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
    },
    contentContainerVisibilityHidden: {
      visibility: 'hidden',
    },
  }),
  stylex.create({
    content: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh',
      position: 'relative',
    },
  }),
  stylex.create({
    closeButton: {
      height: 40,
      opacity: 1,
      position: 'fixed',
      start: 16,
      top: 8,
      width: 40,
      zIndex: 10,
    },
  }),
  stylex.create({
    closeButton: {
      height: 40,
      opacity: 1,
      position: 'fixed',
      start: 16,
      top: 8,
      width: 40,
      zIndex: 10,
    },
  }),
  stylex.create({
    closeButton: {
      height: 40,
      opacity: 1,
      position: 'fixed',
      start: 16,
      top: 8,
      width: 40,
      zIndex: 10,
    },
  }),
  stylex.create({
    contentContainer: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 'inherit',
      position: 'relative',
    },
    contentContainerHidden: {
      display: 'none',
    },
  }),
  stylex.create({
    contentContainer: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 'inherit',
      position: 'relative',
    },
    contentContainerHidden: {
      display: 'none',
    },
  }),
  stylex.create({
    input: {
      'caret-color': 'transparent',
    },
  }),
  stylex.create({
    displayInherit: {
      display: 'inherit',
    },
    inherit: {
      alignContent: 'inherit',
      alignItems: 'inherit',
      flexDirection: 'inherit',
      flexGrow: 'inherit',
      flexShrink: 'inherit',
      height: 'inherit',
      justifyContent: 'inherit',
      maxHeight: 'inherit',
      maxWidth: 'inherit',
      minHeight: 'inherit',
      minWidth: 'inherit',
      position: 'relative',
      width: 'inherit',
    },
  }),
  stylex.create({
    railContent: {
      fontSize: 14,
      margin: 15,
    },
    railItem: {
      marginBottom: 15,
    },
    root: {
      flexGrow: 1,
      listStyleType: 'none',
      margin: 20,
    },
    widgetSet: {
      display: 'flex',
      marginTop: 15,
    },
  }),
  stylex.create({
    container: {
      marginInlineEnd: 15,
    },
    keyInfo: {
      backgroundColor: 'var(--fds-dark-mode-gray-35)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--fds-dark-mode-gray-50)',
      borderRadius: 2,
      marginInlineEnd: 5,
      padding: '0 5px',
    },
    keyInfoItem: {
      marginTop: 10,
    },
  }),
  stylex.create({
    blueBackground: {
      backgroundColor: 'var(--accent)',
      color: 'var(--always-white)',
      padding: 8,
    },
    container: {
      borderWidth: 1,
      borderStyle: 'solid',
      marginInlineEnd: 15,
    },
    inputWrapper: {
      marginTop: 15,
    },
    redBackground: {
      backgroundColor: 'var(--negative)',
      color: 'var(--always-black)',
      padding: 8,
    },
    whiteBackground: {
      backgroundColor: 'var(--always-white)',
      color: 'var(--always-black)',
      padding: 8,
    },
  }),
  stylex.create({
    blueBackground: {
      backgroundColor: 'var(--accent)',
      color: 'var(--always-white)',
      padding: 8,
    },
    container: {
      borderWidth: 1,
      borderStyle: 'solid',
      marginInlineEnd: 15,
    },
    redBackground: {
      backgroundColor: 'var(--negative)',
      color: 'var(--always-black)',
      padding: 8,
    },
    whiteBackground: {
      backgroundColor: 'var(--always-white)',
      color: 'var(--always-black)',
      padding: 8,
    },
  }),
  stylex.create({
    blueBackground: {
      backgroundColor: 'var(--accent)',
      color: 'var(--always-white)',
      padding: 8,
    },
    container: {
      borderWidth: 1,
      borderStyle: 'solid',
      marginInlineEnd: 15,
    },
    greenBackground: {
      backgroundColor: 'var(--positive)',
      color: 'var(--always-white)',
      padding: 8,
    },
    redBackground: {
      backgroundColor: 'var(--negative)',
      color: 'var(--always-black)',
      padding: 8,
    },
    section: {
      marginBottom: 5,
    },
    whiteBackground: {
      backgroundColor: 'var(--always-white)',
      color: 'var(--always-black)',
      padding: 8,
    },
  }),
  stylex.create({
    keyInfo: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 6,
      display: 'inline-block',
      lineHeight: 1,
      margin: 3,
      minWidth: '0.75em',
      padding: 4,
      paddingInlineEnd: 6,
      paddingInlineStart: 6,
      textAlign: 'center',
    },
  }),
  stylex.create({
    keyInfo: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 6,
      display: 'inline-block',
      lineHeight: 1,
      margin: 3,
      minWidth: '0.75em',
      padding: 4,
      paddingInlineEnd: 6,
      paddingInlineStart: 6,
      textAlign: 'center',
    },
    list: {
      paddingBottom: 10,
      paddingTop: 5,
    },
    listItem: {
      paddingTop: 16,
    },
    plus: {
      marginInline: -1,
    },
  }),
  stylex.create({
    wrapperFocusable: {
      ':focus': {
        outline: 'none',
      },
    },
  }),
  stylex.create({
    commandList: {
      display: 'flex',
      flexGrow: 1,
      flexWrap: 'wrap',
      fontSize: 15,
      fontWeight: 500,
    },
    container: {
      backgroundColor: 'var(--nav-bar-background)',
      borderRadius: 8,
      boxShadow:
        '0 12px 28px 0 var(--shadow-2),0 2px 4px 0 var(--shadow-1),inset 0 0 0 1px var(--shadow-inset)',
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      display: 'flex',
      width: '100%',
    },
    contentWrapper: {
      padding: '16px 16px 0px',
    },
    flexWrapper: {
      alignItems: 'center',
      display: 'flex',
      fontSize: 15,
      fontWeight: 500,
      width: '100%',
    },
    footerWrapper: {
      paddingBottom: 16,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
    },
    headingWrapper: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      padding: '10px 16px',
    },
    listHeader: {
      fontWeight: 600,
    },
    listInFocus: {
      zIndex: 3,
    },
    listWrapper: {
      width: '100%',
    },
    metaText: {
      marginBottom: 10,
    },
    spacer: {
      flexGrow: 1,
    },
    wrapper: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      padding: 10,
      position: 'fixed',
      width: 348,
    },
  }),
  stylex.create({
    commandList: {
      display: 'flex',
      flexGrow: 1,
      flexWrap: 'wrap',
      fontSize: 15,
      fontWeight: 500,
      padding: '16px 0px',
    },
    divWrapper: {
      '@media only screen and (max-width: 970px)': {
        maxWidth: 'unset',
        paddingInlineEnd: 32,
        width: '100%',
      },
    },
    footer: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      paddingBottom: 16,
    },
    footerRow: {
      '@media (max-width: 960px)': {
        flexDirection: 'column',
      },
    },
    footerRowItem: {
      '@media (max-width: 960px)': {
        width: '100%',
      },
    },
    pinnedButton: {
      maxWidth: 380,
    },
    sectionWrapper: {
      maxWidth: 350,
      minWidth: 280,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      '@media only screen and (max-width: 970px)': {
        maxWidth: 'unset',
        width: '100%',
      },
    },
    settingInfo: {
      '@media (max-width: 960px)': {
        maxWidth: '100%',
      },
    },
    shortcutSetting: {
      maxWidth: 320,
    },
    wrapper: {
      display: 'flex',
      flexWrap: 'wrap',
      padding: '16px 0',
    },
  }),
  stylex.create({
    activeKey: {
      backgroundColor: 'var(--primary-button-background)',
      color: 'var(--primary-button-text)',
    },
    disabledKey: {
      borderColor: 'transparent',
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
    },
    keyInfo: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 6,
      boxSizing: 'border-box',
      display: 'inline-block',
      lineHeight: 1,
      margin: 3,
      minWidth: '0.75em',
      padding: 4,
      paddingInlineEnd: 6,
      paddingInlineStart: 6,
      textAlign: 'center',
    },
    keyInfoLarge: {
      marginBottom: 6,
      marginTop: 6,
    },
  }),
  stylex.create({
    compactDescriptionCellItem: {
      paddingInlineEnd: 10,
    },
    descriptionCellItem: {
      paddingBottom: 8,
      paddingInlineEnd: 30,
      paddingTop: 8,
    },
    disabled: {
      color: 'var(--disabled-text)',
    },
    enabled: {
      color: 'var(--primary-text)',
    },
    keyBlock: {
      alignItems: 'center',
      display: 'flex',
      flexWrap: 'nowrap',
      whiteSpace: 'nowrap',
    },
    keyCellItem: {
      textAlign: 'end',
      whiteSpace: 'nowrap',
      '@media only screen and (max-width: 970px)': {
        width: 140,
      },
    },
    rowItem: {
      padding: 0,
      textAlign: 'start',
    },
    tableCellItem: {
      fontSize: 15,
      fontWeight: 'inherit',
      lineHeight: 1.5,
      paddingBottom: 6,
      paddingTop: 6,
    },
  }),
  stylex.create({
    table: {
      width: '100%',
    },
  }),
  stylex.create({
    adChoiceIcon: {
      position: 'relative',
      top: -2,
    },
    inline: {
      display: 'inline',
    },
  }),
  stylex.create({
    adChoiceIcon: {
      position: 'relative',
      top: -2,
    },
    inline: {
      display: 'inline',
    },
    link: {
      color: 'var(--secondary-text)',
    },
  }),
  stylex.create({
    layoutEndButtons: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row-reverse',
      height: 'var(--header-height)',
      position: 'relative',
      zIndex: 0,
    },
  }),
  stylex.create({
    displayNone: {
      display: 'none',
    },
    layoutEndButton: {
      alignItems: 'center',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      marginInlineEnd: 8,
    },
    widePivotLink: {
      '@media (max-width: 1260px)': {
        display: 'none',
      },
      '@media (max-width: 1379px)': {
        maxWidth: 131,
      },
      '@media (min-width: 1380px) and (max-height: 789px)': {
        maxWidth: 190,
      },
      '@media (min-width: 1380px) and (min-height: 790px)': {
        maxWidth: 145,
      },
    },
  }),
  stylex.create({
    tabBar: {
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
    },
    tabBarRootView: {
      zIndex: 1,
    },
  }),
  stylex.create({
    calloutWidth: {
      maxWidth: 300,
    },
  }),
  stylex.create({
    glimmer: {
      borderRadius: '50%',
      height: 28,
      width: 28,
    },
    glimmerWrapper: {
      alignItems: 'center',
      display: 'flex',
      height: 'var(--header-height)',
      justifyContent: 'center',
    },
    tab: {
      flexGrow: 1,
      maxWidth: 129.6,
      minWidth: 50,
    },
    tabHiddenAtLargeViewport: {
      '@media (max-width: 1099px)': {
        display: 'none',
      },
    },
    tabHiddenAtSmallViewport: {
      '@media (max-width: 700px)': {
        display: 'none',
      },
    },
    tabResponsive: {
      '@media (max-width: 1099px)': {
        maxWidth: 'calc(15vw - 55px)',
      },
      '@media (min-width: 1100px) and (max-height: 789px), (min-width: 1100px) and (max-width: 1379px)':
        {
          maxWidth: 111.6,
        },
    },
    tabSpacing: {
      marginInlineStart: 8,
    },
  }),
  stylex.create({
    tabContainer: {
      display: 'flex',
      height: 'var(--header-height)',
      justifyContent: 'center',
    },
    tabs: {
      alignItems: 'flex-end',
      display: 'flex',
      flexGrow: 1,
      justifyContent: 'center',
      paddingInlineEnd: 110,
      paddingInlineStart: 110,
    },
    tabsResponsive: {
      '@media (max-width: 700px)': {
        justifyContent: 'flex-start',
      },
    },
  }),
  stylex.create({
    badgeContainer: {
      position: 'absolute',
      start: 20,
      top: -8,
    },
    hideMore: {
      display: 'none',
    },
    iconContainer: {
      position: 'relative',
    },
    link: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
      position: 'relative',
      width: '100%',
    },
    linkOverlayPressed: {
      backgroundColor: 'var(--press-overlay)',
    },
    linkUnderline: {
      backgroundColor: 'var(--primary-button-background)',
      borderTopEndRadius: 1,
      borderTopStartRadius: 1,
      bottom: 0,
      end: 2,
      height: 3,
      position: 'absolute',
      start: 2,
      transform: 'scaleY(0)',
      transformOrigin: 'center bottom',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'transform',
      transitionTimingFunction: 'var(--fds-soft)',
    },
    linkUnderlineSelected: {
      transform: 'none',
    },
    linkWrapper: {
      alignItems: 'center',
      display: 'flex',
      height: 'var(--header-height)',
      position: 'relative',
    },
    moreTab: {
      '@media (min-width: 1100px)': {
        display: 'none',
      },
    },
    tab: {
      flexGrow: 1,
      maxWidth: 129.6,
      minWidth: 50,
    },
    tab500: {
      maxWidth: 93.6,
    },
    tab584: {
      maxWidth: 110.4,
    },
    tabHiddenAtLargeViewport: {
      '@media (max-width: 1099px)': {
        display: 'none',
      },
    },
    tabHiddenAtSmallViewport: {
      '@media (max-width: 700px)': {
        display: 'none',
      },
    },
    tabResponsive: {
      '@media (max-width: 1099px)': {
        maxWidth: 'calc(15vw - 55px)',
      },
      '@media (min-width: 1100px) and (max-height: 789px), (min-width: 1100px) and (max-width: 1379px)':
        {
          maxWidth: 111.6,
        },
    },
    tabSpacing: {
      marginInlineStart: 8,
    },
    tabStyles1: {
      '@media (max-width: 999px)': {
        display: 'none',
      },
    },
    tabStyles2: {
      '@media (max-width: 899px)': {
        display: 'none',
      },
    },
    tabStyles3: {
      '@media (max-width: 799px)': {
        display: 'none',
      },
    },
    tabStyles4: {
      '@media (max-width: 699px)': {
        display: 'none',
      },
    },
    tabStyles5: {
      '@media (max-width: 599px)': {
        display: 'none',
      },
    },
  }),
];
