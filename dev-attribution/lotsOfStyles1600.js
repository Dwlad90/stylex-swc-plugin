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
  stylex.create({
    layoutStartSearch: {
      boxSizing: 'border-box',
      height: 'var(--header-height)',
      maxWidth: '100vw',
      position: 'fixed',
      start: 0,
      top: 0,
      width: 112,
      zIndex: 2,
    },
    layoutStartSearchFocused: {
      width: 320,
      zIndex: 4,
    },
    layoutStartSearchFocusedOnNotHome: {
      width: 360,
    },
    layoutStartSearchInputInCollapsedClosedAndBlurredTypeahead: {
      cursor: 'pointer',
      paddingInlineStart: 24,
    },
    layoutStartSearchInputInExpandedClosedAndBlurredTypeahead: {
      '@media (max-width: 1259px)': {
        cursor: 'pointer',
        paddingInlineStart: 24,
      },
    },
    layoutStartSearchOnHome: {
      '@media (min-width: 1260px)': {
        width: 320,
      },
    },
    layoutStartSearchOnSearch: {
      '@media (min-width: 1260px)': {
        width: 360,
      },
    },
    layoutStartSearchOpened: {
      backgroundColor: 'var(--card-background)',
    },
    placeholder: {
      width: 48,
    },
  }),
  stylex.create({
    panel: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderInlineEndColor: 'var(--wash)',
      bottom: 0,
      display: 'flex',
      position: 'fixed',
      top: 'var(--header-height)',
      width: 'var(--global-panel-width)',
      zIndex: 1,
    },
    panelCollapsedBackground: {
      backgroundColor: 'var(--surface-background)',
    },
    panelExpandedOnLargeScreens: {
      borderInlineEndWidth: 'unset',
      borderInlineEndStyle: 'unset',
      borderInlineEndColor: 'unset',
      width: 'var(--global-panel-width-expanded)',
      '@media (max-width: 1159px)': {
        borderWidth: 1,
        borderStyle: 'solid',
        borderInlineEndColor: 'var(--wash)',
        width: 'var(--global-panel-width)',
      },
    },
    panelExpandedOnLargeScreensBackground: {
      backgroundColor: 'var(--web-wash)',
      '@media (max-width: 1159px)': {
        backgroundColor: 'var(--surface-background)',
      },
    },
  }),
  stylex.create({
    content: {
      height: '100%',
    },
    scrollableAreaTransitioning: {
      width: 'var(--global-panel-width-expanded)',
    },
  }),
  stylex.create({
    backButtonWithLogo: {
      paddingTop: 4,
      position: 'absolute',
    },
    backButtonWithLogoHidden: {
      pointerEvents: 'none',
    },
    topBar: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
      zIndex: 1,
    },
  }),
  stylex.create({
    root: {
      bottom: -10,
      display: 'flex',
      end: -8,
      position: 'absolute',
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--wash)',
      borderRadius: 8,
      bottom: 0,
      end: 4,
      paddingInline: 4,
    },
  }),
  stylex.create({
    footer: {
      padding: 16,
      '@media (max-width: 1159px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    pressable: {
      marginBlock: 2,
    },
    root: {
      position: 'relative',
    },
  }),
  stylex.create({
    root: {
      paddingTop: 9,
    },
  }),
  stylex.create({
    border: {
      backgroundColor: 'var(--wash)',
      bottom: 0,
      position: 'absolute',
      top: 0,
      width: 1,
    },
    card: {
      backgroundColor: 'var(--card-background)',
      minHeight: 'calc(100vh - var(--header-height))',
    },
  }),
  stylex.create({
    scrollableAreaStyle: {
      paddingBlock: 8,
      width: 360,
    },
  }),
  stylex.create({
    card: {
      width: '100%',
    },
    cardMargin: {
      marginBottom: 4,
    },
    container: {
      width: '100%',
    },
    divider: {
      marginBottom: 8,
      marginInline: 16,
    },
    dividerCollapsed: {
      width: 'calc(var(--global-panel-width) - 32px)',
    },
  }),
  stylex.create({
    labelContainedIcon: {
      marginInlineStart: 8,
      width: 'calc(var(--global-panel-width-expanded) - var(--global-panel-width) - 8px)',
    },
    labelIcon: {
      marginInlineStart: 14,
      width: 'calc(var(--global-panel-width-expanded) - var(--global-panel-width) - 14px)',
    },
    pressable: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 20,
      paddingBlock: 8,
    },
    pressableContainedIcon: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 14,
      paddingBlock: 4,
    },
    profilePhoto: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 18,
      paddingBlock: 6,
    },
    root: {
      display: 'flex',
      justifyContent: 'flex-start',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    label: {
      marginInlineStart: 12,
      width: 'calc(var(--global-panel-width-expanded) - var(--global-panel-width) - 12px)',
      '@media (max-width: 1159px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    indicator: {
      backgroundColor: 'var(--accent)',
      borderBottomEndRadius: 8,
      borderTopEndRadius: 8,
      height: 36,
      opacity: 0,
      position: 'absolute',
      start: 0,
      top: '50%',
      transform: 'scaleY(0) translateY(-50%)',
      transformOrigin: 'top',
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionProperty: 'opacity, transform',
      transitionTimingFunction: 'var(--fds-animation-expand-collapse-in)',
      width: 4,
    },
    selectedIndicator: {
      opacity: 1,
      transform: 'scaleY(1) translateY(-50%)',
    },
  }),
  stylex.create({
    badge: {
      end: -16,
      position: 'absolute',
      top: -8,
    },
    pressable: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 18,
      paddingBlock: 8,
    },
    root: {
      display: 'flex',
      position: 'relative',
    },
  }),
  stylex.create({
    pressable: {
      marginBlock: 2,
    },
    root: {
      position: 'relative',
    },
  }),
  stylex.create({
    search: {
      borderRadius: 20,
      height: 36,
      width: '100%',
    },
  }),
  stylex.create({
    baseTarget: {
      borderRadius: 8,
      pointerEvents: 'none',
      position: 'absolute',
      width: 'calc(var(--global-panel-width) - 16px)',
    },
    cardTarget: {
      bottom: 0,
      height: '100%',
      start: 8,
      width: 'calc(var(--global-panel-width) - 16px)',
    },
  }),
  stylex.create({
    baseTarget: {
      height: '100%',
      pointerEvents: 'none',
      position: 'absolute',
      width: 'calc(var(--global-panel-width) - 16px)',
    },
  }),
  stylex.create({
    mask: {
      backgroundColor: 'var(--overlay-alpha-80)',
      bottom: 0,
      end: 0,
      position: 'fixed',
      start: 'var(--global-panel-width)',
      top: 'var(--header-height)',
    },
    root: {
      position: 'fixed',
      start: 'var(--global-panel-width)',
      top: 'var(--header-height)',
    },
  }),
  stylex.create({
    pressable: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 18,
      paddingBlock: 8,
    },
    root: {
      display: 'flex',
      justifyContent: 'flex-start',
      position: 'relative',
    },
  }),
  stylex.create({
    badgeCollapsed: {
      end: 4,
      position: 'absolute',
      top: 0,
    },
    badgeExpanded: {
      end: 16,
      position: 'absolute',
      top: 10,
      '@media (max-width: 1159px)': {
        end: 4,
        top: 0,
      },
    },
  }),
  stylex.create({
    calloutWidth: {
      maxWidth: 300,
    },
  }),
  stylex.create({
    root: {
      flexGrow: 1,
      marginTop: 8,
    },
  }),
  stylex.create({
    expandedNonHomeSearchInput: {
      '@media (min-width: 900px)': {
        width: 320,
      },
    },
    homeClosedSearchInput: {
      maxWidth: '100%',
      '@media (max-width: 899px)': {
        maxWidth: 40,
      },
      '@media (min-width: 900px) and (max-width: 1159px)': {
        maxWidth: 532,
      },
    },
    homeOpenedSearchInput: {
      maxWidth: '100%',
      '@media (min-width: 649px) and (max-width: 899px)': {
        maxWidth: 320,
      },
      '@media (min-width: 900px) and (max-width: 1159px)': {
        maxWidth: 532,
      },
    },
    homeSearchContainer: {
      boxSizing: 'border-box',
      display: 'flex',
      flexBasis: 744,
      flexGrow: 1,
      justifyContent: 'center',
      minWidth: 0,
      paddingInline: 32,
      '@media (max-width: 1159px)': {
        paddingInline: 0,
      },
    },
    nonHomeClosedSearchInput: {
      width: 40,
    },
    nonHomeOpenedSearchInput: {
      width: 320,
      '@media (max-width: 648px)': {
        width: '100%',
      },
    },
    rightRailPlaceholder: {
      flexBasis: 360,
      flexShrink: 9999,
      maxWidth: 360,
      minWidth: 280,
      '@media (max-width: 1159px)': {
        display: 'none',
      },
    },
    root: {
      display: 'flex',
      height: 'var(--header-height)',
      position: 'fixed',
      top: 0,
      zIndex: 2,
    },
    rootHome: {
      end: 0,
      start: 'var(--global-panel-width-expanded)',
      '@media (max-width: 899px)': {
        end: 'unset',
        start: 160,
      },
      '@media (min-width: 900px) and (max-width: 1159px)': {
        end: 160,
        start: 160,
      },
    },
    rootNonHome: {
      start: 160,
    },
    rootOpened: {
      '@media (max-width: 648px)': {
        backgroundColor: 'var(--surface-background)',
        end: 0,
        start: 0,
        zIndex: 4,
      },
    },
    searchContainer: {
      width: '100%',
    },
  }),
  stylex.create({
    homeAndSERPClosedSearchInputInner: {
      marginInlineStart: -28,
      paddingInlineStart: 36,
      ':hover': {
        backgroundColor: 'var(--hover-overlay)',
      },
      '@media (max-width: 899px)': {
        cursor: 'pointer',
        marginInlineStart: -28,
        paddingInlineStart: 28,
      },
    },
    nonHomeClosedSearchInputInner: {
      cursor: 'pointer',
      marginInlineStart: -28,
      paddingInlineStart: 28,
      ':hover': {
        backgroundColor: 'var(--hover-overlay)',
      },
    },
    searchGlimmer: {
      height: 40,
      marginTop: 8,
      minWidth: 40,
    },
  }),
  stylex.create({
    homeAndSERPClosedSearchInputInner: {
      marginInlineStart: -28,
      paddingInlineStart: 36,
      ':hover': {
        backgroundColor: 'var(--hover-overlay)',
      },
      '@media (max-width: 899px)': {
        cursor: 'pointer',
        marginInlineStart: -28,
        paddingInlineStart: 28,
      },
    },
    nonHomeClosedSearchInputInner: {
      cursor: 'pointer',
      marginInlineStart: -28,
      paddingInlineStart: 28,
      ':hover': {
        backgroundColor: 'var(--hover-overlay)',
      },
    },
    searchGlimmer: {
      height: 40,
      marginTop: 8,
      minWidth: 40,
    },
  }),
  stylex.create({
    popoverButton: {
      marginBottom: 2,
    },
  }),
  stylex.create({
    pressable: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 18,
      paddingBlock: 8,
      position: 'relative',
    },
  }),
  stylex.create({
    paddingTop: {
      paddingTop: 120,
    },
  }),
  stylex.create({
    hideCreateJewelOnLargeScreens: {
      '@media (min-width: 1100px)': {
        display: 'none',
      },
    },
    hideMegaMenuJewelOnSmallScreens: {
      '@media (max-width: 1099px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    cardWidth: {
      width: 360,
    },
    minHeight: {
      minHeight: 'calc(100vh - 118px)',
    },
  }),
  stylex.create({
    createMenu: {
      maxWidth: 200,
      overflowAnchor: 'none',
    },
    offsetWithGlobalPanel: {
      paddingTop: 26,
    },
    scrollView: {
      height: 'calc(100vh - 118px)',
      paddingBottom: 16,
      paddingInline: 16,
    },
  }),
  stylex.create({
    textGlimmer: {
      borderRadius: 8,
      height: 15,
    },
    textGlimmerWidth35: {
      width: '35%',
    },
  }),
  stylex.create({
    horizontalOffset: {
      paddingInline: 14,
    },
  }),
  stylex.create({
    input: {
      height: 36,
    },
  }),
  stylex.create({
    textGlimmer: {
      borderRadius: 8,
      height: 15,
    },
    textGlimmerWidth20: {
      width: '20%',
    },
  }),
  stylex.create({
    card: {
      width: 608,
    },
    cardOffsetWithGlobalPanel: {
      paddingTop: 10,
    },
    createMenu: {
      width: 212,
    },
    megaMenu: {
      width: 372,
    },
  }),
  stylex.create({
    pressable: {
      padding: 8,
      width: '100%',
    },
    removeButton: {
      opacity: 0,
      position: 'absolute',
      start: 'calc(100% / 2 + 6px)',
      top: 0,
    },
    removeButtonVisible: {
      opacity: 1,
    },
    wrapper: {
      height: '100%',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    nullState: {
      display: 'flex',
      height: '100%',
    },
  }),
  stylex.create({
    aboveEverything: {
      zIndex: 3,
    },
    absolutePosition: {
      position: 'absolute',
    },
    backButton: {
      alignItems: 'center',
      boxSizing: 'border-box',
      display: 'flex',
      height: 'var(--header-height)',
      opacity: 0,
      padding: '12px 0',
      pointerEvents: 'none',
      transitionDuration: 'var(--fds-duration-extra-extra-short-in)',
      transitionProperty: 'opacity, transform',
      transitionTimingFunction: 'var(--fds-animation-enter-exit-out)',
      width: 40,
    },
    backButtonLTR: {
      transform: 'translateX(-32px) translateZ(0)',
    },
    backButtonRTL: {
      transform: 'translateX(32px) translateZ(0)',
    },
    backButtonVisible: {
      opacity: 1,
      pointerEvents: 'auto',
    },
    backButtonVisibleLTR: {
      transform: 'translateX(16px) translateZ(0)',
    },
    backButtonVisibleRTL: {
      transform: 'translateX(-16px) translateZ(0)',
    },
    layoutEnd: {
      alignItems: 'center',
      display: 'flex',
      height: 'var(--header-height)',
      paddingInlineEnd: 16,
      paddingInlineStart: 4,
    },
    layoutEndButtons: {
      alignItems: 'center',
      display: 'flex',
    },
    layoutEndFixed: {
      end: 0,
      position: 'fixed',
      top: 0,
    },
    layoutStartButtons: {
      alignItems: 'center',
      display: 'flex',
      pointerEvents: 'none',
      position: 'absolute',
      width: '100%',
      zIndex: 1,
    },
    layoutStartButtonsFixed: {
      position: 'fixed',
      start: 0,
      top: 0,
    },
    layoutStartWithSearch: {
      alignItems: 'center',
      display: 'flex',
      flexGrow: 1,
      position: 'relative',
    },
    logo: {
      alignItems: 'center',
      display: 'flex',
      height: 'var(--header-height)',
      opacity: 1,
      transitionDuration: 'var(--fds-duration-extra-extra-short-in)',
      transitionProperty: 'opacity, transform',
      transitionTimingFunction: 'linear',
      width: '100%',
    },
    logoHiddenLTR: {
      transform: 'translateX(24px) translateZ(0)',
      visibility: 'hidden',
    },
    logoHiddenRTL: {
      transform: 'translateX(-24px) translateZ(0)',
      visibility: 'hidden',
    },
    logoLTR: {
      transform: 'translateX(-24px) translateZ(0)',
    },
    logoRTL: {
      transform: 'translateX(24px) translateZ(0)',
    },
    tabBar: {
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
      zIndex: 1,
    },
    tabBarContent: {
      display: 'flex',
      justifyContent: 'space-between',
      position: 'absolute',
      width: '100%',
    },
    tabBarContentFixed: {
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
      zIndex: 1,
    },
  }),
  stylex.create({
    card: {
      maxHeight: 'calc(100vh - 60px)',
      maxWidth: 'calc(100vw - 24px)',
      width: 360,
    },
    root: {
      marginTop: 5,
    },
  }),
  stylex.create({
    menuItems: {
      padding: '4px 0 8px 0',
    },
  }),
  stylex.create({
    cardPressable: {
      alignItems: 'center',
      display: 'block',
      justifyContent: 'center',
      margin: '8px 4px',
      width: 'auto',
    },
    cardStyle: {
      margin: '4px 16px 16px 16px',
    },
    divider: {
      margin: '0 16px',
    },
    pulseEffectContainer: {
      display: 'block',
      width: 'auto',
    },
    pulseEffectContainerInner: {
      borderRadius: 8,
    },
    quickSwitchPicContainer: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 24,
      padding: 3,
      position: 'absolute',
    },
    quickSwitchPressable: {
      alignItems: 'center',
      justifyContent: 'center',
    },
    secondaryIconContainer: {
      transform: 'rotate(-180deg)',
    },
    secondaryIconContainerHover: {
      transform: 'rotate(-0deg)',
      transitionDuration: '500ms',
      transitionProperty: 'transform',
      transitionTimingFunction: 'ease-in-out',
    },
  }),
  stylex.create({
    card: {
      width: 360,
    },
    cardLegacy: {
      padding: '8px 0',
      width: 360,
    },
    divider: {
      margin: '0 16px',
    },
    headerCardStyle: {
      margin: '12px 16px 16px 16px',
    },
    headerCellPrimary: {
      margin: '16px 12px 8px 12px',
    },
    headerCellSecondary: {
      margin: '16px 12px 12px 12px',
    },
    listFooter: {
      margin: '8px 0px',
    },
    root: {
      marginTop: 5,
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      padding: '16px 16px 8px 16px',
    },
    icon: {
      padding: 8,
    },
    list: {
      padding: '8px 0 24px',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 10,
    },
  }),
  stylex.create({
    divider: {
      marginBottom: 8,
      marginTop: 24,
    },
    list: {
      padding: '8px 0 24px',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    banner: {
      backgroundColor: 'var(--card-background-flat)',
      boxSizing: 'border-box',
      padding: '16px 16px 16px 16px',
    },
    bannerSpacing: {
      padding: '16px 16px 8px 16px',
    },
    header: {
      display: 'flex',
      flexDirection: 'row',
      padding: '16px 16px 8px 16px',
    },
    icon: {
      padding: 8,
    },
    list: {
      marginBottom: 8,
      marginInline: 8,
      marginTop: 4,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 10,
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      padding: '16px 16px 8px 16px',
    },
    icon: {
      padding: 8,
    },
    list: {
      marginBottom: 8,
      marginInline: 8,
      marginTop: 4,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 10,
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      padding: '16px 16px 8px 16px',
    },
    icon: {
      padding: 8,
    },
    list: {
      padding: '8px 0 24px',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 10,
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      padding: 16,
    },
    icon: {
      padding: 8,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 24,
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 10,
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      padding: '16px 16px 8px 16px',
    },
    icon: {
      padding: 8,
    },
    list: {
      padding: '8px 0 24px',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 10,
    },
  }),
  stylex.create({
    content: {
      padding: '12px 16px',
      position: 'relative',
    },
    list: {
      paddingBottom: 16,
      paddingTop: 4,
    },
    tab: {
      paddingBottom: 20,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 20,
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
    icon: {
      padding: 8,
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 12,
    },
  }),
  stylex.create({
    list: {
      paddingBottom: 24,
      paddingTop: 8,
    },
    separator: {
      paddingBottom: 4,
      paddingTop: 16,
    },
    separatorWithPagesLink: {
      paddingBottom: 4,
      paddingTop: 8,
    },
  }),
  stylex.create({
    pagePublishingAuthorizationButton: {
      alignItems: 'center',
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
    pagePublishingAuthorizationContainer: {
      backgroundColor: 'var(--web-wash)',
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    pagePublishingAuthorizationContainerRoot: {
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    pagePublishingAuthorizationContent: {
      display: 'flex',
      flexDirection: 'row',
      paddingBottom: 8,
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
      paddingTop: 16,
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      fontSize: 15,
      fontWeight: 600,
      paddingInlineStart: 12,
    },
  }),
  stylex.create({
    buttonContainer: {
      paddingBottom: 0,
      paddingInline: 16,
      paddingTop: 24,
    },
  }),
  stylex.create({
    list: {
      maxHeight: 'calc(100vh - 240px)',
      paddingBottom: 20,
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 7,
      height: 14,
      marginBottom: 14,
    },
    bodyGlimmerContainer: {
      padding: '20px 20px calc(100vh - 376px) 20px',
    },
    bodyGlimmerFirst: {
      width: '80%',
    },
    bodyGlimmerSecond: {
      width: '40%',
    },
    header: {
      alignItems: 'center',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      textAlign: 'center',
    },
    headerGlimmer: {
      borderRadius: 7,
      height: 14,
      width: 100,
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      flexDirection: 'row',
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
    icon: {
      padding: 8,
    },
    list: {
      paddingBottom: 24,
      paddingTop: 8,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 12,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: 'inherit',
    },
    contentArea: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      minHeight: 'inherit',
      minWidth: 0,
    },
  }),
  stylex.create({
    nowrap: {
      display: 'block',
      overflow: 'hidden',
      textOverflow: 'ellipsis',
      whiteSpace: 'nowrap',
    },
    size13: {
      fontSize: 13,
      lineHeight: 1.3076923076923077,
    },
    size15: {
      fontSize: 15,
      lineHeight: 1.2666666666666666,
    },
    size17: {
      fontSize: 17,
      lineHeight: 1.1764705882352942,
    },
    size20: {
      fontSize: 20,
      lineHeight: 1.2,
    },
    size24: {
      fontSize: 24,
      lineHeight: 1.1666666666666667,
    },
    size28: {
      fontSize: 28,
      lineHeight: 1.1428571428571428,
    },
    size32: {
      fontSize: 32,
      lineHeight: 1.125,
    },
    sizeInherit: {
      fontSize: 'inherit',
      fontWeight: 'inherit',
      lineHeight: 'inherit',
    },
    uppercase: {
      textTransform: 'uppercase',
    },
    useBlueLink: {
      color: 'var(--blue-link)',
    },
    useDisabled: {
      color: 'var(--disabled-text)',
    },
    useHighlight: {
      color: 'var(--accent)',
    },
    useInherit: {
      color: 'inherit',
    },
    useInverse: {
      color: 'var(--primary-text-on-media)',
    },
    useNegative: {
      color: 'var(--negative)',
    },
    usePlaceholder: {
      color: 'var(--placeholder-text)',
    },
    usePositive: {
      color: 'var(--positive)',
    },
    usePrimary: {
      color: 'var(--primary-text)',
    },
    useSecondary: {
      color: 'var(--secondary-text)',
    },
    useSecondaryDark: {
      color: 'var(--section-header-text)',
    },
    useTertiary: {
      color: 'var(--placeholder-text)',
    },
    weightBold: {
      fontWeight: 700,
    },
    weightInherit: {
      fontWeight: 'inherit',
    },
    weightMedium: {
      fontWeight: 500,
    },
    weightNormal: {
      fontWeight: 400,
    },
    weightSemiBold: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    pill: {
      alignItems: 'center',
      backgroundColor: 'var(--primary-button-background)',
      borderStyle: 'none',
      borderRadius: 20,
      boxShadow: '0 8px 20px 0 var(--fds-black-alpha-30), 0 2px 4px 0 var(--fds-black-alpha-10)',
      cursor: 'pointer',
      display: 'flex',
      height: 40,
      justifyContent: 'space-between',
      marginTop: 16,
      padding: '0 16px',
      position: 'absolute',
      start: '50%',
      top: 0,
      transform: 'translateX(-50%)',
      width: 'auto',
    },
  }),
  stylex.create({
    content: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      maxWidth: '100%',
      minHeight: 'inherit',
      width: '100%',
    },
    contentArea: {
      alignItems: 'stretch',
      display: 'flex',
      justifyContent: 'center',
      maxWidth: '100%',
      minHeight: 'inherit',
    },
    contentCentered: {
      justifyContent: 'center',
    },
    fullHeight: {
      height: '100%',
    },
  }),
  stylex.create({
    content: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
    },
    content_DEPRECATED: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      maxWidth: '100%',
    },
    contentArea: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: 'inherit',
      position: 'relative',
    },
  }),
  stylex.create({
    alignContentCenter: {
      alignItems: 'center',
    },
    alignContentStretch: {
      alignItems: 'stretch',
    },
    chatSliver: {
      backgroundColor: 'var(--card-background)',
      boxShadow: '-1px 0 0 var(--divider)',
      display: 'flex',
      end: 0,
      height: '100%',
      position: 'fixed',
      top: 'var(--header-height)',
      width: 80,
    },
    content: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      minWidth: 360,
    },
    contentArea: {
      display: 'flex',
      minHeight: 'inherit',
    },
    contentCentered: {
      justifyContent: 'center',
    },
    contentContainer: {
      display: 'flex',
      flexDirection: 'column',
      maxWidth: '100%',
    },
    rightRail: {
      backgroundColor: 'var(--surface-background)',
      display: 'flex',
      flexShrink: 0,
      minHeight: 'inherit',
      position: 'relative',
    },
    rightRailContainer: {
      backgroundColor: 'var(--surface-background)',
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      maxHeight: 0,
      position: 'fixed',
      width: 'inherit',
      '@media (max-width: 719px)': {
        position: 'sticky',
      },
    },
    rightRailContainerHeight: {
      minHeight: 'calc(100% - var(--header-height))',
      top: 'var(--header-height)',
    },
    rightRailContainerHeightBlue: {
      minHeight: 'calc(100% - 42px)',
      top: 42,
    },
    rightRailContainerPushViewHeader: {
      boxShadow: '0 1px 0 var(--divider)',
      end: 0,
      height: 'var(--header-height)',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    rightRailContainerWithShadow: {
      boxSizing: 'content-box',
      paddingInlineStart: 3,
    },
    rightRailExpanded: {
      width: '50vw',
    },
    rightRailHidden: {
      display: 'none',
    },
    rightRailShadow: {
      backgroundColor: 'var(--web-wash)',
      backgroundImage:
        'url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAYAAAABCAQAAABXwBd7AAAAE0lEQVQI12NgYAZCVgYOBn4GeQAA4ABCt0ka/wAAAABJRU5ErkJggg==)',
      backgroundRepeat: 'repeat-y',
      backgroundSize: '3px 1px',
      bottom: 0,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      width: 3,
    },
  }),
  stylex.create({
    360: {
      width: 360,
    },
    400: {
      width: 400,
    },
    440: {
      width: 440,
    },
    480: {
      width: 480,
    },
    520: {
      width: 520,
    },
  }),
  stylex.create({
    tabBar: {
      zIndex: 2,
    },
  }),
  stylex.create({
    containerResponsive: {
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      minHeight: 'inherit',
      position: 'relative',
      '@media (max-width: 899px)': {
        flexDirection: 'column',
        zIndex: 0,
      },
    },
    contentArea: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      minHeight: 'inherit',
      minWidth: 0,
      position: 'relative',
      zIndex: 0,
    },
    contentAreaWithLeftRailPrimary: {
      '@media (max-width: 899px)': {
        display: 'none',
      },
    },
    leftRailResponsive: {
      display: 'flex',
      flexShrink: 0,
      minHeight: 'inherit',
      overflowAnchor: 'none',
      width: 360,
      zIndex: 1,
    },
    mainContent: {
      display: 'flex',
      flexGrow: 1,
      minHeight: 'inherit',
      '@media (min-width: 900px)': {
        width: 'calc(100% - 360px) !important',
      },
    },
    responsiveHeader: {
      display: 'block',
      zIndex: 1,
    },
  }),
  stylex.create({
    glimmerContainer: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    left: {
      display: 'flex',
      minWidth: 'inherit',
    },
    top: {
      backgroundColor: 'var(--nav-bar-background)',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      display: 'flex',
      paddingBlock: 10,
      zIndex: 1,
    },
    ufiGlimmer: {
      borderRadius: 5,
      height: 10,
      margin: 16,
      width: 120,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexGrow: 1,
      minHeight: 'inherit',
      position: 'relative',
      zIndex: 0,
    },
    containerWithMinWidth: {
      minWidth: 900,
    },
    contentArea: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      minHeight: 'inherit',
      minWidth: 0,
      position: 'relative',
      zIndex: 0,
    },
    contentAreaWithLeftRailPrimary: {
      '@media (max-width: 899px)': {
        display: 'none',
      },
    },
    hideLeftRail: {
      display: 'none',
    },
    leftRail: {
      flexShrink: 0,
      minHeight: 'inherit',
      overflowAnchor: 'none',
      width: 360,
      zIndex: 1,
    },
    leftRailPrimaryResponsive: {
      '@media (max-width: 899px)': {
        width: '100%',
      },
    },
    leftRailSecondaryResponsive: {
      '@media (max-width: 899px)': {
        display: 'none',
      },
    },
    mainContent: {
      display: 'flex',
      minHeight: 'inherit',
    },
  }),
  stylex.create({
    headerOnGemini: {
      marginTop: 60,
    },
    leftRailContainer: {
      backgroundColor: 'var(--surface-background)',
      boxSizing: 'content-box',
      display: 'flex',
      flexDirection: 'column',
      minHeight: 'inherit',
      position: 'relative',
      top: 0,
      width: 360,
      '@media (max-width: 899px)': {
        height: '100vh',
        position: 'sticky',
      },
      '@media (min-width: 900px)': {
        maxHeight: 0,
        position: 'fixed',
      },
    },
    leftRailContainerInBizWeb: {
      backgroundColor: 'var(--surface-background)',
      boxSizing: 'content-box',
      display: 'flex',
      flexDirection: 'column',
      minHeight: 'inherit',
      position: 'relative',
      top: 0,
      width: 360,
    },
    leftRailContainerInDialog: {
      top: 0,
      '@media (max-width: 899px)': {
        minHeight: '100%',
        position: 'relative',
      },
      '@media (min-width: 900px)': {
        minHeight: '100%',
        position: 'relative',
      },
    },
    leftRailContainerPermalink: {
      top: 'var(--header-height)',
      '@media (max-width: 899px)': {
        height: 'calc(100vh - var(--header-height))',
        position: 'sticky',
      },
      '@media (min-width: 900px)': {
        position: 'fixed',
      },
    },
    leftRailContainerPermalinkBlue: {
      top: 42,
      '@media (max-width: 899px)': {
        position: 'sticky',
      },
      '@media (min-width: 900px)': {
        position: 'fixed',
      },
    },
    leftRailContainerPermalinkBlueLoggedOut: {
      top: 0,
      '@media (max-width: 899px)': {
        minHeight: '100%',
        position: 'relative',
      },
      '@media (min-width: 900px)': {
        minHeight: '100%',
        position: 'relative',
      },
    },
    leftRailContainerPushViewHeader: {
      boxShadow: '0 1px 4px var(--shadow-1)',
      flexShrink: 0,
      height: 'var(--header-height)',
    },
    leftRailPrimaryContainer: {
      '@media (max-width: 899px)': {
        height: 'auto',
        position: 'relative',
        top: 0,
        width: '100%',
      },
    },
    primaryNav: {
      marginTop: 8,
    },
    primaryNavExpanding: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
    },
    primaryNavWithSearch: {
      marginBottom: 8,
      marginTop: 4,
    },
    scrollDropShadow: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      marginInline: 16,
    },
    search: {
      marginBottom: 12,
      marginTop: 4,
    },
    stickyNavWithoutHeader: {
      marginTop: 8,
    },
    stickyNavWithoutSearch: {
      marginBottom: 12,
    },
  }),
  stylex.create({
    auxiliary: {
      alignItems: 'flex-end',
      display: 'flex',
      flexBasis: 'calc(100% / 3)',
      flexGrow: 0,
      flexShrink: 0,
      justifyContent: 'center',
      maxHeight: 17,
    },
    auxiliaryFlexBasisAuto: {
      flexBasis: 'auto',
    },
    auxiliaryInner: {
      display: 'flex',
      flexShrink: 0,
    },
    backButton: {
      marginInlineEnd: 20,
    },
    headingRows: {
      margin: '20px 16px 12px',
    },
    headingRowWithGlobalPanel: {
      marginTop: 26,
    },
    meta: {
      marginBottom: 5,
    },
    titleBlock: {
      flexBasis: 'calc(100% * (2 / 3))',
      flexGrow: 1,
    },
    titleBlockFlexBasisAuto: {
      flexBasis: 'auto',
    },
  }),
  stylex.create({
    root: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
    },
  }),
  stylex.create({
    leftRailPrimaryShadow: {
      '@media (max-width: 899px)': {
        display: 'none',
      },
    },
    leftRailShadow: {
      backgroundImage:
        'url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAACBAMAAACapPCZAAAAFVBMVEUAAAAAAAAAAAAAAAAAAAAAAAD29va1cB7UAAAAB3RSTlMCCwQHGBAaZf6MKAAAABJJREFUCNdjSHMVNFZiYGCA0gAUdgIjNiRPgQAAAABJRU5ErkJggg==)',
      backgroundRepeat: 'repeat-y',
      backgroundSize: '7px 1px',
      bottom: 0,
      end: -6,
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
      width: 7,
    },
  }),
  stylex.create({
    row: {
      paddingInline: 16,
    },
    tabs: {
      overflow: 'hidden',
    },
    title: {
      paddingInlineEnd: 16,
    },
  }),
  stylex.create({
    entityHeader: {
      marginBottom: 16,
      marginTop: 16,
    },
    filters: {
      overflow: 'hidden',
    },
    search: {
      marginBottom: 12,
      marginTop: 12,
    },
  }),
  stylex.create({
    backgroundHidden: {
      backgroundColor: 'var(--negative)',
    },
    backgroundVisible: {
      backgroundColor: 'var(--positive)',
    },
    card: {
      height: 200,
      padding: 20,
      width: 200,
    },
    wrapper: {
      marginBottom: 16,
      zIndex: 0,
    },
  }),
  stylex.create({
    appIcon: {
      height: 20,
      width: 20,
    },
    divider: {
      marginInline: 12,
      marginBlock: 4,
    },
    hovercardAppIcon: {
      position: 'relative',
      display: 'flex',
    },
    hovercardChevron: {
      display: 'flex',
      position: 'absolute',
      right: -12,
      top: 2,
      bottom: 0,
      height: 12,
      margin: 'auto',
    },
  }),
  stylex.create({
    badgeCount: {
      position: 'absolute',
      right: -10,
      top: -9,
    },
    badgeDot: {
      position: 'absolute',
      right: -14,
      top: -10,
    },
  }),
  stylex.create({
    addOn: {
      display: 'flex',
      height: 'auto',
      position: 'relative',
    },
    container: {
      width: '100%',
    },
    content: {
      alignItems: 'center',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 0,
      justifyContent: 'center',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'relative',
      width: '100%',
      wordBreak: 'keep-all',
      zIndex: 0,
    },
    largeAddOn: {
      height: 40,
    },
    link: {
      borderRadius: 8,
      boxSizing: 'border-box',
      display: 'flex',
      height: 40,
      width: 42,
    },
    linkHovered: {
      backgroundColor: 'var(--hover-overlay)',
    },
    linkLight: {
      color: 'var(--secondary-text)',
    },
    linkSelected: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
    },
    tooltipContainer: {
      width: '100%',
    },
  }),
  stylex.create({
    card: {
      overflow: 'scroll',
    },
    hovercardSpacing: {
      paddingInline: 16,
    },
    listHeader: {
      paddingInline: 9,
      paddingBlock: 8,
    },
  }),
  stylex.create({
    editorContainer: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--disabled-text)',
      borderRadius: '8px',
      boxSizing: 'border-box',
      padding: '8px',
      position: 'relative',
    },
  }),
  stylex.create({
    tab: {
      paddingBottom: 20,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 20,
    },
  }),
  stylex.create({
    wrapper: {
      position: 'relative',
      right: -6,
    },
  }),
  stylex.create({
    footer: {
      padding: 4,
    },
    pressable: {
      borderRadius: 8,
      color: 'var(--primary-text)',
      display: 'flex',
      justifyContent: 'center',
      padding: 12,
      width: '100%',
    },
  }),
  stylex.create({
    header: {
      display: 'flex',
      justifyContent: 'space-between',
      marginBottom: 12,
    },
  }),
  stylex.create({
    addOnEnd: {
      end: 0,
      height: '100%',
      position: 'absolute',
    },
    addOnEndContent: {
      alignItems: 'center',
      display: 'flex',
      height: '100%',
      paddingInlineEnd: 16,
    },
    content: {
      borderRadius: 8,
      flexGrow: 1,
      flexShrink: 1,
      minWidth: 0,
      paddingInline: 8,
      width: '100%',
    },
    listItem: {
      display: 'flex',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      marginBottom: 8,
    },
  }),
  stylex.create({
    column: {
      paddingInline: 16,
    },
    notFirstItem: {
      marginTop: 16,
    },
  }),
  stylex.create({
    cardPadding: {
      backgroundColor: 'var(--surface-background)',
      height: '90',
    },
    staticMap: {
      height: 167,
    },
  }),
  stylex.create({
    contentRoot: {
      backgroundColor: 'var(--always-black)',
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
    },
  }),
  stylex.create({
    cardContainer: {
      justifyContent: 'center',
      marginInlineEnd: 16,
      marginInlineStart: 16,
      marginTop: 64,
      maxWidth: '560px',
      '@media (max-width: 768px)': {
        marginInlineEnd: 8,
        marginInlineStart: 8,
      },
    },
    container: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    body: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 24,
    },
    button: {
      marginTop: 32,
    },
  }),
  stylex.create({
    body: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 24,
    },
    disclaimer: {
      marginBlock: 32,
    },
    error: {
      marginTop: 32,
    },
    icon: {
      marginTop: 12,
    },
  }),
  stylex.create({
    body: {
      paddingInline: 16,
      paddingBlock: 24,
    },
  }),
  stylex.create({
    container: {
      padding: 16,
    },
    errorContainer: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    illustration: {
      borderRadius: '4px',
    },
  }),
  stylex.create({
    anchor: {
      maxHeight: '476px',
    },
  }),
  stylex.create({
    heading: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    default: {
      paddingInline: 20,
    },
    heading: {
      paddingBottom: 32,
    },
  }),
  stylex.create({
    li: {
      marginInlineStart: '2em',
      marginBlock: 8,
    },
    olList: {
      listStyleType: 'decimal',
      marginTop: 12,
    },
    ulList: {
      listStyleType: 'disc',
      marginTop: 12,
    },
  }),
  stylex.create({
    listItem: {
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    captureReviewWrapper: {
      backgroundColor: 'var(--shadow-2)',
      borderRadius: 4,
      display: 'flex',
      justifyContent: 'center',
      overflow: 'hidden',
      position: 'relative',
    },
    reviewImage: {
      display: 'block',
      margin: 'auto',
      maxWidth: '100%',
    },
  }),
  stylex.create({
    container: {
      position: 'relative',
    },
  }),
  stylex.create({
    preview: {
      borderRadius: '4px',
    },
  }),
  stylex.create({
    error: {
      marginBottom: 12,
    },
    preview: {
      borderRadius: '4px',
    },
  }),
  stylex.create({
    buttons: {
      marginTop: 8,
    },
    container: {
      color: 'var(--primary-text)',
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      marginTop: '16px',
    },
  }),
  stylex.create({
    container: {
      marginTop: 150,
    },
  }),
  stylex.create({
    breadcrumb: {
      padding: '16px 0',
    },
    container: {
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      width: '50vw',
    },
  }),
  stylex.create({
    cardContainer: {
      position: 'relative',
    },
    container: {
      width: 766,
    },
    divider: {
      backgroundColor: 'var(--wash)',
      height: 2,
      width: '100%',
    },
    headerContainer: {
      marginBottom: 16,
    },
    introText: {
      margin: '8px 0 20px 0',
    },
    learnMoreColumnContainer: {
      marginTop: 16,
    },
    sectionSeparation: {
      marginTop: 16,
    },
    settingsButton: {
      end: 12,
      position: 'absolute',
      top: 12,
      width: 106,
    },
    startFlowCard: {
      marginTop: 16,
    },
  }),
  stylex.create({
    container: {
      minWidth: 766,
    },
    content: {
      padding: 16,
    },
    image: {
      height: 232,
    },
  }),
  stylex.create({
    button: {
      marginTop: 16,
    },
    container: {
      padding: 24,
      textAlign: 'center',
    },
    content: {
      marginTop: 16,
    },
  }),
  stylex.create({
    listContainer: {
      margin: '8px 0',
      paddingBottom: 12,
    },
  }),
  stylex.create({
    disclaimerContainer: {
      padding: 16,
    },
    listContainer: {
      margin: '8px, 0',
      paddingBottom: 12,
    },
  }),
  stylex.create({
    button: {
      marginTop: 10,
    },
    container: {
      width: 244,
    },
    content: {
      padding: 12,
    },
    image: {
      borderTopEndRadius: 8,
      borderTopStartRadius: 8,
      height: 80,
    },
  }),
  stylex.create({
    container: {
      maxWidth: 600,
      minWidth: 500,
      paddingBottom: 16,
      paddingTop: 8,
    },
  }),
  stylex.create({
    border: {
      borderColor: 'var(--media-outer-border)',
      borderStyle: 'solid',
      borderWidth: 4,
    },
    container: {
      display: 'flex',
      justifyContent: 'center',
      minHeight: 200,
      width: '100%',
    },
    coverImageContainer: {
      height: 116,
      width: '100%',
    },
    emptyCoverImage: {
      backgroundColor: 'var(--web-wash);',
      height: 116,
    },
    image: {
      margin: -4,
    },
    profileImageContainer: {
      position: 'absolute',
      top: 56,
    },
  }),
  stylex.create({
    container: {
      padding: 16,
    },
    countrySelectionContainer: {
      padding: '16px 16px 0 16px',
    },
    divider: {
      backgroundColor: 'var(--wash)',
      height: 2,
      margin: 'auto',
      width: '95%',
    },
    messageContainer: {
      padding: 16,
    },
    messageContent: {
      padding: 20,
    },
  }),
  stylex.create({
    startFlowCard: {
      padding: 16,
    },
  }),
  stylex.create({
    button: {
      marginTop: 16,
    },
  }),
  stylex.create({
    actionRequiredText: {
      color: 'var(--warning)',
    },
    confirmedText: {
      color: 'var(--positive)',
    },
    inReviewText: {
      color: 'var(--base-blue);',
    },
    notConfirmedText: {
      color: 'var(--negative)',
    },
  }),
  stylex.create({
    bulletList: {
      listStyleType: 'disc',
      padding: '12px 16px',
    },
    disclaimerContainer: {
      padding: 12,
    },
    disclaimerText: {
      marginBottom: 12,
    },
    listItem: {
      margin: '8px 0',
    },
  }),
  stylex.create({
    actionListContainer: {
      marginTop: 12,
    },
    disclaimerContainer: {
      padding: 16,
    },
  }),
  stylex.create({
    children: {
      marginTop: 20,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    container: {
      paddingBottom: 16,
    },
    list: {
      margin: '8px 0',
    },
    notConfirmedText: {
      color: 'var(--negative)',
    },
  }),
  stylex.create({
    container: {
      margin: '8px 0',
      paddingBottom: 12,
    },
  }),
  stylex.create({
    container: {
      padding: '0px 16px 16px 16px',
    },
    description: {
      marginTop: 16,
    },
    header: {
      marginBottom: 12,
    },
  }),
  stylex.create({
    actionListContainer: {
      marginTop: 12,
    },
  }),
  stylex.create({
    columnCard: {
      marginTop: 16,
    },
    heading: {
      paddingBottom: 12,
    },
    singleColumn: {
      maxWidth: '50%',
    },
  }),
  stylex.create({
    listContainer: {
      margin: '12px 0',
      paddingBottom: 32,
    },
  }),
  stylex.create({
    container: {
      padding: '0 16px 16px 16px',
    },
    description: {
      marginTop: 16,
    },
    header: {
      marginBottom: 12,
    },
    separator: {
      marginTop: 16,
    },
  }),
  stylex.create({
    button: {
      marginBottom: 16,
      marginTop: 16,
    },
  }),
  stylex.create({
    appealsCard: {
      marginInline: 16,
      marginTop: 16,
    },
    bodyText: {
      marginBlock: 28,
    },
    cometCardContainer: {
      paddingBottom: 16,
    },
    marginBetweenMessages: {
      marginBottom: 32,
    },
    message: {
      marginBlock: 4,
    },
    messageBox: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 20,
      marginBottom: 4,
      paddingInline: 16,
      paddingBlock: 12,
    },
    timestamp: {
      marginInlineStart: 12,
      marginTop: 8,
    },
  }),
  stylex.create({
    appealsCard: {
      marginTop: 20,
    },
    columnsContainer: {
      marginTop: 12,
    },
    singleColumn: {
      width: '50%',
    },
  }),
  stylex.create({
    cometCardContainer: {
      paddingBottom: 16,
    },
    disclaimerContainer: {
      marginBottom: -16,
      padding: 16,
      paddingBottom: 0,
    },
    listContainer: {
      marginBottom: 16,
      marginTop: 16,
    },
  }),
  stylex.create({
    textContainer: {
      padding: 16,
    },
  }),
  stylex.create({
    container: {
      margin: '12px 0 16px 0',
    },
  }),
  stylex.create({
    statusCardButton: {
      marginTop: 10,
    },
  }),
  stylex.create({
    childrenContainer: {
      padding: 8,
      paddingTop: 0,
    },
    content: {
      padding: 8,
    },
    icon: {
      marginTop: 12,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      paddingTop: 16,
    },
    default: {
      padding: 16,
    },
    disclaimer: {
      marginTop: 16,
      padding: 0,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      paddingTop: 16,
    },
    default: {
      padding: 16,
    },
    iconWrapper: {
      margin: '32px auto',
      textAlign: 'center',
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      paddingTop: 16,
    },
    default: {
      padding: 16,
    },
    disclaimer: {
      marginTop: 16,
      padding: 0,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      paddingTop: 16,
    },
    default: {
      padding: 16,
    },
    iconWrapper: {
      margin: '32px auto',
      textAlign: 'center',
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
    listContainer: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      padding: '38px 16px',
      paddingBottom: 52,
      textAlign: 'center',
    },
    iconWrapper: {
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'center',
      marginBottom: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      paddingBottom: 8,
      paddingTop: 16,
    },
  }),
  stylex.create({
    default: {
      paddingBottom: 8,
      paddingInline: 16,
      paddingTop: 4,
    },
    semibold: {
      fontWeight: 600,
    },
    text: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    default: {
      paddingBottom: 8,
      paddingInline: 16,
      paddingTop: 4,
    },
    text: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    default: {
      padding: '0 16px',
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      paddingInline: 16,
    },
    semibold: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      padding: 16,
    },
    numberedListItem: {
      alignItems: 'center',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'black',
      borderRadius: '100%',
      display: 'flex',
      height: 24,
      justifyContent: 'center',
      marginTop: -3,
      width: 24,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      padding: 16,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 575,
      paddingTop: 16,
    },
    voucherDisabled: {
      opacity: 0.3,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
    },
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
    list: {
      marginTop: 16,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      padding: 16,
    },
  }),
  stylex.create({
    bodyWrapper: {
      maxHeight: 810,
      padding: '6px 0 16px 6px',
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: '16px 16px 0 16px',
    },
    listContainer: {
      marginTop: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
  }),
  stylex.create({
    default: {
      padding: 16,
    },
    footer: {
      display: 'flex',
      justifyContent: 'flex-end',
      paddingBottom: 16,
    },
  }),
  stylex.create({
    headline: {
      padding: 16,
      paddingBottom: 0,
    },
    illustration: {
      paddingBottom: 4,
    },
  }),
  stylex.create({
    container: {
      padding: 16,
    },
  }),
  stylex.create({
    icon: {
      marginTop: 12,
    },
  }),
  stylex.create({
    body: {
      paddingInlineStart: 16,
      paddingBlock: 24,
    },
    headline: {
      paddingBottom: 24,
    },
    icon: {
      marginTop: 8,
    },
  }),
  stylex.create({
    cardContainer: {
      justifyContent: 'center',
      marginInline: 16,
      marginTop: 64,
      maxWidth: '560px',
      '@media (max-width: 768px)': {
        marginInline: 8,
      },
    },
    container: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    container: {
      padding: 16,
    },
  }),
  stylex.create({
    body: {
      marginBottom: 4,
      marginTop: 20,
    },
    text: {
      marginBottom: 20,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-end',
      paddingBottom: 16,
      paddingInline: 16,
    },
    nextButton: {
      paddingInlineStart: 8,
    },
    nextButtonStretched: {
      width: '100%',
    },
    withTopBorder: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--media-inner-border)',
      paddingTop: 16,
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--always-white)',
    },
    dialogContainer: {
      justifyContent: 'start',
    },
  }),
  stylex.create({
    selector: {
      marginTop: 16,
    },
  }),
  stylex.create({
    container: {
      paddingBottom: 20,
      paddingTop: 32,
    },
    disclaimer: {
      marginTop: 12,
    },
    extraMargin: {
      marginBottom: 4,
    },
    icon: {
      marginBottom: 20,
      textAlign: 'center',
    },
  }),
  stylex.create({
    bodyTextOffset: {
      marginBottom: 4,
    },
    container: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 8,
      paddingBottom: 2,
      paddingInline: 16,
      paddingTop: 4,
    },
    secondaryBodyTextOffset: {
      marginTop: 12,
    },
  }),
  stylex.create({
    field: {
      flexGrow: 1,
      marginInlineEnd: 4,
    },
    formFields: {
      display: 'flex',
      flexDirection: 'column',
      marginTop: 12,
    },
    inlineFields: {
      display: 'flex',
      flexDirection: 'row',
      marginInlineEnd: -4,
    },
  }),
  stylex.create({
    field: {
      flexGrow: 1,
      marginInlineEnd: 4,
      width: '100%',
    },
    formFields: {
      display: 'flex',
      flexDirection: 'row',
      marginInlineEnd: -4,
      marginTop: 8,
    },
  }),
  stylex.create({
    field: {
      flexGrow: 1,
      marginInlineEnd: 4,
      width: '50%',
    },
    formFields: {
      display: 'flex',
      flexDirection: 'row',
      marginInlineEnd: -4,
      marginTop: 8,
    },
  }),
  stylex.create({
    input: {
      backgroundColor: 'transparent',
      borderStyle: 'none',
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      fontSize: '1rem !important',
      fontWeight: 'normal',
      lineHeight: 1.25,
      outline: 'none',
      paddingBottom: 10,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 26,
      width: '100%',
    },
    placeholder: {
      borderInlineEndColor: 'var(--divider)',
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 1,
      color: 'var(--disabled-text)',
      fontSize: 15,
      fontWeight: 'normal',
      outline: 'none',
      paddingInline: 16,
      paddingTop: 18,
    },
  }),
  stylex.create({
    helperText: {
      marginBlock: 8,
    },
    textContainer: {
      marginTop: 24,
    },
  }),
  stylex.create({
    textContainer: {
      marginBlock: 26,
    },
  }),
  stylex.create({
    bottomText: {
      marginTop: 20,
    },
    container: {
      paddingBottom: 4,
    },
    errorMessage: {
      paddingBottom: 16,
    },
    hr: {
      paddingBlock: 16,
    },
    inputBox: {
      paddingBottom: 8,
    },
  }),
  stylex.create({
    container: {
      paddingBottom: 4,
    },
    disclaimer: {
      paddingTop: 16,
    },
    errorMessage: {
      paddingBottom: 16,
    },
    hr: {
      paddingBlock: 16,
    },
    inputBox: {
      paddingBottom: 8,
    },
  }),
  stylex.create({
    divider: {
      marginBlock: 20,
    },
  }),
  stylex.create({
    divider: {
      marginBlock: 20,
    },
  }),
  stylex.create({
    divider: {
      marginBlock: 20,
    },
  }),
  stylex.create({
    bodyText: {
      marginInline: 0,
      paddingBottom: 16,
    },
    karmaWarningCard: {
      margin: 0,
      marginBottom: 1,
      marginTop: 20,
    },
    listContainer: {
      margin: '8px -8px',
      paddingBottom: 12,
    },
  }),
  stylex.create({
    optionsWrapper: {
      marginTop: 12,
    },
  }),
  stylex.create({
    notificationTriggerArea: {
      marginBottom: 16,
      marginTop: 12,
    },
  }),
  stylex.create({
    disclaimer: {
      marginTop: 16,
    },
    downloadedSection: {
      marginInlineStart: 36,
      marginTop: -6,
    },
    firstParagraph: {
      marginBottom: 24,
    },
    icon: {
      marginTop: 12,
    },
  }),
  stylex.create({
    rowPadding: {
      padding: 16,
    },
  }),
  stylex.create({
    disclaimer: {
      flexGrow: 1,
      marginBottom: 4,
      marginTop: 12,
    },
    field: {
      flexGrow: 1,
      marginBottom: 8,
    },
    formFields: {
      display: 'flex',
      flexDirection: 'column',
    },
    notice: {
      marginBottom: 16,
    },
    text: {
      marginBottom: 20,
    },
  }),
  stylex.create({
    textContainer: {
      marginBlock: 26,
    },
  }),
  stylex.create({
    textContainer: {
      marginBottom: 8,
      marginTop: 20,
    },
  }),
  stylex.create({
    body: {
      marginInlineEnd: 16,
    },
    disclaimer: {
      marginTop: 24,
    },
  }),
  stylex.create({
    listContainer: {
      marginBottom: 20,
      marginTop: 28,
    },
  }),
  stylex.create({
    listContainer: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    link: {
      paddingBlock: 8,
    },
    listContainer: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    bodyText: {
      paddingBottom: 12,
    },
    listContainer: {
      margin: '8px -8px',
      paddingBottom: 12,
    },
  }),
  stylex.create({
    button: {
      alignItems: 'stretch',
      display: 'flex',
      height: '100%',
    },
    disclaimer: {
      marginBottom: 4,
      marginTop: 20,
    },
    hr: {
      marginBlock: 20,
    },
    label: {
      marginBottom: 16,
    },
    row: {
      padding: 0,
    },
  }),
  stylex.create({
    category: {
      alignItems: 'center',
      backgroundColor: 'var(--divider)',
      borderRadius: 20,
      boxSizing: 'border-box',
      display: 'inline-flex',
      height: 40,
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
    },
    categoryOddIndex: {
      backgroundColor: 'var(--background-deemphasized)',
    },
    categorySelected: {
      backgroundColor: 'var(--accent)',
    },
    categoryShiftBehindPrevious: {
      marginInlineStart: -30,
      paddingInlineStart: 36,
    },
    categoryText: {
      paddingInlineStart: 10,
    },
    categoryTextColor: {
      color: 'var(--toggle-active-text)',
    },
    group: {
      display: 'inline-flex',
      flexDirection: 'row',
      marginInlineEnd: 10,
    },
    list: {
      backgroundColor: 'var(--surface-background)',
    },
    listContent: {
      paddingBottom: 12,
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
      paddingTop: 12,
      whiteSpace: 'nowrap',
      width: 'max-content',
    },
  }),
  stylex.create({
    pressed: {
      transform: 'scale(0.98)',
    },
    root: {
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'space-between',
      position: 'absolute',
      width: '100%',
    },
    scrollableArea: {
      maxHeight: '100%',
    },
    selectedSwatch: {
      boxShadow: 'inset 0 0 0 2px var(--primary-icon), inset 0 0 0 4px var(--card-background)',
    },
    sliderContainer: {
      backgroundColor: 'var(--card-background)',
      boxShadow: '0px -6px 7px -7px var(--comment-background)',
      paddingInline: 22,
      paddingBlock: 16,
    },
    sliderGlimmer: {
      borderRadius: 999,
      height: '100%',
      width: '100%',
    },
    sliderGlimmerBackground: {
      backgroundColor: 'var(--wash)',
      borderRadius: 999,
      height: 40,
      width: '100%',
    },
    swatch: {
      alignItems: 'center',
      borderRadius: '50%',
      display: 'flex',
      justifyContent: 'center',
    },
    swatchContainer: {
      alignContent: 'flex-start',
      display: 'flex',
      flexWrap: 'wrap',
      width: '100%',
    },
  }),
  stylex.create({
    rail: {
      backgroundImage: 'var(--slider-background)',
      borderRadius: 100,
      height: '100%',
      top: '50%',
      transform: 'translateY(-50%)',
    },
    root: {
      height: 40,
    },
    sliderRoot: {
      height: '100%',
      padding: 0,
    },
    thumb: {
      backgroundColor: 'var(--thumb-background-color)',
      borderColor: 'var(--always-white)',
      borderWidth: 6,
      boxShadow: '0px 5px 10px 0px var(--media-inner-border)',
      height: 48,
      top: 4,
      transform: 'translateX(-14px)',
      width: 48,
    },
    track: {
      backgroundColor: 'transparent',
    },
  }),
  stylex.create({
    button: {
      alignItems: 'center',
      backgroundColor: 'var(--card-background)',
      borderRadius: '100%',
      display: 'flex',
      height: 40,
      justifyContent: 'center',
      width: 40,
    },
    icon: {
      alignItems: 'center',
      backgroundImage:
        'linear-gradient(0deg, rgba(255, 255, 255, 0.1), rgba(255, 255, 255, 0.1)), conic-gradient(from 180deg at 50% 50%, #FD4C4C -25.03deg, #F16770 27deg, #F8E968 85.5deg, #82F778 124.03deg, #4BFBFB 180.56deg, #6D7DFF 225.28deg, #FA05FF 273.66deg, #FD4C4C 334.97deg, #F16770 387deg)',
      borderRadius: '100%',
      display: 'flex',
      height: 30,
      justifyContent: 'center',
      width: 30,
    },
    iconInner: {
      backgroundColor: 'var(--card-background)',
      borderRadius: '100%',
      height: 13,
      width: 13,
    },
    pressable: {
      alignItems: 'center',
      borderRadius: '100%',
      boxShadow: '0px 2px 15px -3px var(--media-inner-border)',
      display: 'flex',
      height: 40,
      justifyContent: 'center',
      width: 40,
    },
  }),
  stylex.create({
    category: {
      alignItems: 'center',
      backgroundColor: 'var(--nav-bar-background)',
      borderRadius: 20,
      boxSizing: 'border-box',
      display: 'inline-flex',
      flexDirection: 'row',
      height: 40,
      marginInlineEnd: 10,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    categorySelected: {
      backgroundColor: 'var(--background-deemphasized)',
    },
    categoryText: {
      paddingInlineStart: 4,
    },
    categoryTextColor: {
      color: 'var(--primary-text)',
    },
    listContent: {
      paddingBottom: 12,
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
      paddingTop: 12,
      whiteSpace: 'nowrap',
      width: 'max-content',
    },
  }),
  stylex.create({
    choiceArea: {
      height: '100%',
      overflow: 'hidden',
      position: 'relative',
    },
    footerArea: {
      boxShadow: '0px -6px 7px -7px var(--comment-background)',
      boxSizing: 'border-box',
      padding: 16,
      width: '100%',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
    },
    switchWithLable: {
      alignItems: 'center',
      borderRadius: 12,
      display: 'flex',
      justifyContent: 'space-between',
      margin: 16,
      padding: 10,
    },
    tabGroup: {
      boxShadow: '0px 6px 7px -7px var(--comment-background)',
      padding: 16,
      paddingBottom: 0,
    },
    tabPressable: {
      borderRadius: 12,
      paddingInline: 20,
      paddingBlock: 15,
    },
    tabSelected: {
      backgroundColor: 'var(--hover-overlay)',
    },
  }),
  stylex.create({
    choiceArea: {
      backgroundColor: 'var(--card-background)',
      flexBasis: 1,
      flexGrow: 1,
      flexShrink: 1,
      overflowY: 'hidden',
      position: 'relative',
    },
    choiceScrollShadow: {
      boxShadow: 'inset 0 16px 16px -16px var(--comment-background)',
      end: 0,
      height: 16,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    doneButton: {
      display: 'inline-block',
      end: 0,
      position: 'absolute',
    },
    headerArea: {
      backgroundColor: 'var(--surface-background)',
      borderRadius: '24px 24px 0 0',
      flexGrow: 0,
      flexShrink: 0,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      overflow: 'hidden',
    },
    title: {
      textAlign: 'center',
    },
    titleSection: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      height: 68,
      justifyContent: 'center',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    root: {
      alignContent: 'flex-start',
      boxSizing: 'border-box',
      display: 'flex',
      flexWrap: 'wrap',
      width: '100%',
    },
    rootDesktop: {
      marginBottom: 85,
    },
    scrollableArea: {
      maxHeight: '100%',
    },
  }),
  stylex.create({
    root: {
      alignContent: 'flex-start',
      display: 'flex',
      flexWrap: 'wrap',
      width: '100%',
    },
    rootDesktop: {
      marginBottom: 85,
    },
    scrollableArea: {
      maxHeight: '100%',
    },
  }),
  stylex.create({
    glimmerCell: {
      height: '100%',
      width: '100%',
    },
    gridContainer: {
      alignContent: 'flex-start',
      boxSizing: 'border-box',
      display: 'inline-flex',
      flexWrap: 'wrap',
      maxHeight: '100%',
      overflow: 'hidden',
      width: '100%',
    },
  }),
  stylex.create({
    ColorText: {
      end: '42%',
      position: 'absolute',
      top: '3%',
    },
    DCText: {
      end: '45%',
      position: 'absolute',
      top: '3%',
    },
  }),
  stylex.create({
    errorMessage: {
      paddingBottom: '10px',
      paddingTop: '25px',
    },
    errorMessageDesktop: {
      paddingInlineEnd: '10px',
      paddingInlineStart: '10px',
    },
    errorMessageMobile: {
      paddingInlineEnd: '40px',
      paddingInlineStart: '30px',
    },
    root: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
    },
    rootDesktop: {
      height: '50vh',
    },
  }),
  stylex.create({
    root: {
      alignContent: 'flex-start',
      display: 'flex',
      flexWrap: 'wrap',
      maxHeight: '100%',
      overflow: 'hidden',
    },
  }),
  stylex.create({
    inner: {
      alignItems: 'center',
      backgroundColor: 'var(--accent)',
      borderRadius: '3px',
      color: 'var(--always-white)',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      transform: 'translateX(-100%)',
      transitionDuration: '30s',
      transitionTimingFunction: 'cubic-bezier(0.25, 1, 0.5, 1)',
      width: '100%',
    },
    innerDone: {
      transform: 'translateX(0%)',
      transitionDuration: '0s',
    },
    innerProgressing: {
      transform: 'translateX(-5%)',
    },
    progress: {
      backgroundColor: 'var(--divider)',
      borderRadius: '3px',
      height: '4px',
      margin: '15px 0',
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    centerAligned: {
      display: 'flex',
      flexDirection: 'row',
      margin: 'auto',
      paddingInlineEnd: 10,
      width: 280,
    },
    container: {
      bottom: 0,
      display: 'flex',
      justifyContent: 'flex-end',
      position: 'fixed',
    },
    deleteButton: {
      marginInlineEnd: 20,
    },
    gradient: {
      bottom: 0,
      height: 100,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      width: '100%',
    },
    gradientDark: {
      backgroundImage: 'linear-gradient(360deg, rgba(0, 0, 0, 1) 10%, rgba(0, 0, 0, 0) 100%)',
    },
    gradientLight: {
      backgroundImage:
        'linear-gradient(360deg, rgba(255, 255, 255, 1) 10%, rgba(255, 255, 255, 0) 100%)',
    },
    innerContainer: {
      bottom: 20,
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-end',
      position: 'fixed',
    },
    rightAligned: {
      display: 'flex',
      flexDirection: 'row',
      width: 620,
    },
  }),
  stylex.create({
    positionSpy: {
      height: 1,
      width: '100%',
    },
  }),
  stylex.create({
    centeredViewport: {
      position: 'fixed',
    },
    customizeButtonHorizontalAlign: {
      end: '36%',
      position: 'absolute',
    },
    customizeButtonVerticalAlign: {
      position: 'fixed',
      top: '50%',
      transform: 'translateY(-50%)',
    },
    navigationArea: {
      height: '100%',
      position: 'absolute',
      start: 0,
      width: '34%',
    },
    paletteAreaDynamicConfig: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 16,
      end: 0,
      height: '95%',
      marginTop: 20,
      overflow: 'hidden',
      position: 'absolute',
      width: '35%',
    },
    paletteAreaStaticConfig: {
      end: 0,
      height: '100%',
      position: 'absolute',
      width: '35%',
    },
    previewArea: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
      overflow: 'hidden',
      position: 'absolute',
      width: '100%',
    },
    previewContainer: {
      height: '100%',
      position: 'absolute',
      width: '100%',
    },
    scrollableArea: {
      height: '100%',
    },
    topRightButtonsInner: {
      display: 'flex',
      flexDirection: 'column',
      pointerEvents: 'all',
    },
    topRightButtonsOuter: {
      boxSizing: 'border-box',
      display: 'flex',
      justifyContent: 'flex-end',
      paddingInlineEnd: 20,
      paddingTop: 20,
      pointerEvents: 'none',
      position: 'fixed',
    },
  }),
  stylex.create({
    categoryAndChociesSection: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      overflow: 'hidden',
    },
    categoryArea: {
      flexGrow: 0,
      flexShrink: 0,
    },
    choiceArea: {
      backgroundColor: 'var(--card-background)',
      flexBasis: 1,
      flexGrow: 1,
      flexShrink: 1,
      overflowY: 'hidden',
      position: 'relative',
    },
    choiceScrollShadow: {
      boxShadow: 'inset 0 16px 16px -16px var(--comment-background)',
      end: 0,
      height: 16,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    customizeButton: {
      end: 30,
      marginTop: -50,
      position: 'absolute',
      transform: 'translateX(50%)',
    },
    previewArea: {
      flexBasis: 1,
      flexGrow: 0,
      flexShrink: 0,
      minHeight: '50%',
      overflow: 'hidden',
    },
    viewport: {
      display: 'flex',
      flexDirection: 'column',
      overflow: 'hidden',
      position: 'fixed',
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 60,
      paddingTop: 28,
      '@media (max-width: 999px)': {
        paddingTop: 60,
      },
    },
    displayNoneOnSmallViewport: {
      '@media (max-width: 999px)': {
        display: 'none',
      },
    },
    root: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderInlineEndColor: 'var(--divider)',
      flexBasis: 360,
      flexShrink: 0,
      marginInlineEnd: 0,
      maxHeight: 'calc(100vh - 56px)',
      overflow: 'hidden',
      position: 'sticky',
      top: 'var(--header-height)',
      '@media (max-width: 999px)': {
        alignSelf: 'flex-start',
        flexBasis: 710,
        flexShrink: 1,
        maxHeight: 'auto',
        position: 'static',
      },
    },
    scrollableArea: {
      maxHeight: 'inherit',
      paddingInline: 20,
    },
  }),
  stylex.create({
    fileInput: {
      display: 'none',
    },
    gridRoot: {
      alignContent: 'flex-start',
      display: 'flex',
      flexWrap: 'wrap',
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      bottom: 0,
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      justifyContent: 'center',
      position: 'fixed',
      start: 0,
      top: 'var(--header-height)',
    },
    controlContainer: {
      flexGrow: 0,
      flexShrink: 0,
      height: 'min-content',
      marginBottom: 20,
      paddingTop: 20,
      width: '60%',
    },
  }),
  stylex.create({
    closeButtonOnMobile: {
      marginInlineStart: -10,
    },
    container: {
      alignItems: 'stretch',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      padding: 20,
      paddingTop: 0,
      position: 'fixed',
    },
    header: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'space-between',
      minHeight: 56,
      paddingBottom: 16,
      paddingTop: 10,
    },
    headerDesktop: {},
    headerMobile: {
      marginInlineEnd: -6,
    },
    leftHeader: {
      flexBasis: 0,
      flexGrow: 1,
      paddingInlineEnd: '10px',
    },
    metaLockupOnMobile: {
      marginInlineStart: '3%',
    },
    narrowScreenLogo: {
      marginInlineStart: '18.5%',
    },
    rightHeader: {
      flexBasis: 0,
      flexGrow: 1,
      textAlign: 'end',
    },
  }),
  stylex.create({
    avatarCircle: {
      borderWidth: 3,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: '50%',
      height: 'calc(min(40vh, 90vw))',
      overflow: 'hidden',
      position: 'relative',
      width: 'calc(min(40vh, 90vw))',
    },
    avatarCircleBackground: {
      backgroundColor: 'var(--surface-background)',
      height: '100%',
      opacity: 0.4,
      position: 'absolute',
      width: '100%',
    },
    avatarCircleImage: {
      backgroundPosition: '40% 15%',
      backgroundSize: '140% auto',
      height: '100%',
      position: 'absolute',
      width: '100%',
    },
    controlContainer: {
      flexGrow: 0,
      flexShrink: 0,
      height: 'min-content',
      paddingTop: 11,
    },
    disabled: {
      opacity: 0.7,
    },
    humanicSizeImage: {
      transform: 'scale(1.2)',
    },
    image: {
      alignItems: 'center',
      display: 'flex',
      flexBasis: 56,
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'center',
    },
    imageContainer: {
      height: '100%',
      padding: 20,
      width: '100%',
    },
    imageContainerInner: {
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
      backgroundSize: 'contain',
      height: '100%',
      justifyContent: 'center',
      width: '100%',
    },
    logo: {
      paddingBottom: 8,
    },
  }),
  stylex.create({
    root: {
      alignContent: 'flex-start',
      display: 'flex',
      flexWrap: 'wrap',
      width: '100%',
    },
  }),
  stylex.create({
    buttonContainer: {
      display: 'flex',
      flexDirection: 'column-reverse',
      width: '100%',
    },
    buttonContainerDesktop: {
      flexDirection: 'row',
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
      margin: '0 auto',
      maxWidth: '599px',
      width: '100%',
    },
    control: {
      marginTop: '6px',
    },
    footer: {
      flexBasis: 0,
      flexGrow: 0,
      flexShrink: 0,
      marginBottom: 10,
      width: '100%',
    },
    grid: {
      display: 'flex',
      flexBasis: 0,
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      height: 0,
    },
    header: {
      flexBasis: 0,
      flexGrow: 0,
      flexShrink: 0,
      paddingBottom: 10,
      width: '100%',
    },
    personalizeButtonDesktop: {
      marginInlineStart: 10,
      width: '100%',
    },
    scratchButtonDesktop: {
      marginInlineEnd: 10,
      width: '100%',
    },
  }),
  stylex.create({
    choice: {
      backgroundColor: 'var(--wash)',
      borderRadius: 12,
      overflow: 'hidden',
    },
    glimmer: {
      height: '100%',
      width: '100%',
    },
  }),
  stylex.create({
    choice: {
      backgroundColor: 'var(--divider)',
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
      backgroundSize: 'contain',
      borderRadius: 12,
      boxSizing: 'border-box',
      cursor: 'pointer',
    },
    focused: {
      boxShadow: 'inset 0 0 0 2px var(--primary-button-background)',
    },
    hovered: {
      boxShadow: 'inset 0 0 0 2px var(--hover-overlay)',
    },
    pressed: {
      transform: 'scale(0.98)',
    },
    selectedChoice: {
      boxShadow: 'inset 0 0 0 2px var(--accent), inset 0 0 0 4px var(--background-deemphasized)',
    },
  }),
  stylex.create({
    floor: {
      backgroundImage:
        'radial-gradient(50% 50% at 49.82% 50%, #769DD1 0%, rgba(218, 234, 255, 0) 97.74%)',
      borderRadius: '50%',
      bottom: 0,
      height: 145,
      opacity: 0.6,
      position: 'absolute',
      start: '34%',
      width: '33.33%',
    },
    floorHorizon: {
      start: '35%',
      transform: 'translateY(50%)',
    },
    image: {
      opacity: 1,
      position: 'absolute',
      start: '34%',
      transform: 'scale(1.2)',
      width: '33.33%',
    },
    image2: {
      opacity: 'var(--image2-opacity)',
    },
    imageContainer: {
      height: '575px',
      position: 'relative',
      width: '100%',
    },
    imageHorizon: {
      position: 'absolute',
      start: '35%',
      transform: 'scale(1.5)',
      width: '33.33%',
    },
    rippleEffect: {
      backgroundImage:
        'radial-gradient(38.86% 38.86% at 50% -21.12%, #A1C0EC 0%, rgba(219, 233, 253, 0) 100%)',
      end: '-10%',
      height: '500px',
      position: 'absolute',
      top: '70%',
      width: '120%',
    },
    rippleEffectHorizon: {
      end: '-10%',
      transform: 'translateY(15%)',
    },
  }),
  stylex.create({
    imageContainer: {
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
    viewport: {
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
      backgroundSize: 'contain',
      height: '100%',
      position: 'absolute',
      transitionDuration: '0.5s',
      transitionProperty: 'transform',
      width: '100%',
    },
  }),
  stylex.create({
    closeButton: {
      position: 'absolute',
      start: 5,
      top: 20,
    },
    container: {
      display: 'flex',
      flexDirection: 'row',
      height: '100%',
      overflow: 'hidden',
      width: '100%',
    },
    controlsOnRight: {
      alignItems: 'center',
      display: 'flex',
      end: 30,
      flexDirection: 'column',
      position: 'absolute',
      top: 20,
      transform: 'translateX(50%)',
    },
    doneButton: {
      marginBottom: 10,
    },
    metaLockupOnMobile: {
      position: 'absolute',
      start: 30,
      top: 30,
    },
  }),
  stylex.create({
    choiceArea: {
      backgroundColor: 'var(--card-background)',
      flexBasis: 1,
      flexGrow: 1,
      flexShrink: 1,
      overflowY: 'hidden',
      padding: 20,
      width: '402px',
    },
  }),
  stylex.create({
    sectionMargins: {
      marginInline: 16,
      marginTop: 25,
    },
  }),
  stylex.create({
    headline: {
      marginBottom: '10px',
      marginTop: '10px',
    },
  }),
  stylex.create({
    headline: {
      marginBottom: '10px',
    },
  }),
  stylex.create({
    buttonContainer: {
      display: 'flex',
      flexDirection: 'column-reverse',
      width: '100%',
    },
    buttonContainerDesktop: {
      alignContent: 'center',
      alignItems: 'center',
      bottom: '12px',
      display: 'flex',
      flexDirection: 'column',
      position: 'fixed',
    },
    buttonSizeDesktop: {
      width: '50%',
    },
    control: {
      marginTop: '6px',
    },
    footer: {
      flexBasis: 0,
      flexGrow: 0,
      flexShrink: 0,
      marginBottom: 0,
      width: '100%',
    },
  }),
  stylex.create({
    avatarGridStyles: {
      flexBasis: '65',
    },
    avatarPreviewStyles: {
      flexBasis: '40%',
    },
    container: {
      height: '100vh',
      position: 'fixed',
      width: '100vw',
    },
    desktopContainer: {
      height: '100%',
      paddingInline: '10%',
      paddingTop: '2%',
      position: 'fixed',
      top: '5%',
      width: '80%',
    },
    desktopHeader: {
      paddingBottom: '40px',
    },
    grid: {
      display: 'flex',
      flexBasis: 1,
      flexDirection: 'row',
      flexGrow: 0.92,
      height: 0,
    },
    gridDesktop: {
      flexGrow: 1,
      paddingBottom: 62,
    },
    headerStyling: {
      flexBasis: '80%',
    },
    mobileContainer: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
      margin: '0 auto',
      maxWidth: '599px',
      width: '100%',
    },
    mobileHeader: {
      flexBasis: 0,
      flexGrow: 0,
      flexShrink: 0,
      paddingBottom: '20px',
      width: '100%',
    },
    rippleEffect: {
      backgroundImage:
        'radial-gradient(38.86% 38.86% at 50% -21.12%, #A1C0EC 0%, rgba(219, 233, 253, 0) 100%)',
      end: '-10%',
      height: '500px',
      position: 'absolute',
      top: '60%',
      width: '120%',
    },
  }),
  stylex.create({
    avatarSize: {
      height: '87vh',
      start: '32.19vh',
      top: '0vh',
      width: '57.42vh',
    },
  }),
  stylex.create({
    descriptionForDesktop: {
      alignSelf: 'center',
      bottom: 20,
    },
    descriptionForMobile: {
      paddingBlock: 5,
    },
  }),
  stylex.create({
    descriptionForDesktop: {
      alignSelf: 'center',
      bottom: 20,
    },
    descriptionForMobile: {
      paddingBlock: 5,
    },
    profiles: {
      alignItems: 'flex-start',
      display: 'flex',
      height: '100%',
      justifyContent: 'space-between',
      width: '100%',
    },
    profilesContainer: {
      height: '100%',
      marginBottom: '5px',
      overflow: 'hidden',
      width: '100%',
    },
    selectedAvatarImage: {
      boxSizing: 'border-box',
      padding: 0,
    },
    slider: {
      height: '100%',
      minWidth: 'min-content',
      overflowX: 'hidden',
      overflowY: 'auto',
      padding: 2,
    },
  }),
  stylex.create({
    imageContainerInner: {
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
      backgroundSize: 'contain',
      height: '100%',
      justifyContent: 'center',
      width: '100%',
    },
  }),
  stylex.create({
    avatarPreview: {
      padding: 0,
    },
    avatarPreviewImageContainer: {
      backgroundPosition: '55% 70%',
      backgroundSize: 'auto 140%',
    },
    description: {
      alignItems: 'center',
      alignSelf: 'center',
      bottom: 20,
      display: 'flex',
    },
    descriptionText: {
      paddingInlineStart: '5px',
    },
    profiles: {
      alignItems: 'flex-start',
      display: 'flex',
      height: '100%',
      justifyContent: 'space-between',
      width: '100%',
    },
    profilesContainer: {
      height: '100%',
      marginBottom: '5px',
      overflow: 'hidden',
      width: '100%',
    },
    slider: {
      height: '100%',
      minWidth: 'min-content',
      overflowX: 'hidden',
      overflowY: 'auto',
      padding: 4,
    },
  }),
  stylex.create({
    avatarProfileImage: {
      backgroundColor: 'var(--divider)',
      backgroundPosition: '45% 24%',
      backgroundSize: '330% auto',
      height: '100%',
      position: 'relative',
      width: '100%',
    },
    badge: {
      bottom: 2,
      end: 2,
      height: 18,
      position: 'absolute',
      width: 18,
    },
    badgeBackground: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 50,
      bottom: 0,
      end: 0,
      height: 22,
      position: 'absolute',
      width: 22,
    },
  }),
  stylex.create({
    profileSlider: {
      width: 'calc(min(19vw, 140px))',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'stretch',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      padding: 20,
      paddingTop: 0,
      position: 'fixed',
    },
    header: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'space-between',
      minHeight: 56,
      paddingBottom: 16,
      paddingTop: 10,
    },
    headerMobile: {
      marginInlineEnd: -6,
    },
    leftHeader: {
      flexBasis: 0,
      flexGrow: 1,
      paddingInlineEnd: '10px',
    },
    metaLockupOnMobile: {
      marginInlineStart: '3%',
    },
    rightHeader: {
      flexBasis: 0,
      flexGrow: 1,
      textAlign: 'end',
    },
  }),
  stylex.create({
    imageSelected: {
      margin: -2,
    },
    pressableSize: {
      height: '100%',
      width: '100%',
    },
    selectedInner: {
      borderStyle: 'solid',
      borderColor: 'var(--surface-background)',
      borderRadius: 100,
      borderWidth: 2,
    },
    selectedOuter: {
      borderStyle: 'solid',
      borderColor: 'var(--primary-icon)',
      borderRadius: 100,
      borderWidth: 2,
    },
  }),
  stylex.create({
    grid: {
      borderStyle: 'solid',
      borderColor: 'var(--surface-background)',
      borderRadius: 100,
      borderWidth: 0.5,
    },
    profileSlider: {
      width: 'calc(min(16vw, 130px))',
    },
  }),
  stylex.create({
    buttonLayer: {
      backgroundColor: 'var(--media-hover)',
      height: '100%',
      opacity: 0,
      paddingTop: 12,
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
    buttonLayerVisible: {
      opacity: 1,
    },
  }),
  stylex.create({
    collagePlaceholder: {
      height: 466,
    },
    collageRemoveButton: {
      end: 10,
      top: 12,
    },
    mediaAreaRemoveButton: {
      end: -7,
      top: -7,
    },
    mediaCollageRoot: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      margin: '8px 8px 8px',
      overflow: 'hidden',
      padding: '0px 0px 8px',
      position: 'relative',
    },
    removeButton: {
      position: 'absolute',
      transform: 'scale(0.8)',
    },
  }),
  stylex.create({
    acceptedFileExtensionText: {
      padding: 8,
    },
    attachments: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      minHeight: 254,
    },
    uploadIconButtonContainer: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      height: 254,
      justifyContent: 'center',
    },
  }),
  stylex.create({
    dropContainer: {
      alignItems: 'center',
      backgroundColor: 'var(--overlay-alpha-80)',
      bottom: 0,
      display: 'flex',
      end: 0,
      justifyContent: 'center',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    dropContent: {
      padding: 8,
    },
    root: {
      boxSizing: 'border-box',
      height: '100%',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    root: {
      padding: '0px 0px 8px',
    },
  }),
  stylex.create({
    collageOptimistic: {
      borderWidth: 0,
      borderRadius: 0,
    },
    root: {
      borderRadius: 8,
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    errorMsg: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    card: {
      alignItems: 'stretch',
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      backgroundColor: 'var(--card-background)',
      borderRadius: 8,
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'center',
      minHeight: '72px',
      minWidth: 0,
      overflowX: 'auto',
      paddingInline: 12,
      paddingBlock: 12,
    },
    icon: {
      maxHeight: '32px',
      maxWidth: '32px',
    },
    relativeRoot: {
      alignItems: 'stretch',
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      paddingTop: 0,
      paddingInlineEnd: 0,
      paddingBottom: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      minWidth: 0,
    },
    removeButton: {
      end: '4px',
      position: 'absolute',
      top: '4px',
      zIndex: 1,
    },
  }),
  stylex.create({
    contentContainer: {
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      marginTop: 12,
      minWidth: 1056,
      padding: 20,
      zIndex: 20,
    },
  }),
  stylex.create({
    wrapper: {
      display: 'flex',
      flexDirection: 'column',
      height: 188,
      width: 280,
    },
  }),
  stylex.create({
    reasonSection: {
      marginBottom: 8,
      marginTop: 10,
    },
    smallMarginEnd: {
      marginInlineEnd: 4,
    },
    subtotal: {
      marginInlineEnd: 8,
    },
    wrapper: {
      display: 'flex',
      flexDirection: 'column',
      height: 188,
      width: 300,
    },
    zeroPadding: {
      padding: 0,
    },
  }),
  stylex.create({
    pmContainer: {
      width: 200,
    },
    rightSectionWidth: {
      width: 152,
    },
    txIDWrapper: {
      width: 230,
    },
    wrapper: {
      display: 'flex',
      flexDirection: 'column',
      height: 188,
      width: 442,
    },
  }),
  stylex.create({
    horizontal: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'flex-start',
    },
    icon: {
      marginInlineEnd: 4,
      marginTop: 4,
    },
  }),
  stylex.create({
    disclaimer: {
      marginBottom: 16,
      width: '40%',
    },
    firstcard: {
      marginTop: 16,
      width: '100%',
    },
    otherCard: {
      marginTop: 32,
      width: '100%',
    },
  }),
  stylex.create({
    center: {
      alignSelf: 'center',
    },
    end: {
      alignSelf: 'flex-end',
    },
    start: {
      alignSelf: 'flex-start',
    },
  }),
  stylex.create({
    0: {
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
    },
    8: {
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
    },
    12: {
      paddingInlineEnd: 6,
      paddingInlineStart: 6,
    },
  }),
  stylex.create({
    1: {
      width: '8.33%',
    },
    2: {
      width: '16.66%',
    },
    3: {
      width: '25%',
    },
    4: {
      width: '33.33%',
    },
    5: {
      width: '41.66%',
    },
    6: {
      width: '50%',
    },
    7: {
      width: '58.33%',
    },
    8: {
      width: '66.66%',
    },
    9: {
      width: '75%',
    },
    10: {
      width: '83.33%',
    },
    11: {
      width: '91.66%',
    },
    12: {
      width: '100%',
    },
  }),
  stylex.create({
    0: {
      padding: 0,
    },
    8: {
      padding: 8,
    },
    16: {
      padding: 16,
    },
    '16-notice': {
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 8,
    },
    '8-text': {
      paddingBottom: 5,
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
      paddingTop: 5,
    },
    '8-wide': {
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
    },
  }),
  stylex.create({
    boxSizing: {
      boxSizing: 'border-box',
    },
    expanding: {
      flexGrow: 1,
    },
    flex: {
      display: 'flex',
    },
    noPaddingBottom: {
      paddingBottom: 0,
    },
    noPaddingTop: {
      paddingTop: 0,
    },
    withBackground: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 5,
    },
    withBorder: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 5,
    },
    withScrollBar: {
      height: '150px',
    },
  }),
  stylex.create({
    '10%': {
      transform: 'translate3d(-1px, 0, 0)',
    },
    '20%': {
      transform: 'translate3d(2px, 0, 0)',
    },
    '30%': {
      transform: 'translate3d(-4px, 0, 0)',
    },
    '40%': {
      transform: 'translate3d(4px, 0, 0)',
    },
    '50%': {
      transform: 'translate3d(-4px, 0, 0)',
    },
    '60%': {
      transform: 'translate3d(4px, 0, 0)',
    },
    '70%': {
      transform: 'translate3d(-4px, 0, 0)',
    },
    '80%': {
      transform: 'translate3d(2px, 0, 0)',
    },
    '90%': {
      transform: 'translate3d(-1px, 0, 0)',
    },
  }),
  stylex.create({
    absolutePositionSecondary: {
      bottom: 0,
      display: 'flex',
      end: 0,
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
    },
    cursorPointer: {
      cursor: 'pointer',
    },
    disabled: {
      cursor: 'not-allowed',
    },
    error: {
      borderColor: 'var(--negative)',
      ':active': {
        backgroundColor: 'hsla(var(--negative-h), var(--negative-s), var(--negative-l), 0.05)',
      },
    },
    headerMask: {
      backgroundColor: 'inherit',
      end: 16,
      height: 26,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    helperText: {
      marginTop: 8,
    },
    input: {
      backgroundColor: 'inherit',
      flexGrow: 1,
      maxWidth: '100%',
      position: 'relative',
    },
    inputRow: {
      backgroundColor: 'inherit',
      display: 'flex',
      width: '100%',
    },
    root: {
      backgroundColor: 'var(--card-background)',
      display: 'flex',
      flexDirection: 'column',
      overflow: 'hidden',
      position: 'relative',
      zIndex: 0,
      ':active': {
        backgroundColor: 'hsla(var(--accent-h), var(--accent-s), var(--accent-l), 0.05)',
      },
    },
    secondary: {
      display: 'flex',
    },
    shake: {
      animationDuration: '0.82s',
      animationFillMode: 'both',
      animationName: 'xv0dc80-B',
      animationTimingFunction: 'var(--fds-soft)',
    },
    showBorder: {
      borderRadius: 6,
      borderStyle: 'solid',
      borderWidth: 1,
    },
    validationIcon: {
      paddingInlineEnd: 15,
      paddingTop: 15,
    },
    warn: {
      borderColor: 'var(--warning)',
      ':active': {
        backgroundColor: 'hsla(var(--warning-h), var(--warning-s), var(--warning-l), 0.05)',
      },
    },
  }),
  stylex.create({
    card: {
      position: 'absolute',
      width: '100%',
    },
    cardBackwardAnimation: {
      transform: 'translate(-100%)',
    },
    cardForwardAnimation: {
      transform: 'translate(100%)',
    },
    cardInPlace: {
      transform: 'translate(0)',
    },
    cardWithAnimations: {
      transitionDuration: '400ms',
      transitionProperty: 'opacity, transform',
      transitionTimingFunction: 'var(--fds-soft)',
    },
    heightController: {
      position: 'relative',
      transitionDuration: '400ms',
      transitionProperty: 'height',
      transitionTimingFunction: 'var(--fds-soft)',
    },
  }),
  stylex.keyframes({
    '0%': {
      transform: 'scale(0.98)',
    },
    '100%': {
      transform: 'scale(1)',
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 7,
      height: 14,
      marginBottom: 14,
    },
    bodyGlimmerContainer: {
      padding: '20px 20px 150px 20px',
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
      justifyContent: 'center',
      textAlign: 'center',
    },
    headerGlimmer: {
      borderRadius: 7,
      height: 14,
      width: 100,
    },
    root: {
      animationDuration: 'var(--fds-fast)',
      animationName: 'xitoqud-B',
      animationTimingFunction: 'var(--fds-soft)',
    },
  }),
  stylex.create({
    input: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 6,
      padding: '14px 12px 13px 12px',
    },
  }),
  stylex.create({
    butttonStyle: {
      marginInlineEnd: 16,
      marginInlineStart: 16,
      paddingBottom: 12,
      paddingTop: 16,
    },
    iframe: {
      borderWidth: 0,
      height: '100%',
      overflow: 'hidden',
      width: '100%',
    },
    loading: {
      alignItems: 'center',
      bottom: 0,
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
    marginBottom: {
      marginBottom: 16,
    },
    section: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      padding: 16,
      paddingTop: 200,
    },
    wrapper: {
      borderWidth: 0,
      height: 600,
      width: '100%',
    },
  }),
  stylex.create({
    icon: {
      marginInlineStart: -12,
    },
    selectedFilesRow: {
      alignItems: 'start',
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 12,
    },
    uploadButton: {
      marginInlineEnd: 8,
    },
    uploadFileButtonRow: {
      alignItems: 'center',
      display: 'flex',
      marginBottom: 12,
      marginTop: 5,
      width: '100%',
    },
  }),
  stylex.create({
    body: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      width: '100%',
    },
    card: {
      alignItems: 'center',
      borderColor: 'var(--divider)',
      borderRadius: 8,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      minHeight: 254,
      position: 'relative',
      width: '100%',
    },
    cardContainer: {
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'space-between',
      width: '100%',
    },
  }),
  stylex.create({
    pmPlaceholder: {
      height: 20,
      width: 30,
    },
    tintedBackground: {
      alignItems: 'center',
      borderRadius: '100%',
      display: 'flex',
      justifyContent: 'center',
      position: 'relative',
    },
  }),
  stylex.create({
    'primary-button-background': {
      backgroundColor: 'var(--primary-button-background)',
    },
    'secondary-button-background': {
      backgroundColor: 'var(--secondary-button-background)',
    },
  }),
  stylex.create({
    28: {
      height: 28,
      width: 28,
    },
    36: {
      height: 36,
      width: 36,
    },
    46: {
      height: 46,
      width: 46,
    },
    48: {
      height: 48,
      width: 48,
    },
    60: {
      height: 60,
      width: 60,
    },
  }),
  stylex.create({
    icon: {
      display: 'flex',
      paddingInline: '1.25px',
    },
    icons: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginInline: '-1.25px',
    },
  }),
  stylex.create({
    helperText: {
      paddingTop: 8,
    },
    iconWrapper: {
      margin: 9,
      position: 'absolute',
    },
    iconWrapperLabel: {
      marginTop: 30,
    },
    scroll: {
      boxSizing: 'border-box',
      height: '275px',
      paddingInlineEnd: 16,
      width: '100%',
    },
    searchInput: {
      position: 'relative',
    },
    width: {
      width: '90%',
    },
  }),
  stylex.create({
    icon: {
      display: 'flex',
      paddingInline: '1.25px',
    },
    icons: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginInline: '-1.25px',
    },
  }),
  stylex.create({
    textSpacing: {
      marginInlineStart: 12,
    },
  }),
  stylex.create({
    actionButton: {
      color: 'var(--blue-link)',
      cursor: 'pointer',
      textDecoration: 'none !important',
    },
    borderBottom: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      paddingBottom: 16,
    },
    flex: {
      display: 'flex',
    },
    row: {
      flexGrow: 1,
      minWidth: '100%',
    },
    selectRow: {
      borderRadius: 5,
      boxSizing: 'content-box',
      display: 'flex',
      margin: -8,
      padding: 8,
      width: '100%',
    },
    wrap: {
      flexWrap: 'wrap',
    },
  }),
  stylex.create({
    highlight: {
      backgroundColor: 'var(--new-notification-background)',
    },
    pill: {
      alignItems: 'center',
      borderRadius: 10,
      display: 'flex',
      flexDirection: 'row',
      height: 20,
      justifyContent: 'center',
      marginInlineEnd: 8,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      position: 'relative',
      width: '100%',
    },
    secondary: {
      backgroundColor: 'var(--web-wash)',
    },
  }),
  stylex.create({
    center: {
      alignItems: 'center',
    },
    end: {
      alignItems: 'flex-end',
    },
    start: {
      alignItems: 'flex-start',
    },
  }),
  stylex.create({
    0: {
      marginBottom: 0,
    },
    4: {
      marginBottom: 4,
    },
    8: {
      marginBottom: 8,
    },
    12: {
      marginBottom: 12,
    },
    16: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    0: {
      marginInlineEnd: 0,
      marginInlineStart: 0,
    },
    8: {
      marginInlineEnd: -4,
      marginInlineStart: -4,
    },
    12: {
      marginInlineEnd: -6,
      marginInlineStart: -6,
    },
  }),
  stylex.create({
    center: {
      justifyContent: 'center',
    },
    end: {
      justifyContent: 'flex-end',
    },
    'space-around': {
      justifyContent: 'space-around',
    },
    'space-between': {
      justifyContent: 'space-between',
    },
    'space-evenly': {
      justifyContent: 'space-evenly',
    },
    start: {
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    block: {
      display: 'block',
    },
    inline: {
      display: 'inline',
    },
    textWrapper: {
      marginBottom: 5,
      marginTop: 5,
      whiteSpace: 'pre-wrap',
    },
  }),
  stylex.create({
    1: {
      marginBottom: 7,
      marginTop: 7,
    },
    2: {
      marginBottom: 6,
      marginTop: 6,
    },
  }),
  stylex.create({
    margin4: {
      display: 'flex',
      marginInline: 4,
      verticalAlign: 'middle',
    },
    popover: {
      padding: 10,
      textAlign: 'center',
    },
  }),
  stylex.create({
    anchor: {
      paddingInline: 8,
      paddingBlock: 56,
    },
    card: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 8,
      boxShadow: '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1)',
      maxWidth: 600,
      width: '100%',
    },
  }),
  stylex.create({
    backButton: {
      position: 'absolute',
      start: 16,
      top: 12,
    },
    closeButton: {
      end: 16,
      position: 'absolute',
      top: 12,
    },
    footerButton: {
      paddingBottom: '16px',
    },
    header: {
      alignItems: 'center',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      textAlign: 'center',
    },
    savingIndicator: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-text-on-media)',
      borderRadius: 8,
      bottom: 0,
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      justifyContent: 'center',
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 1,
    },
  }),
  stylex.create({
    errorWrapper: {
      display: 'flex',
      flexGrow: 1,
      justifyContent: 'space-around',
      maxWidth: 1000,
      padding: 16,
    },
  }),
  stylex.create({
    wrapper: {
      display: 'flex',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    address: {
      maxWidth: 340,
    },
    emptyLine: {
      height: 19,
      width: '100%',
    },
    wrapText: {
      marginBottom: 5,
      marginTop: 5,
    },
  }),
  stylex.create({
    icon: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'center',
      paddingInlineEnd: 10,
      position: 'relative',
    },
    iconContainer: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      justifyContent: 'center',
    },
  }),
  stylex.create({
    banner: {
      marginInlineStart: -16,
      marginTop: -12,
    },
  }),
  stylex.create({
    currentBalanceAmount: {
      marginTop: 48,
    },
    currentBalanceCard: {
      minWidth: 200,
    },
    icon: {
      marginBottom: 2,
    },
    statementInfoColumn: {
      height: '100%',
      padding: '12px 0px',
    },
    tintedBackground: {
      alignItems: 'center',
      backgroundColor: 'var(--wash)',
      borderRadius: '100%',
      display: 'flex',
      height: 32,
      justifyContent: 'center',
      position: 'relative',
      width: 32,
    },
  }),
  stylex.create({
    image: {
      alignItems: 'center',
      marginInlineEnd: 6,
      marginInlineStart: 6,
      width: 30,
    },
    row: {
      paddingBottom: 8,
      paddingTop: 8,
    },
    rowNumber: {
      alignItems: 'center',
      marginBottom: 4,
      marginInlineEnd: 8,
      marginTop: 4,
      paddingTop: 10,
      width: 30,
    },
  }),
  stylex.create({
    amount: {
      width: '22%',
    },
    date: {
      width: '22%',
    },
    paymentMethod: {
      width: '34%',
    },
    status: {
      width: '19%',
    },
    statusIcon: {
      padding: 'none',
      top: -2,
      width: '3%',
    },
    textWrapper: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    marginBottom: {
      marginBottom: 8,
    },
    marginTop: {
      marginTop: 3,
    },
  }),
  stylex.create({
    layout: {
      bottom: 44,
      end: 44,
      maxWidth: '100%',
      position: 'fixed',
      width: 350,
      zIndex: 104,
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    headline: {
      minHeight: 36,
    },
    margin: {
      marginBottom: 16,
    },
    popover: {
      padding: 16,
      width: 260,
    },
  }),
  stylex.create({
    padding: {
      marginTop: 2,
      paddingInlineEnd: 24,
    },
  }),
  stylex.create({
    body: {
      borderRadius: 7,
      height: 20,
      width: '80%',
    },
    bottomMargin: {
      height: 36,
    },
    header: {
      borderRadius: 7,
      height: 28,
      width: '66%',
    },
    root: {
      marginBottom: 16,
      width: '100%',
    },
  }),
  stylex.create({
    cardsWrapper: {
      display: 'flex',
      end: 30,
      flexGrow: 1,
      justifyContent: 'space-around',
      maxWidth: 1000,
      padding: 16,
      position: 'relative',
      start: 30,
    },
    helpLink: {
      display: 'flex',
    },
    left: {
      marginInlineEnd: 16,
      width: 640,
    },
    right: {
      width: 360,
    },
  }),
  stylex.create({
    contentContainer: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      width: 245,
    },
    icon: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      borderWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      height: 32,
      justifyContent: 'center',
      marginInlineEnd: 8,
      padding: 8,
      position: 'relative',
    },
    line: {
      backgroundColor: 'var(--secondary-button-background)',
      height: 12,
      margin: '2px 0px',
      width: 1,
    },
    pencil: {
      marginInlineStart: 8,
      marginTop: -5,
    },
    rowContainer: {
      alignItems: 'center',
      backgroundColor: 'var(--web-wash)',
      borderRadius: 5,
      display: 'flex',
      justifyContent: 'space-around',
      padding: 12,
    },
    separator: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
    },
    textContainer: {
      display: 'flex',
      flexDirection: 'column',
    },
    textIconContainer: {
      display: 'flex',
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    end: {
      alignItems: 'flex-end',
    },
    popover: {
      padding: 16,
    },
    rowNumber: {
      alignItems: 'center',
      marginInlineEnd: 8,
      marginTop: 5,
      width: 30,
    },
    textWrap: {
      marginBottom: 4,
      marginTop: 4,
    },
  }),
  stylex.create({
    couponList: {
      paddingTop: 8,
    },
    end: {
      alignItems: 'flex-end',
      paddingTop: 2,
    },
    flex: {
      justifyContent: 'space-between',
    },
    image: {
      marginInlineEnd: 8,
      marginTop: 2,
    },
    textWrap: {
      marginBottom: 5,
      marginInlineEnd: 8,
      marginTop: 5,
    },
  }),
  stylex.create({
    icon: {
      marginInlineEnd: 2,
    },
    label: {
      marginInlineEnd: 4,
    },
    ownerRow: {
      marginTop: 6,
    },
    subRow: {
      marginTop: 2,
    },
  }),
  stylex.create({
    wrapper: {
      marginBottom: 16,
      marginTop: -8,
    },
  }),
  stylex.create({
    wrapper: {
      maxWidth: 350,
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    autopay: {
      paddingInlineEnd: 40,
    },
    bottomMargin: {
      marginBottom: 16,
    },
    end: {
      alignItems: 'flex-end',
    },
    height: {
      height: 36,
    },
    icon: {
      verticalAlign: 'top',
    },
    image: {
      marginInlineEnd: 8,
    },
    primary: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
      borderRadius: 10,
      marginInlineEnd: 8,
      paddingBottom: 5,
      paddingInlineEnd: 10,
      paddingInlineStart: 10,
      paddingTop: 5,
    },
    textWrap: {
      marginBottom: 5,
      marginInlineEnd: 8,
      marginTop: 5,
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    popover: {
      padding: 16,
      width: 300,
    },
  }),
  stylex.create({
    dot: {
      borderRadius: '100%',
      height: 6,
      marginBottom: 2,
      width: 6,
    },
    filler: {
      borderRadius: 'inherit',
      height: '100%',
      maxWidth: '100%',
      minWidth: '0%',
    },
    progressBar: {
      backgroundColor: 'var(--primary-deemphasized-button-pressed)',
      borderRadius: '50px',
      height: '8px',
      marginTop: 8,
      position: 'relative',
      width: 360,
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    bodyContainer: {
      padding: 16,
    },
    bodyGlimmer: {
      borderRadius: 7,
      height: 40,
      margin: 16,
    },
    dialog: {
      maxWidth: '500px',
      width: '100%',
    },
    footerContainer: {
      alignItems: 'flex-end',
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    headerContainer: {
      margin: 16,
    },
  }),
  stylex.create({
    card: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    columnContainer: {
      paddingTop: 8,
    },
    megaphoneContainer: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    bodyText: {
      paddingBottom: 16,
      paddingTop: 12,
    },
    cardContainerBottomMargin: {
      marginBottom: 16,
    },
    cardPadding: {
      paddingBottom: 16,
      paddingInline: 16,
    },
    headerContainer: {
      marginTop: -4,
    },
    iconContainer: {
      display: 'flex',
      justifyContent: 'center',
      paddingTop: 16,
    },
  }),
  stylex.create({
    containerCardBottomMargin: {
      marginBottom: 16,
    },
    containerCardContent: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 16,
      paddingInline: 16,
    },
    containerCardHeader: {
      paddingBottom: 16,
    },
    pressable: {
      borderRadius: 8,
      display: 'flex',
      height: '100%',
      paddingInline: 16,
      paddingBlock: 6,
      width: '100%',
    },
    successStoryBody: {
      paddingBottom: 12,
      paddingTop: 8,
    },
    successStoryBodyContainer: {
      flexGrow: 1,
      paddingInlineStart: 12,
      verticalAlign: 'top',
    },
    successStoryBodyText: {
      paddingTop: '8px',
    },
    successStoryChevronRight: {
      end: 12,
      position: 'absolute',
      top: 8,
    },
    successStoryHeader: {
      marginBottom: 12,
    },
    successStoryImg: {
      borderRadius: 8,
      objectFit: 'cover',
    },
    successStoryImgContainer: {
      display: 'inline-block',
      verticalAlign: 'top',
    },
    successStorySeeAllButton: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 12,
    },
    successStoryText: {
      display: 'inline-block',
      verticalAlign: 'top',
      width: '100%',
    },
  }),
  stylex.create({
    buttonContainer: {
      paddingTop: 20,
    },
    cardContainerBottomMargin: {
      marginBottom: 16,
    },
    cardPadding: {
      paddingBottom: 16,
      paddingInline: 16,
    },
    divider: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      marginBottom: 6,
      marginTop: 6,
    },
    headerPadding: {
      paddingBottom: 8,
    },
    icon: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      display: 'flex',
      height: 36,
      justifyContent: 'center',
      width: 36,
    },
    listBlock: {
      marginTop: 16,
    },
    listItems: {
      listStylePosition: 'outside',
      listStyleType: 'disc',
      marginBottom: 16,
      paddingInlineStart: 20,
    },
    secondaryPadding: {
      paddingBottom: 16,
      paddingInlineStart: 36,
      paddingTop: 8,
    },
  }),
  stylex.create({
    body: {
      marginBottom: 12,
      marginTop: 8,
      paddingBottom: 12,
      paddingInlineStart: 22,
    },
    header: {
      margin: '22px 24px 24px 24px',
    },
    more: {
      margin: '32px 32px 32px 320px',
    },
  }),
  stylex.create({
    bodyText: {
      paddingBottom: 16,
    },
    cardContainerBottomMargin: {
      marginBottom: 16,
    },
    cardPadding: {
      paddingBottom: 16,
      paddingInline: 16,
    },
    headerContainer: {
      marginBottom: 12,
    },
  }),
  stylex.create({
    body: {
      paddingBlock: 16,
    },
    container: {
      margin: 8,
      minWidth: 500,
    },
    date: {
      color: 'var(--primary-text)',
    },
  }),
  stylex.create({
    bodyText: {
      paddingBottom: 16,
      paddingTop: 12,
    },
    cardContainerBottomMargin: {
      marginBottom: 16,
    },
    cardPadding: {
      paddingBottom: 16,
    },
    contentPadding: {
      paddingInline: 16,
    },
    headerContainer: {
      marginTop: -4,
    },
    iconContainer: {
      display: 'flex',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    body: {
      paddingBlock: 16,
    },
    button: {
      paddingTop: 32,
    },
    container: {
      margin: 8,
      minWidth: 500,
    },
    icon: {
      paddingBottom: 12,
      paddingTop: 32,
    },
  }),
  stylex.create({
    bodyText: {
      paddingBottom: 16,
      paddingTop: 12,
    },
    cardContainerBottomMargin: {
      marginBottom: 16,
    },
    cardPadding: {
      paddingBottom: 16,
    },
    contentPadding: {
      paddingInline: 16,
    },
    headerContainer: {
      marginTop: -4,
    },
    imageContainer: {
      backgroundColor: 'var(--fds-spectrum-teal-tint-90)',
      backgroundSize: 'cover',
      height: 216,
    },
  }),
  stylex.create({
    arrowIcon: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      paddingTop: 112,
    },
    containerCardBottomMargin: {
      marginBottom: 16,
    },
    containerCardContent: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 16,
    },
    containerCardHeader: {
      paddingBottom: 16,
    },
    fakeTextCard: {
      marginBottom: 131,
      paddingInline: 31,
      paddingTop: 30,
    },
    seeAllButtonContainer: {
      paddingInline: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    anchor: {
      minHeight: 800,
    },
    rowItem: {
      width: 342,
    },
    scrollableArea: {
      height: 736,
      paddingInline: 8,
    },
  }),
  stylex.create({
    containerCardBottomMargin: {
      marginBottom: 16,
    },
    containerCardContent: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 4,
      paddingInline: 16,
    },
    containerCardHeader: {
      paddingBottom: 16,
    },
    expansionButtonContainer: {
      paddingBottom: 12,
      paddingTop: 8,
    },
  }),
  stylex.create({
    bodyContainer: {
      paddingInline: 16,
    },
    containerCardBottomMargin: {
      marginBottom: 16,
    },
    contentContainer: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 16,
    },
    headerContainer: {
      paddingBottom: 12,
    },
    icon: {
      alignItems: 'center',
      backgroundColor: 'var(--primary-deemphasized-button-background)',
      borderRadius: '50%',
      display: 'flex',
      height: 30,
      justifyContent: 'center',
      width: 30,
    },
  }),
  stylex.create({
    card: {
      display: 'flex',
      flexDirection: 'column',
    },
    root: {
      padding: '8px 0',
    },
  }),
  stylex.create({
    list: {
      paddingInlineStart: 24,
    },
  }),
  stylex.create({
    bottomContent: {
      paddingTop: 24,
    },
    container: {
      alignItems: 'stretch',
      backgroundColor: '#0B9E89',
      display: 'flex',
      justifyContent: 'center',
      width: '100%',
    },
    listMarginLeft: {
      paddingInlineStart: 12,
    },
    listSmallMargin: {
      paddingTop: 12,
    },
  }),
  stylex.create({
    container: {
      alignItems: 'stretch',
      backgroundColor: '#0B9E89',
      display: 'flex',
      justifyContent: 'center',
      width: '100%',
    },
    listMargin: {
      paddingTop: 24,
    },
    listSmallMargin: {
      paddingTop: 12,
    },
  }),
  stylex.create({
    container: {
      alignItems: 'stretch',
      backgroundColor: 'var(--base-grape)',
      display: 'flex',
      justifyContent: 'center',
      width: '100%',
    },
    listMargin: {
      paddingTop: 24,
    },
    mainContent: {
      paddingInlineEnd: 20,
      paddingTop: 24,
    },
    mainContentSmallPadding: {
      paddingInlineStart: 12,
    },
    nextParagraph: {
      paddingTop: 20,
    },
  }),
  stylex.create({
    bulletList: {
      paddingInlineStart: 24,
    },
    bulletListItem: {
      listStyle: 'disc',
    },
    marginTop: {
      marginTop: 12,
    },
    marginTopNone: {
      marginTop: 0,
    },
    subsectionTitle: {
      fontSize: 15,
      fontWeight: 'bold',
      lineHeight: 1.6,
    },
    text: {
      color: 'var(--secondary-text)',
      fontSize: 15,
      lineHeight: 1.6,
    },
  }),
  stylex.create({
    container: {
      width: '100%',
      '@media (max-width: 899px)': {
        padding: 16,
      },
    },
    coverImage: {
      height: '100%',
      width: '100%',
      '@media (max-width: 899px)': {
        height: 242,
        objectFit: 'contain',
        transform: 'translateX(-10%)',
        width: '120%',
      },
    },
    coverImageContainer: {
      marginTop: '-10px',
      overflow: 'hidden',
      '@media (max-width: 899px)': {
        height: 156,
      },
      '@media (max-width: 899px) and (min-width: 520px)': {
        marginTop: 16,
      },
      '@media (min-width: 900px)': {
        borderRadius: 8,
        position: 'absolute',
        zIndex: -1,
      },
    },
    entityHeader: {
      paddingBottom: 16,
    },
    hubImageBackground: {
      backgroundImage: 'linear-gradient(to top, var(--web-wash), var(--primary-button-pressed))',
    },
    textContainer: {
      backgroundColor: 'var(--base-blue)',
      borderBottomEndRadius: '8px',
      borderBottomStartRadius: '8px',
      display: 'flex',
      marginTop: '-10px',
      paddingInlineStart: '-1px',
      width: '100%',
    },
    wrapper: {
      alignItems: 'flex-end',
      display: 'flex',
      justifyContent: 'space-between',
      paddingBottom: '32px',
      paddingTop: '180px',
      width: '100%',
      '@media (max-width: 899px)': {
        alignItems: 'center',
        flexDirection: 'column',
        justifyContent: 'flex-end',
      },
      '@media (min-width: 900px)': {
        paddingInlineEnd: '16px',
        paddingInlineStart: '30px',
      },
    },
  }),
  stylex.create({
    pageNameContainer: {
      maxWidth: 140,
      paddingInline: 8,
    },
    pageSwitcherContainer: {
      backgroundColor: 'var(--progress-ring-on-media-background)',
      borderRadius: 6,
      marginBottom: 24,
      maxWidth: 220,
      '@media (max-width: 899px)': {
        display: 'flex',
        justifyContent: 'center',
      },
    },
    singlePageSwitcherContainer: {
      marginBottom: 8,
      marginInlineStart: -6,
      maxWidth: 220,
      '@media (max-width: 899px)': {
        display: 'flex',
        justifyContent: 'flex-start',
      },
    },
    triangleDownContainer: {
      paddingInlineEnd: 16,
    },
  }),
  stylex.create({
    anchor: {
      minHeight: 600,
    },
    buttonContainer: {
      alignItems: 'flex-end',
      display: 'flex',
      flexDirection: 'column',
    },
    divider: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      marginTop: 10,
    },
    scrollableArea: {
      maxHeight: 215,
      paddingInline: 8,
    },
    subtitleContainer: {
      paddingBottom: 16,
      paddingInline: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    whiteButton: {
      backgroundColor: 'var(--always-white)',
      borderRadius: 6,
    },
  }),
  stylex.create({
    chevronDownIcon: {
      paddingInlineStart: 9,
    },
    pressable: {
      appearance: 'none',
      backgroundColor: 'transparent',
      borderStyle: 'solid',
      borderWidth: 0,
      boxSizing: 'border-box',
      margin: 0,
      padding: 0,
      position: 'relative',
      textAlign: 'inherit',
      zIndex: 0,
    },
    pressableOverlayPressed: {
      backgroundColor: 'var(--non-media-pressed)',
    },
  }),
  stylex.create({
    innerCardContainer: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 4,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 32,
      marginTop: 120,
      position: 'relative',
      width: 500,
    },
    subtitleText: {
      paddingBottom: 28,
      paddingTop: 16,
    },
    topBar: {
      backgroundColor: 'var(--card-background)',
      height: 60,
      position: 'fixed',
      width: '100%',
      zIndex: 1,
    },
  }),
  stylex.create({
    mapContainer: {
      position: 'relative',
    },
    mapContent: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 8,
      boxShadow: '0 1px 2px var(--media-inner-border)',
      maxHeight: 110,
      padding: '8px 16px 8px 0px',
      position: 'absolute',
      start: 20,
      top: 20,
      width: 280,
    },
  }),
  stylex.create({
    buttonContainer: {
      paddingTop: 16,
    },
    icon: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      display: 'flex',
      height: 32,
      justifyContent: 'center',
      width: 32,
    },
    innerCardContainer: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 4,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 32,
      marginTop: 120,
      position: 'relative',
      width: 500,
    },
    subtitleText: {
      paddingBottom: 28,
      paddingTop: 16,
    },
    topBar: {
      backgroundColor: 'var(--card-background)',
      height: 60,
      position: 'fixed',
      width: '100%',
      zIndex: 1,
    },
  }),
  stylex.create({
    button: {
      marginTop: 4,
    },
    content: {
      display: 'flex',
      flexDirection: 'column',
      padding: '12px 16px 16px 16px',
    },
    header: {
      alignItems: 'flex-start',
      display: 'flex',
      justifyContent: 'space-between',
    },
    headerImg: {
      height: 182,
      objectFit: 'cover',
      width: '100%',
    },
    innerCardContainer: {
      height: 343,
    },
    text: {
      padding: '8px 0',
    },
    textContainer: {
      height: 93,
    },
  }),
  stylex.create({
    body: {
      paddingTop: 10,
    },
    closeButton: {
      end: 16,
      position: 'absolute',
      top: 16,
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      padding: 16,
    },
    megaphoneButton: {
      alignContent: 'flex-start',
      display: 'flex',
      justifyContent: 'flex-start',
      minWidth: 150,
    },
    megaphoneContent: {
      paddingInlineEnd: 52,
      paddingInlineStart: 12,
    },
    tipContainer: {
      alignItems: 'flex-start',
      display: 'flex',
    },
  }),
  stylex.create({
    bodyText: {
      paddingTop: '8px',
    },
    divider: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
    },
    pressable: {
      borderRadius: 8,
      display: 'flex',
      height: '100%',
      paddingTop: 4,
      width: '100%',
    },
    tipBody: {
      paddingBottom: 12,
      paddingTop: 8,
    },
    tipBodyContainer: {
      flexGrow: 1,
      paddingInlineStart: 12,
      verticalAlign: 'top',
    },
    tipChevronRight: {
      end: 12,
      position: 'absolute',
      top: 8,
    },
    tipIcon: {
      display: 'inline-block',
      verticalAlign: 'top',
      width: '40',
    },
    tipText: {
      display: 'inline-block',
      verticalAlign: 'top',
      width: '100%',
    },
  }),
  stylex.create({
    completedSection: {
      paddingTop: 16,
    },
    completedSectionTitle: {
      paddingBottom: 16,
      paddingTop: 8,
    },
    contentContainer: {
      paddingInline: 16,
    },
    finishButtonContainer: {
      paddingTop: 16,
    },
    image: {
      height: '105%',
      width: '105%',
    },
    imageContainer: {
      backgroundColor: 'var(--fds-spectrum-teal-tint-90)',
    },
    innerCardContainer: {
      paddingBottom: 16,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 32,
      marginTop: 120,
      position: 'relative',
      width: 500,
    },
    subtitle: {
      paddingBlock: 12,
    },
    topBar: {
      backgroundColor: 'var(--card-background)',
      height: 60,
      position: 'fixed',
      width: '100%',
      zIndex: 1,
    },
  }),
  stylex.create({
    body: {
      display: 'inline-block',
      paddingBottom: 16,
      paddingInlineEnd: 24,
      paddingTop: 8,
      verticalAlign: 'top',
      width: '100%',
    },
    bodyContainer: {
      alignItems: 'flex-start',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
      display: 'flex',
    },
    bodyText: {
      paddingBottom: 16,
      paddingTop: 12,
    },
    icon: {
      float: 'start',
      paddingInline: 12,
    },
    iconBackground: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      display: 'flex',
      height: 32,
      justifyContent: 'center',
      width: 32,
    },
    itemContainer: {
      paddingTop: 12,
    },
    popoverTrigger: {
      float: 'end',
      marginTop: -4,
    },
  }),
  stylex.create({
    popoverContainer: {
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-evenly',
      padding: 8,
      paddingInlineEnd: 20,
      textAlign: 'start',
    },
    truncatedSuggestionTitle: {
      display: 'inline-block',
      verticalAlign: 'bottom',
      width: 100,
    },
  }),
  stylex.create({
    container: {
      width: '100%',
    },
    popover: {
      paddingTop: -8,
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 20,
      paddingInline: 8,
      paddingTop: 4,
    },
    icon: {
      marginInlineStart: 0,
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    header: {
      paddingBottom: 8,
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 20,
      paddingInline: 8,
      paddingTop: 4,
    },
    divider: {
      marginBlock: 16,
    },
    header: {
      alignItems: 'flex-end',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      padding: '4px 16px 6px',
      position: 'relative',
    },
  }),
  stylex.create({
    container: {
      marginBottom: 16,
    },
    divider: {
      marginBlock: 20,
    },
  }),
  stylex.create({
    backgroundCard: {
      paddingInline: 24,
      paddingBlock: 24,
    },
    container: {
      marginBottom: 16,
    },
    root: {
      paddingTop: 24,
      width: '80%',
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    header: {
      alignItems: 'flex-end',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      padding: '4px 16px 16px',
      position: 'relative',
    },
    infoCard: {
      marginTop: 16,
    },
  }),
  stylex.create({
    container: {
      marginBottom: 16,
    },
    root: {
      paddingTop: 24,
      width: 500,
    },
  }),
  stylex.create({
    container: {
      marginBottom: 16,
    },
    root: {
      paddingTop: 24,
      width: 500,
    },
  }),
  stylex.create({
    popoverBulletPoint: {
      paddingInlineEnd: 16,
    },
  }),
  stylex.create({
    row: {
      margin: '20px 20px 0 20px',
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 16,
    },
    header: {
      alignItems: 'flex-end',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      padding: '4px 16px 16px',
      position: 'relative',
    },
  }),
  stylex.create({
    backgroundCard: {
      paddingInline: 24,
      paddingBlock: 24,
    },
    bottomButton: {
      flexBasis: 0,
      flexGrow: 1,
      flexShrink: 1,
    },
    container: {
      marginBottom: 16,
    },
    divider: {
      marginBottom: 16,
    },
    root: {
      paddingTop: 24,
      width: '80%',
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 16,
      paddingInline: 16,
      paddingTop: 4,
    },
    header: {
      alignItems: 'flex-end',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      padding: '4px 16px 16px',
      position: 'relative',
    },
  }),
  stylex.create({
    container: {
      paddingTop: 8,
    },
  }),
  stylex.create({
    buttons: {
      alignItems: 'flex-end',
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      display: 'flex',
      flexDirection: 'column',
      margin: '0px 16px',
      marginBottom: 8,
      paddingBottom: 16,
    },
    content: {
      alignItems: 'stretch',
      display: 'flex',
      flexDirection: 'column',
    },
    disclaimerContainer: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'space-between',
      margin: '0px 16px',
      padding: '24px 0',
    },
    icon: {
      alignItems: 'center',
      backgroundColor: 'var(--fds-spectrum-teal-dark-2)',
      borderRadius: '50%',
      boxShadow: '0px 1px 3px 0px var(--fds-gray-70)',
      display: 'flex',
      flexShrink: 0,
      height: '40px',
      justifyContent: 'center',
      marginInlineEnd: '14px',
      width: '40px',
    },
    tagger: {
      paddingInlineEnd: -8,
    },
  }),
  stylex.create({
    buttons: {
      alignItems: 'flex-end',
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 8,
    },
    content: {
      alignItems: 'stretch',
      display: 'flex',
      flexDirection: 'column',
      padding: 16,
    },
    disclaimerContainer: {
      marginTop: 14,
    },
  }),
  stylex.create({
    sponsoredContentDisclaimer: {
      margin: '12px 16px 16px 16px',
    },
    sponsoredInfo: {
      margin: '0px 0px 12px 0px',
    },
    verificationIcon: {
      marginInlineStart: 5,
      verticalAlign: 'middle',
    },
  }),
  stylex.create({
    listNumber: {
      backgroundColor: 'var(--shadow-8)',
      borderRadius: 20,
      display: 'inline-block',
      height: 24,
      textAlign: 'center',
      width: 24,
    },
    numberOffset: {
      marginTop: 2,
    },
  }),
  stylex.create({
    calloutMaxWidth: {
      maxWidth: 300,
    },
    root: {
      left: 125,
      maxWidth: 300,
      position: 'fixed',
      top: 0,
    },
  }),
  stylex.create({
    root: {
      display: 'block',
      marginInline: 16,
      paddingInline: 12,
      paddingBlock: 12,
    },
  }),
  stylex.create({
    body: {
      borderRadius: 8,
      height: 16,
      marginBlock: 4,
      width: '100%',
    },
    header: {
      borderRadius: 8,
      height: 18,
      marginBlock: 4,
      width: '40%',
    },
    wrapper: {
      paddingInline: 16,
      paddingBlock: 8,
    },
  }),
  stylex.create({
    content: {
      boxSizing: 'border-box',
      maxHeight: 'min(calc(100vh - 113px - (2 * var(--dialog-anchor-vertical-padding))), 587px)',
      overflowX: 'hidden',
      overflowY: 'scroll',
    },
  }),
  stylex.create({
    root: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    keywordBody: {
      paddingInline: 16,
    },
    keywordPile: {
      marginTop: 16,
    },
  }),
  stylex.create({
    pulseWrapper: {
      alignItems: 'center',
      display: 'flex',
      height: 8,
      position: 'absolute',
      start: -8,
    },
  }),
  stylex.create({
    searchWrapper: {
      paddingInline: 16,
      paddingBlock: 8,
    },
  }),
  stylex.create({
    articleListContainer: {
      maxHeight: 'min(calc(100vh - 165px - (2 * var(--dialog-anchor-vertical-padding))), 535px)',
      overflowY: 'scroll',
    },
    header: {
      paddingBottom: 8,
      paddingInline: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    root: {
      display: 'block',
      marginInline: 16,
      paddingInline: 12,
      paddingBlock: 12,
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      height: 560,
      justifyContent: 'center',
      paddingBottom: 60,
    },
    noResultsMessage: {
      marginBlock: 12,
    },
  }),
  stylex.create({
    root: {
      maxWidth: 360,
      padding: 12,
      width: '100%',
    },
  }),
  stylex.create({
    wrapper: {
      margin: 16,
    },
  }),
  stylex.create({
    body: {
      borderRadius: 8,
      height: 16,
      marginBlock: 8,
    },
    header: {
      borderRadius: 8,
      height: 20,
      marginBlock: 16,
      width: '60%',
    },
    long: {
      width: '100%',
    },
    section: {
      marginBlock: 24,
    },
    short: {
      width: '40%',
    },
    title: {
      borderRadius: 8,
      height: 24,
      marginBlock: 4,
      width: '60%',
    },
    wrapper: {
      paddingInline: 16,
      paddingBlock: 20,
    },
  }),
  stylex.create({
    root: {
      boxSizing: 'border-box',
      display: 'flex',
      height: 480,
      justifyContent: 'center',
      paddingBottom: 40,
    },
  }),
  stylex.create({
    container: {
      paddingInline: 16,
      paddingBlock: 20,
    },
    title: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    listContainer: {
      margin: '0px 0px 0px 8px',
      paddingInline: 16,
      paddingBlock: 0,
    },
    ordered: {
      listStyleType: 'decimal',
    },
    root: {
      marginBlock: 16,
    },
    unordered: {
      listStyleType: 'disc',
    },
  }),
  stylex.create({
    withTopMargin: {
      marginTop: 16,
    },
  }),
  stylex.create({
    unused: {
      display: 'flex',
    },
  }),
  stylex.create({
    unused: {
      display: 'flex',
    },
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      backgroundColor: 'var(--overlay-alpha-80)',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    formRoot: {
      paddingBottom: 20,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      position: 'relative',
    },
    hidden: {
      display: 'none',
    },
  }),
  stylex.create({
    helpContent: {
      marginBottom: 12,
      marginInlineEnd: 20,
      marginInlineStart: 20,
      marginTop: 12,
    },
  }),
  stylex.create({
    content: {
      alignItems: 'center',
      display: 'flex',
      height: 260,
      justifyContent: 'center',
    },
    root: {
      paddingBottom: 20,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      position: 'relative',
    },
  }),
  stylex.create({
    submitContainer: {
      minWidth: 130,
    },
  }),
  stylex.create({
    unitHeaderSpacing: {
      paddingTop: 4,
    },
  }),
  stylex.create({
    imagePreview: {
      maxHeight: 80,
      maxWidth: 80,
    },
    item: {
      alignItems: 'center',
      backgroundColor: 'var(--wash)',
      borderRadius: 6,
      display: 'flex',
      height: 80,
      justifyContent: 'center',
      marginInlineEnd: 8,
      overflow: 'hidden',
      position: 'relative',
      width: 80,
    },
    mimeType: {
      color: 'var(--primary-text)',
      fontSize: 10,
      textAlign: 'center',
    },
    removeButton: {
      end: 8,
      height: 16,
      position: 'absolute',
      top: 8,
      width: 16,
    },
    sizeWarning: {
      borderStyle: 'solid',
      borderColor: 'var(--negative)',
    },
  }),
  stylex.create({
    uploadedFiles: {
      display: 'flex',
    },
  }),
  stylex.create({
    highlightedBox: {
      borderColor: 'var(--base-lemon)',
      borderStyle: 'solid',
      borderWidth: 4,
      pointerEvents: 'none',
      position: 'fixed',
    },
    redactedBox: {
      backgroundColor: 'var(--always-black)',
      pointerEvents: 'none',
      position: 'fixed',
    },
    root: {
      bottom: 50,
      display: 'flex',
      justifyContent: 'center',
      position: 'fixed',
      start: 0,
      visibility: 'visible',
      width: '100vw',
    },
    shadow: {
      borderRadius: 6,
      boxShadow: '0 2px 4px var(--shadow-1), 0 12px 28px var(--shadow-2)',
    },
    tooltip: {
      alignItems: 'end',
      display: 'flex',
      height: 75,
      justifyContent: 'center',
      paddingInline: 20,
      position: 'relative',
    },
  }),
  stylex.create({
    grid: {
      marginBottom: 8,
    },
  }),
  stylex.create({
    container: {
      paddingBottom: 20,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      position: 'relative',
    },
    placeholderText: {
      marginInlineStart: 16,
      marginTop: 8,
    },
  }),
  stylex.create({
    container: {
      paddingBottom: '1.25rem',
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      position: 'relative',
    },
    instructions: {
      margin: '0.5rem 1rem',
    },
  }),
  stylex.create({
    cardPadding: {
      padding: '1rem',
    },
  }),
  stylex.create({
    content: {
      margin: '8px 8px',
    },
    primaryButton: {
      minWidth: 130,
    },
    root: {
      paddingBottom: 20,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      position: 'relative',
    },
  }),
  stylex.create({
    buttonWrapper: {
      width: 335,
    },
    root: {
      position: 'fixed',
      start: '50%',
      top: '50%',
      transform: 'translate(-50%, -50%)',
    },
  }),
  stylex.create({
    column: {
      maxWidth: 680,
    },
    rootView: {
      alignItems: 'center',
      marginTop: 30,
      paddingInline: 0,
    },
  }),
  stylex.create({
    auxButtonWrapper: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineEnd: 12,
    },
    pressable: {
      borderRadius: 8,
      padding: 16,
    },
  }),
  stylex.create({
    strikeThrough: {
      textDecoration: 'line-through',
    },
  }),
  stylex.create({
    disabledOffset: {
      marginInlineStart: 20,
      textDecoration: 'line-through',
    },
    enabledOffset: {
      marginInlineStart: 28,
    },
  }),
  stylex.create({
    container: {
      maxHeight: 'calc(100vh - var(--header-height) - 70px)',
    },
    rootView: {
      maxWidth: 680,
      paddingBottom: 20,
      paddingTop: 64,
    },
    scrollableAreaInner: {
      marginBlock: 8,
    },
  }),
  stylex.create({
    image: {
      display: 'block',
      maxWidth: '100%',
      minHeight: '100%',
    },
  }),
  stylex.create({
    root: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    root: {
      maxWidth: 680,
      paddingBottom: 20,
      paddingTop: 60,
    },
  }),
  stylex.create({
    bottom: {
      paddingBottom: 20,
    },
    top: {
      paddingTop: 48,
    },
  }),
  stylex.create({
    bottom: {
      paddingBottom: 20,
    },
    container: {
      maxHeight: 'calc(100vh - var(--header-height) - 70px)',
    },
    scrollableAreaInner: {
      marginBlock: 8,
    },
  }),
  stylex.create({
    radio: {
      marginTop: '4px',
    },
  }),
  stylex.create({
    size: {
      margin: '0 auto',
      maxWidth: '100%',
      paddingTop: '24px',
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      justifyContent: 'center',
      maxHeight: 400,
      width: '100%',
    },
  }),
  stylex.create({
    engagementSection: {
      marginTop: 16,
    },
  }),
  stylex.create({
    root: {
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      maxWidth: 700,
      padding: '0px 16px',
    },
  }),
  stylex.create({
    image: {
      display: 'block',
      maxWidth: '100%',
      minHeight: '100%',
      objectFit: 'cover',
    },
  }),
  stylex.create({
    subscribeTitle: {
      letterSpacing: 0.36,
      marginBottom: '8px',
    },
  }),
  stylex.create({
    expand: {
      flexBasis: '0%',
      flexGrow: 1,
      flexShrink: 1,
    },
    fallbackImgContainer: {
      alignItems: 'center',
      display: 'flex',
      height: 64,
      justifyContent: 'center',
      overflow: 'hidden',
      width: 64,
    },
    transparentColor: {
      minHeight: 12,
    },
  }),
  stylex.create({
    dotSeperatorContainer: {
      marginInline: 4,
    },
    explicitIndication: {
      alignItems: 'center',
      backgroundColor: 'var(--always-gray-40)',
      borderRadius: '50%',
      display: 'flex',
      height: 16,
      justifyContent: 'center',
      width: 16,
    },
    explicitIndicationContainer: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginInlineEnd: 6,
    },
    explicitIndicationText: {
      userSelect: 'none',
    },
    overflowBlur: {
      backgroundColor: 'var(--card-background)',
      filter: 'blur(5px)',
      width: '5%',
    },
    rowItem: {
      display: 'flex',
      flexBasis: '0%',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      overflow: 'hidden',
      padding: 0,
    },
    scrollContainer: {
      display: 'flex',
      flexBasis: '0%',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      overflow: 'hidden',
    },
    subtext: {
      overflowX: 'visible',
    },
    subtextContainer: {
      backgroundColor: 'var(--card-background)',
      width: '95%',
    },
    subtextOverflowContainer: {
      alignItems: 'center',
      display: 'flex',
      minWidth: '100%',
      paddingBlock: 5,
      position: 'relative',
      width: 'fit-content',
    },
  }),
  stylex.create({
    container: {
      height: 2,
      paddingInline: 4,
      width: '100%',
    },
    progressBar: {
      backgroundColor: 'var(--fb-logo-color)',
      height: 2,
      marginTop: 5,
      position: 'absolute',
    },
    progressBarContainer: {
      cursor: 'pointer',
      height: 10,
      marginTop: -5,
      width: '100%',
    },
    progressBarContainerIndented: {
      start: 60,
      width: 'calc(100% - 60px)',
    },
    progressBarFilled: {
      backgroundColor: 'var(--disabled-icon)',
      height: 2,
      marginTop: 5,
      position: 'absolute',
      width: '100%',
    },
    progressTransition: {
      transition: 'all 250ms linear',
    },
    removePadding: {
      padding: 0,
    },
    scrubberHead: {
      backgroundColor: 'var(--always-white)',
      borderRadius: '50%',
      boxShadow: '0px 0px 8px var(--shadow-2)',
      cursor: 'pointer',
      height: 12,
      marginInlineStart: -5,
      position: 'absolute',
      width: 12,
    },
    unselectable: {
      '-webkit-tap-highlight-color': 'transparent',
      userSelect: 'none',
    },
  }),
  stylex.create({
    playerContainer: {
      width: '100%',
    },
    playerRow: {
      width: '100%',
    },
    progressBar: {
      marginTop: '-2px',
    },
  }),
  stylex.create({
    buttonContainer: {
      padding: 3,
    },
    hideInDesktopView: {
      '@media screen and (min-width: 501px)': {
        display: 'none',
      },
    },
    hideInMobileView: {
      '@media screen and (max-width: 500px)': {
        display: 'none',
      },
    },
    removePadding: {
      padding: 0,
    },
    root: {
      paddingInlineEnd: 12,
      paddingInlineStart: 0,
    },
  }),
  stylex.create({
    topText: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    bottomText: {
      marginTop: 16,
    },
  }),
  stylex.create({
    headline1Body: {
      marginTop: 20,
    },
    headline2Body: {
      marginTop: 20,
    },
    headline2Meta: {
      marginTop: 16,
    },
    headline3Body: {
      marginTop: 12,
    },
    headline3Meta: {
      marginTop: 12,
    },
    headline4Body: {
      marginTop: 8,
    },
    secondaryLabelMeta: {
      marginTop: 8,
    },
  }),
  stylex.create({
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabledText: {
      color: 'var(--disabled-text)',
    },
    highlight: {
      color: 'var(--accent)',
    },
    negative: {
      color: 'var(--negative)',
    },
    placeholderText: {
      color: 'var(--placeholder-text)',
    },
    placeholderTextOnMedia: {
      color: 'var(--placeholder-text-on-media)',
    },
    primaryText: {
      color: 'var(--primary-text)',
    },
    primaryTextinMediaManager: {
      color: '#1c2b33',
    },
    primaryTextOnMedia: {
      color: 'var(--primary-text-on-media)',
    },
    secondaryText: {
      color: 'var(--secondary-text)',
    },
    secondaryTextOnMedia: {
      color: 'var(--secondary-text-on-media)',
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily:
        "system-ui, -apple-system, BlinkMacSystemFont, '.SFNSText-Regular', sans-serif !important",
    },
    default: {
      fontFamily: 'Helvetica, Arial, sans-serif !important',
    },
    segoe: {
      fontFamily: 'Segoe UI Historic, Segoe UI, Helvetica, Arial, sans-serif !important',
    },
  }),
  stylex.create({
    body1: {
      fontFamily: 'Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 300,
      letterSpacing: 0.32,
    },
    body2: {
      fontSize: 20,
      fontWeight: 400,
      letterSpacing: 0.38,
    },
    body3: {
      fontSize: 17,
      fontWeight: 400,
      letterSpacing: 0.38,
    },
    body3Emphasized: {
      fontSize: 17,
      fontWeight: 600,
      letterSpacing: 0.38,
    },
    body4: {
      fontSize: 15,
      fontWeight: 400,
      letterSpacing: -0.23,
    },
    body4Emphasized: {
      fontSize: 15,
      fontWeight: 600,
      letterSpacing: -0.23,
    },
    body5: {
      fontFamily: 'Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 15,
      fontWeight: 300,
      letterSpacing: 0.38,
    },
    headline1: {
      fontFamily: 'Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 28,
      fontWeight: 700,
      letterSpacing: 0.36,
    },
    headline2: {
      fontFamily: 'Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 700,
      letterSpacing: 0.32,
    },
    headline3: {
      fontFamily: 'Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 20,
      fontWeight: 700,
      letterSpacing: 0.28,
    },
    headline4: {
      fontFamily: 'Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 700,
      letterSpacing: 0.24,
    },
    homepageHeadline1: {
      fontFamily: 'Optimistic Display Medium, system-ui, sans-serif !important',
      fontSize: 48,
      fontWeight: 400,
      letterSpacing: 0.5,
    },
    homepageHeadline2: {
      fontFamily: 'Optimistic Display Medium, system-ui, sans-serif !important',
      fontSize: 32,
      fontWeight: 400,
      letterSpacing: 0.5,
    },
    meta: {
      fontSize: 13,
      fontWeight: 400,
      letterSpacing: -0.08,
    },
    meta2: {
      fontFamily: 'Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 13,
      fontWeight: 400,
      letterSpacing: -0.08,
    },
    primaryLabel: {
      fontFamily: 'Optimistic Display Medium, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 500,
      letterSpacing: 0.12,
    },
    secondaryLabel: {
      fontSize: 15,
      fontWeight: 500,
      letterSpacing: -0.23,
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily:
        "Karla, system-ui, -apple-system, BlinkMacSystemFont, '.SFNSText-Regular', sans-serif !important",
    },
    default: {
      fontFamily: 'Karla, Helvetica, Arial, sans-serif !important',
    },
    segoe: {
      fontFamily: 'Karla, Segoe UI Historic, Segoe UI, Helvetica, Arial, sans-serif !important',
    },
  }),
  stylex.create({
    body1: {
      fontFamily: 'Rubik, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 400,
    },
    body2: {
      fontSize: 20,
      fontWeight: 400,
      letterSpacing: -0.08,
    },
    body3: {
      fontSize: 18,
      fontWeight: 400,
      letterSpacing: -0.08,
    },
    body3Emphasized: {
      fontSize: 18,
      fontWeight: 600,
      letterSpacing: -0.08,
    },
    body4: {
      fontSize: 16,
      fontWeight: 400,
      letterSpacing: -0.24,
    },
    body4Emphasized: {
      fontSize: 16,
      fontWeight: 600,
      letterSpacing: -0.24,
    },
    body5: {
      fontFamily: 'Rubik, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 15,
      fontWeight: 300,
      letterSpacing: 0,
    },
    headline1: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 28,
      fontWeight: 600,
    },
    headline2: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 600,
    },
    headline3: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 20,
      fontWeight: 600,
    },
    headline4: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 600,
    },
    homepageHeadline1: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 48,
      fontWeight: 400,
      letterSpacing: 0.5,
    },
    homepageHeadline2: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 32,
      fontWeight: 400,
      letterSpacing: 0.5,
    },
    meta: {
      fontSize: 14,
      fontWeight: 400,
    },
    meta2: {
      fontFamily: 'Rubik, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 13,
      fontWeight: 300,
    },
    primaryLabel: {
      fontFamily: 'Rubik, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 18,
      fontWeight: 500,
    },
    secondaryLabel: {
      fontSize: 16,
      fontWeight: 500,
      letterSpacing: -0.24,
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily:
        "Merriweather, system-ui, -apple-system, BlinkMacSystemFont, '.SFNSText-Regular', sans-serif !important",
    },
    default: {
      fontFamily: 'Merriweather, Helvetica, Arial, sans-serif !important',
    },
    segoe: {
      fontFamily:
        'Merriweather, Segoe UI Historic, Segoe UI, Helvetica, Arial, sans-serif !important',
    },
  }),
  stylex.create({
    body1: {
      fontFamily: 'Lora, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 500,
    },
    body2: {
      fontSize: 18,
      fontWeight: 400,
    },
    body3: {
      fontSize: 16,
      fontWeight: 400,
    },
    body3Emphasized: {
      fontSize: 16,
      fontWeight: 700,
    },
    body4: {
      fontSize: 14,
      fontWeight: 400,
    },
    body4Emphasized: {
      fontSize: 14,
      fontWeight: 700,
    },
    body5: {
      fontFamily: 'Lora, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 15,
      fontWeight: 500,
    },
    headline1: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 28,
      fontWeight: 700,
    },
    headline2: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 700,
    },
    headline3: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 20,
      fontWeight: 700,
    },
    headline4: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 700,
    },
    homepageHeadline1: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 48,
      fontWeight: 500,
    },
    homepageHeadline2: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 32,
      fontWeight: 500,
    },
    meta: {
      fontSize: 12,
      fontWeight: 400,
    },
    meta2: {
      fontFamily: 'Lora, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 13,
      fontWeight: 500,
    },
    primaryLabel: {
      fontFamily: 'Lora, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 600,
    },
    secondaryLabel: {
      fontSize: 14,
      fontWeight: 700,
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily:
        "Bitter, system-ui, -apple-system, BlinkMacSystemFont, '.SFNSText-Regular', sans-serif !important",
    },
    default: {
      fontFamily: 'Bitter, Helvetica, Arial, sans-serif !important',
    },
    segoe: {
      fontFamily: 'Bitter, Segoe UI Historic, Segoe UI, Helvetica, Arial, sans-serif !important',
    },
  }),
  stylex.create({
    body1: {
      fontFamily: 'Bitter, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 400,
    },
    body2: {
      fontSize: 20,
      fontWeight: 400,
    },
    body3: {
      fontSize: 17,
      fontWeight: 400,
    },
    body3Emphasized: {
      fontSize: 17,
      fontWeight: 600,
    },
    body4: {
      fontSize: 15,
      fontWeight: 400,
      letterSpacing: 0.24,
    },
    body4Emphasized: {
      fontSize: 15,
      fontWeight: 600,
      letterSpacing: 0.24,
    },
    body5: {
      fontFamily: 'Bitter, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 15,
      fontWeight: 400,
      letterSpacing: 0.24,
    },
    headline1: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 28,
      fontWeight: 700,
    },
    headline2: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 700,
    },
    headline3: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 20,
      fontWeight: 700,
    },
    headline4: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 700,
    },
    homepageHeadline1: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 48,
      fontWeight: 500,
    },
    homepageHeadline2: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 32,
      fontWeight: 500,
    },
    meta: {
      fontSize: 13,
      fontWeight: 400,
    },
    meta2: {
      fontFamily: 'Bitter, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 13,
      fontWeight: 400,
    },
    primaryLabel: {
      fontFamily: 'Bitter, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 17,
      fontWeight: 600,
    },
    secondaryLabel: {
      fontSize: 15,
      fontWeight: 600,
      letterSpacing: 0.24,
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily:
        "Montserrat, system-ui, -apple-system, BlinkMacSystemFont, '.SFNSText-Regular', sans-serif !important",
    },
    default: {
      fontFamily: 'Montserrat, Helvetica, Arial, sans-serif !important',
    },
    segoe: {
      fontFamily:
        'Montserrat, Segoe UI Historic, Segoe UI, Helvetica, Arial, sans-serif !important',
    },
  }),
  stylex.create({
    body1: {
      fontFamily: 'Montserrat, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 300,
    },
    body2: {
      fontSize: 18,
      fontWeight: 400,
    },
    body3: {
      fontSize: 16,
      fontWeight: 400,
    },
    body3Emphasized: {
      fontSize: 16,
      fontWeight: 500,
    },
    body4: {
      fontSize: 14,
      fontWeight: 400,
    },
    body4Emphasized: {
      fontSize: 14,
      fontWeight: 500,
    },
    body5: {
      fontFamily: 'Montserrat, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 14,
      fontWeight: 300,
    },
    headline1: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 28,
      fontWeight: 600,
      letterSpacing: -0.32,
    },
    headline2: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 24,
      fontWeight: 600,
      letterSpacing: -0.16,
    },
    headline3: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 20,
      fontWeight: 600,
      letterSpacing: -0.08,
    },
    headline4: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 16,
      fontWeight: 600,
      letterSpacing: -0.08,
    },
    homepageHeadline1: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 46,
      fontWeight: 400,
    },
    homepageHeadline2: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 30,
      fontWeight: 400,
    },
    meta: {
      fontSize: 13,
      fontWeight: 400,
    },
    meta2: {
      fontFamily: 'Montserrat, Optimistic Display Light, system-ui, sans-serif !important',
      fontSize: 13,
      fontWeight: 400,
    },
    primaryLabel: {
      fontFamily: 'Montserrat, Optimistic Display Bold, system-ui, sans-serif !important',
      fontSize: 16,
      fontWeight: 500,
    },
    secondaryLabel: {
      fontSize: 14,
      fontWeight: 500,
    },
  }),
  stylex.create({
    root: {
      bottom: 0,
      end: 0,
      height: '100%',
      position: 'fixed',
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      marginInlineEnd: 16,
      marginTop: 8,
    },
    listContainer: {
      marginInlineEnd: 32,
      marginTop: 4,
    },
  }),
  stylex.create({
    bold: {
      fontWeight: 'bold',
    },
    footnote: {
      color: 'var(--blue-link)',
      fontSize: 12,
      ':hover': {
        textDecoration: 'underline',
      },
    },
    italic: {
      fontStyle: 'italic',
    },
    link: {
      color: 'var(--blue-link)',
      ':hover': {
        textDecoration: 'underline',
      },
    },
    strikethrough: {
      textDecoration: 'line-through',
    },
    underline: {
      textDecoration: 'underline',
    },
    underlineStrikethrough: {
      textDecoration: 'underline line-through',
    },
  }),
  stylex.create({
    circle: {
      backgroundColor: 'var(--primary-text)',
      borderRadius: '50%',
      height: 4,
      marginInlineEnd: 25,
      width: 4,
    },
    lastCircle: {
      marginInlineEnd: 0,
    },
    root: {
      display: 'flex',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    fontSize: {
      fontSize: '20px',
      lineHeight: 1.5,
    },
    headlineWrapper: {
      marginBlock: 24,
    },
    listLineHeight: {
      lineHeight: 1.34,
    },
    listWrapper: {
      marginBlock: 20,
      paddingInlineStart: 40,
    },
    richTextBlockquote: {
      whiteSpace: 'pre-wrap',
    },
    richTextBlockquoteWrapper: {
      borderStartColor: 'var(--secondary-button-background)',
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 4,
      fontStyle: 'italic',
      marginInline: 0,
      marginBlock: 4,
      paddingInline: 20,
    },
    richTextMargin: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      justifyContent: 'center',
      position: 'relative',
    },
    innerContainer: {
      display: 'flex',
      minHeight: 192,
    },
    maxTheEnforcerOfHeight: {
      display: 'flex',
      flexGrow: 1,
      position: 'relative',
    },
    noCoverContainer: {
      paddingBottom: 24,
      paddingTop: 24,
    },
    noCoverInnerContainer: {
      minHeight: 10,
    },
    noCoverInnerContainerMobile: {
      paddingInlineStart: 16,
    },
    sizeSelect: {
      position: 'absolute',
      start: 16,
      top: 16,
    },
    sizeSelectMobile: {
      position: 'absolute',
    },
    uploadInProgress: {
      alignItems: 'center',
      backgroundColor: 'var(--card-background-flat)',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
    },
  }),
  stylex.create({
    root: {
      boxSizing: 'border-box',
      marginInline: 'auto',
      marginTop: 16,
      maxWidth: 732,
      paddingInline: 16,
      width: '100%',
    },
  }),
  stylex.create({
    subtitleEditor: {
      marginBottom: 14,
    },
  }),
  stylex.create({
    titleEditor: {
      marginBottom: 4,
    },
  }),
  stylex.create({
    fullWidthSection: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      borderTopColor: 'var(--divider)',
      marginTop: 16,
    },
    uploadError: {
      color: 'var(--negative)',
      marginBottom: 8,
    },
  }),
  stylex.create({
    editButton: {
      end: -75,
      position: 'absolute',
      top: 22,
      '@media (max-width: 1100px)': {
        display: 'none',
      },
    },
    root: {
      position: 'relative',
    },
  }),
  stylex.create({
    progressIndicator: {
      marginTop: 14,
    },
    uploadWrapper: {
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      paddingTop: 0,
      paddingInlineEnd: 0,
      paddingBottom: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      alignItems: 'center',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    margin: {
      marginBottom: '12px',
    },
    root: {
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      paddingTop: 0,
      paddingInlineEnd: 0,
      paddingBottom: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      alignItems: 'center',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    placeholder: {
      '::placeholder': {
        color: 'var(--secondary-text)',
      },
    },
  }),
  stylex.create({
    placeholder: {
      '::placeholder': {
        color: 'var(--secondary-text)',
      },
    },
  }),
  stylex.create({
    aspectRatioWrapper: {
      paddingTop: '25%',
      position: 'relative',
    },
    centerWrapper: {
      alignItems: 'center',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'relative',
      width: '100%',
      zIndex: 0,
    },
    containerWidth: {
      maxWidth: 500,
      width: '100%',
    },
    placeholder: {
      alignItems: 'center',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--fds-gray-30)',
      boxSizing: 'border-box',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      padding: '0px 24px',
      position: 'absolute',
      top: 0,
      width: '100%',
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 0,
    },
    '100%': {
      opacity: 1,
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 1,
    },
    '100%': {
      opacity: 0,
    },
  }),
  stylex.create({
    bottom: {
      alignItems: 'center',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      height: 40,
      justifyContent: 'space-between',
      marginBottom: 20,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 20,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'relative',
      zIndex: 0,
    },
    buttonWidth: {
      width: 48,
    },
    fadeIn: {
      animationDuration: '400ms',
      animationName: 'x33l7jf-B',
      opacity: 1,
    },
    fadeOut: {
      animationDuration: '400ms',
      animationName: 'xmgcbcn-B',
      opacity: 0,
    },
    root: {
      ':hover': {
        textDecoration: 'none',
      },
    },
  }),
  stylex.create({
    container: {
      height: '100%',
      width: '100%',
    },
    video: {
      height: '100%',
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      display: 'flex',
    },
  }),
  stylex.create({
    root: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--secondary-button-background)',
      borderBottomWidth: 0,
      borderTopEndRadius: 6,
      borderTopStartRadius: 6,
      height: 30,
      padding: 10,
      position: 'relative',
    },
    trigger: {
      position: 'absolute',
      width: 'fit-content',
    },
    triggerMenu: {
      end: 20,
      top: -18,
    },
    triggerTierSelect: {
      top: -19,
    },
  }),
  stylex.create({
    root: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--secondary-button-background)',
      borderBottomEndRadius: 6,
      borderBottomStartRadius: 6,
      borderTopWidth: 0,
      height: 30,
    },
  }),
  stylex.create({
    errorIconContainer: {
      alignItems: 'center',
      backgroundColor: 'var(--overlay-alpha-80)',
      display: 'flex',
      height: 64,
      justifyContent: 'center',
      width: 64,
    },
    innerContainer: {
      width: '100%',
    },
    link: {
      color: 'var(--blue-link)',
    },
    root: {
      width: '100%',
    },
    textContainer: {
      flexBasis: '0%',
      flexGrow: 1,
      flexShrink: 1,
    },
  }),
  stylex.create({
    playerContainer: {
      height: 0,
      width: '100%',
    },
    root: {
      height: 86,
      marginInline: 0,
      width: '100%',
    },
    rowItem: {
      padding: 0,
      width: '100%',
    },
  }),
  stylex.create({
    backgroundColorStyle: {
      backgroundColor: 'var(--card-background)',
    },
    borderBottomEndRadiusStyle: {
      borderBottomEndRadius: 8,
    },
    borderBottomStartRadiusStyle: {
      borderBottomStartRadius: 8,
    },
    borderRadiusStyle: {
      borderRadius: 8,
    },
    borderTopStartRadiusStyle: {
      borderTopStartRadius: 8,
    },
    boxShadowStyle: {
      boxShadow: '0px 1px 0px var(--hover-overlay), 0px 0px 8px var(--shadow-2)',
    },
    heightStyle: {
      height: 64,
    },
    maxWidthStyle: {
      maxWidth: 700,
    },
    minWidthStyle: {
      minWidth: 200,
    },
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--primary-button-background)',
      borderRadius: 5,
      display: 'inline-block',
      padding: 8,
    },
  }),
  stylex.create({
    blackBackground: {
      backgroundColor: 'var(--always-black)',
    },
    container: {
      borderRadius: 10,
      marginBottom: 8,
      position: 'relative',
    },
    cursor: {
      alignItems: 'center',
      backgroundColor: '#c73842',
      borderColor: 'var(--always-white)',
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: 2.5,
      boxShadow: '0px 8px 16px 0px var(--shadow-2)',
      cursor: 'move',
      display: 'flex',
      justifyContent: 'center',
      position: 'absolute',
      zIndex: 3,
    },
    cursorBorderColor: {
      borderColor: 'var(--media-inner-border)',
    },
    gradient: {
      borderRadius: 8,
      display: 'flex',
      overflow: 'hidden',
    },
    hideSlider: {
      display: 'none',
    },
    keyCommandProps: {
      borderRadius: 8,
      display: 'flex',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 2,
    },
    overlay: {
      borderRadius: 8,
      bottom: 0,
      display: 'flex',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 1,
    },
    sliderRail: {
      backgroundImage:
        'linear-gradient(90deg, var(--always-black) 0%, #c73842 50%, var(--always-white) 100%)',
    },
    sliderTrack: {
      backgroundColor: 'none',
    },
    whiteBackground: {
      backgroundColor: 'var(--always-white)',
    },
    whiteBackgroundBorder: {
      boxShadow: '0px 0px 0px 2px var(--media-inner-border)',
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      justifyContent: 'center',
      maxHeight: 400,
      position: 'relative',
    },
    innerContainer: {
      display: 'flex',
    },
    maxTheEnforcerOfHeight: {
      display: 'flex',
      flexGrow: 1,
      maxHeight: 400,
      position: 'relative',
    },
  }),
  stylex.create({
    docHeight: {
      height: 'calc(100vh - 76px)',
    },
    innerContainer: {
      boxSizing: 'border-box',
      margin: 'auto',
      maxWidth: 732,
      width: '100%',
    },
    noninteractableContainer: {
      pointerEvents: 'none',
    },
    page: {
      backgroundColor: 'var(--card-background)',
      position: 'relative',
    },
  }),
  stylex.create({
    wrapper: {
      padding: 12,
      textAlign: 'center',
    },
  }),
  stylex.create({
    listBottomContainer: {
      padding: 16,
    },
    root: {
      boxShadow: '-1px 0 0 0 var(--shadow-1)',
      height: 'inherit',
    },
    scrollableContainer: {
      height: 'inherit',
    },
    title: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
    },
    titleNonPushView: {
      padding: 16,
    },
    titlePushView: {
      padding: '12px 8px 16px 16px',
    },
  }),
  stylex.create({
    text: {
      paddingInlineStart: 4,
    },
    textContainer: {
      paddingInlineStart: 8,
    },
  }),
  stylex.create({
    bottom: {
      paddingBottom: 20,
    },
    loginButton: {
      paddingInline: 20,
      width: '100%',
    },
    loginTitle: {
      letterSpacing: 0.32,
      marginBottom: '12px',
    },
    top: {
      paddingTop: 48,
    },
  }),
  stylex.create({
    button: {
      alignItems: 'center',
      borderRadius: 4,
      margin: 4,
    },
    icon: {
      height: 16,
      width: 16,
    },
    likedButton: {
      color: 'var(--primary-deemphasized-button-text)',
    },
    likedIcon: {
      color: 'var(--primary-deemphasized-button-text)',
    },
  }),
  stylex.create({
    engagementSectionLayout: {
      paddingTop: 16,
      '@media (max-width: 899px)': {
        paddingTop: 0,
      },
    },
  }),
  stylex.create({
    button: {
      alignItems: 'center',
      borderRadius: 18,
      display: 'flex',
      flexDirection: 'row',
      height: 36,
      justifyContent: 'center',
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      position: 'relative',
      width: '100%',
    },
    icon: {
      display: 'flex',
      height: 16,
      marginInlineEnd: 6,
      width: 16,
    },
    likedButton: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
      color: 'var(--primary-deemphasized-button-text)',
    },
    likedIcon: {
      color: 'var(--primary-deemphasized-button-text)',
    },
    notLikedButton: {
      backgroundColor: 'var(--secondary-button-background)',
      color: 'var(--secondary-button-text)',
    },
    notLikedIcon: {
      color: 'var(--secondary-button-text)',
    },
  }),
  stylex.create({
    card: {
      overflow: 'auto',
      width: 300,
    },
    headingContainer: {
      paddingInline: 16,
      paddingTop: 16,
    },
    listItems: {
      paddingBottom: 12,
      paddingTop: 12,
    },
  }),
  stylex.create({
    qrCode: {
      height: 120,
      paddingInline: 8,
      paddingTop: 8,
      width: 120,
    },
  }),
  stylex.create({
    root: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    footerRow: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      marginInline: 6,
      width: 'calc(100% - 12px)',
    },
    footerRowContent: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      paddingTop: 6,
    },
    footerRowIcon: {
      marginInlineEnd: 6,
    },
  }),
  stylex.create({
    row: {
      margin: '20px 0',
      marginInlineStart: '20px',
    },
    star: {
      marginTop: '8px',
    },
  }),
  stylex.create({
    horizontal_separator: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
    },
    vertical_separator: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderStartColor: 'var(--divider)',
      height: '100%',
    },
  }),
  stylex.create({
    row: {
      marginTop: '4px',
    },
    starRating: {
      paddingBlock: 8,
      position: 'fixed',
    },
  }),
  stylex.create({
    bottomPadding: {
      paddingBottom: 16,
    },
    topPadding: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    body: {
      padding: '20px 20px 0 20px',
    },
    examples: {
      paddingBottom: 36,
      paddingTop: 36,
      width: 600,
    },
    header: {
      backgroundImage:
        'url(\n      /images/business_integrity/non_discrimination/Non_Discrimination_Header.png\n    )',
      backgroundPosition: 'top center',
      backgroundRepeat: 'no-repeat',
      padding: '272px 20px 16px 20px',
    },
    images: {
      textAlign: 'center',
      width: 282,
    },
    intl: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--fds-gray-30)',
      borderTopColor: 'var(--fds-gray-30)',
    },
    topP16: {
      paddingTop: 16,
    },
    topP20: {
      paddingTop: 20,
    },
    vertP20: {
      paddingBottom: 20,
      paddingTop: 20,
    },
    vertP8: {
      paddingBottom: 8,
      paddingTop: 8,
    },
    width: {
      width: '100%',
    },
  }),
  stylex.create({
    expandableCardContentAcceptableTitle: {
      backgroundColor: 'var(--fds-spectrum-teal)',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--fds-spectrum-teal-dark-1)',
      textAlign: 'center',
    },
    expandableCardContentDiscriminationTitle: {
      backgroundColor: 'var(--fds-spectrum-tomato-tint-30)',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--fds-spectrum-tomato)',
      textAlign: 'center',
    },
    expandableCardContentInside: {
      backgroundColor: 'var(--always-white)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--fds-gray-00)',
      borderRadius: 5,
      width: 275,
    },
    expandableCardContentInsideAcceptable: {
      borderColor: 'var(--fds-spectrum-teal-dark-1)',
    },
    expandableCardContentInsideDiscrimination: {
      borderColor: 'var(--fds-spectrum-tomato)',
    },
    marginB12: {
      marginBottom: 12,
    },
    marginL4: {
      marginInlineStart: 4,
    },
    marginR4: {
      marginInlineEnd: 4,
    },
    padding12: {
      paddingBottom: 12,
      paddingTop: 12,
    },
    padding20: {
      paddingBottom: 20,
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
      paddingTop: 20,
    },
    padding4: {
      paddingBottom: 4,
      paddingTop: 4,
    },
    paddingB24: {
      paddingBottom: 24,
    },
  }),
  stylex.create({
    allPadding: {
      padding: 12,
    },
    contentCardPad: {
      paddingBottom: 20,
      paddingInlineEnd: 28,
      paddingInlineStart: 28,
      paddingTop: 20,
    },
    expandableCardContentInside: {
      backgroundColor: 'var(--always-white)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--fds-gray-00)',
      borderRadius: 5,
      overflow: 'hidden',
    },
    listBlock: {
      marginTop: 16,
    },
    listItems: {
      listStylePosition: 'inside',
      listStyleType: 'disc',
      marginBottom: 16,
      paddingInlineStart: 8,
    },
    policyLink: {
      marginTop: 8,
    },
    policyText: {
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      paddingTop: 12,
    },
  }),
  stylex.create({
    close: {
      margin: 20,
    },
  }),
  stylex.create({
    bodySection: {
      margin: 16,
    },
    footer: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      paddingBottom: 16,
    },
  }),
  stylex.create({
    confirmation: {
      margin: 'auto 12px',
    },
    confirmationWrap: {
      alignItems: 'center',
      display: 'flex',
    },
    content: {
      marginTop: 8,
      padding: 12,
    },
    footer: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      padding: 8,
      textAlign: 'end',
    },
    header: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      height: 24,
      padding: 20,
      paddingBottom: 16,
    },
  }),
  stylex.create({
    icon: {
      marginTop: -4,
    },
    root: {
      backgroundColor: 'var(--wash)',
      overflow: 'hidden',
      paddingBottom: 16,
    },
  }),
  stylex.create({
    exampleText: {
      marginInline: 20,
      marginBlock: 8,
    },
  }),
  stylex.create({
    divider: {
      marginBlock: 16,
    },
    header: {
      marginBottom: 16,
    },
    otherReasonTextBox: {
      marginBlock: 12,
    },
  }),
  stylex.create({
    root: {
      height: 500,
      width: 548,
    },
  }),
  stylex.create({
    modalContainer: {
      margin: 12,
      marginBottom: 0,
    },
    modalHeader: {
      marginBottom: 12,
    },
    wycdBody: {
      marginTop: 8,
    },
    wycdSection: {
      marginBottom: 12,
      marginTop: 24,
    },
  }),
  stylex.create({
    root: {
      height: 500,
      width: 548,
    },
  }),
  stylex.create({
    modalContainer: {
      height: 220,
      marginBottom: 0,
      marginInline: 12,
      marginTop: 12,
    },
    modalHeader: {
      marginBottom: 12,
    },
    wycdBody: {
      marginInlineStart: 36,
    },
    wycdComponent: {
      marginTop: 8,
    },
    wycdHeader: {
      marginBottom: 4,
      marginInlineStart: -8,
    },
    wycdSection: {
      marginBottom: 12,
      marginTop: 24,
    },
  }),
  stylex.create({
    badge: {
      backgroundColor: '#e6e6e6',
      borderRadius: 4,
      display: 'inline-block',
      marginBottom: 1,
      padding: '4px 4px',
    },
    weight: {
      fontWeight: 500,
    },
  }),
  stylex.create({
    vert16: {
      paddingBlock: 16,
    },
  }),
  stylex.create({
    discardPadding: {
      padding: 16,
    },
  }),
  stylex.create({
    padding: {
      paddingInline: 0,
      paddingBlock: 16,
    },
  }),
  stylex.create({
    stepsContainer: {
      marginBottom: '32px',
    },
    welcomeImage: {
      width: '100%',
    },
  }),
  stylex.create({
    stepsContainer: {
      marginBottom: '32px',
    },
    welcomeImage: {
      width: '100%',
    },
  }),
  stylex.create({
    header: {
      paddingInline: 16,
      paddingBlock: 12,
    },
  }),
  stylex.create({
    header: {
      paddingInline: 16,
      paddingBlock: 12,
    },
  }),
  stylex.create({
    countryCode: {
      maxWidth: '20%',
    },
  }),
  stylex.create({
    item: {
      listStyleType: 'disc',
      marginBottom: 4,
    },
    list: {
      marginInlineStart: 24,
    },
  }),
  stylex.create({
    resendCode: {
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 8,
    },
    row: {
      paddingInline: 16,
      paddingTop: 16,
    },
    troubleshooting: {
      paddingBottom: 12,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 12,
    },
  }),
  stylex.create({
    item: {
      listStyleType: 'decimal',
      marginBlock: 5,
    },
    list: {
      marginInlineStart: -25,
      marginBlock: -5,
      paddingBlock: 10,
    },
  }),
  stylex.create({
    paddingTopZero: {
      paddingTop: 0,
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
    marginBottom16: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    paddingTopZero: {
      paddingTop: 0,
    },
  }),
  stylex.create({
    divider: {
      margin: '12px 16px 0 16px',
    },
    fullWidth: {
      width: '100%',
    },
    marginTop: {
      marginTop: '20px',
    },
  }),
  stylex.create({
    marginTop: {
      marginTop: '20px',
    },
  }),
  stylex.create({
    divider: {
      margin: '12px 16px 10px 16px',
    },
    subheading: {
      margin: '0 0 24px',
    },
  }),
  stylex.create({
    confidentialLabel: {
      paddingInlineEnd: '2px',
    },
    marginTop: {
      marginTop: '20px',
    },
    ohioLabel: {
      padding: '11px 0 8px 0',
    },
    radioInputs: {
      padding: '4px 8px 0 8px',
    },
  }),
  stylex.create({
    marginTop: {
      marginTop: '20px',
    },
  }),
  stylex.create({
    fullWidth: {
      width: '100%',
    },
    marginTop: {
      marginTop: '20px',
    },
  }),
  stylex.create({
    row: {
      alignItems: 'center',
      justifyContent: 'flex-start',
      margin: 0,
    },
  }),
  stylex.create({
    item: {
      listStyleType: 'disc',
    },
    list: {
      marginInlineStart: 24,
    },
    recommendedBadgeStyle: {
      backgroundColor: 'var(--comment-background)',
      borderRadius: 8,
      color: 'var(--blue-link)',
      display: 'inline-block',
      fontSize: 10,
      margin: '0, 8px',
      padding: '2px, 8px',
    },
  }),
  stylex.create({
    item: {
      listStyleType: 'disc',
      marginBottom: 4,
    },
    list: {
      marginInlineStart: 24,
    },
  }),
  stylex.create({
    paddingTopZero: {
      paddingTop: 0,
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
    marginBottom16: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    item: {
      listStyleType: 'disc',
    },
    list: {
      marginInlineStart: 24,
    },
    recommendedBadgeStyle: {
      backgroundColor: 'var(--comment-background)',
      borderRadius: 8,
      color: 'var(--blue-link)',
      display: 'inline-block',
      fontSize: 10,
      margin: '0, 8px',
      padding: '2px, 8px',
    },
  }),
  stylex.create({
    bottom10: {
      paddingBottom: 10,
    },
    top12: {
      paddingTop: 12,
    },
  }),
  stylex.create({
    all16: {
      margin: 'var(--p-space-4)',
    },
    bottom16: {
      marginBottom: 'var(--p-space-4)',
    },
    horiz16: {
      marginInline: 'var(--p-space-4)',
    },
    top16: {
      marginTop: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    horizontalMargin: {
      marginInline: 'var(--p-space-4)',
    },
    vertical8Margin: {
      marginBlock: 'var(--p-space-2)',
    },
    verticalMargin: {
      marginBlock: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    containerMargin: {
      margin: 'var(--p-space-4)',
    },
    verticalMargin: {
      marginBlock: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    padding: {
      paddingInline: 'var(--p-space-4)',
      paddingBlock: 'var(--p-space-3)',
    },
  }),
  stylex.create({
    container: {
      margin: 'var(--p-space-4)',
    },
    description: {
      marginInlineStart: 'var(--p-space-8)',
    },
    divider: {
      borderBottomStyle: 'solid',
      borderBottomWidth: 1,
      height: 0,
      marginInlineStart: 'var(--p-space-16)',
      marginBlock: 'var(--p-space-3)',
    },
  }),
  stylex.create({
    reviewInstructions: {
      paddingInline: 'var(--p-space-4)',
      paddingBlock: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    containerMargin: {
      margin: 'var(--p-space-4)',
    },
    verticalMargin: {
      marginBlock: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    containerMargin: {
      margin: 'var(--p-space-4)',
    },
    divider: {
      borderBottomStyle: 'solid',
      borderBottomWidth: 1,
      height: 0,
      marginBlock: 'var(--p-space-4)',
    },
    involvedSteps: {
      width: '520px',
    },
  }),
  stylex.create({
    modalContentGrid: {
      display: 'grid',
      gridRowGap: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    container: {
      margin: 'var(--p-space-4)',
    },
    personalNumber: {
      marginTop: 'var(--p-space-3)',
    },
  }),
  stylex.create({
    container: {
      margin: 'var(--p-space-4)',
    },
    detailedInfo: {
      marginBottom: 'var(--p-space-3)',
    },
    divider: {
      borderBottomStyle: 'solid',
      borderBottomWidth: 1,
      height: 0,
      marginBlock: 'var(--p-space-4)',
    },
    row: {
      alignItems: 'center',
      justifyContent: 'flex-start',
      margin: 0,
      paddingTop: 'var(--p-space-1)',
    },
    subheading: {
      marginBottom: 'var(--p-space-2)',
    },
  }),
  stylex.create({
    container: {
      margin: 'var(--p-space-4)',
    },
    emailInput: {
      marginTop: 'var(--p-space-2)',
    },
  }),
  stylex.create({
    containerMargin: {
      margin: 'var(--p-space-4)',
    },
    horiz16: {
      marginInline: 'var(--p-space-4)',
    },
    vert16: {
      marginBlock: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    horizontalMargin: {
      marginInline: 'var(--p-space-4)',
    },
    resendMargin: {
      marginBlock: 'var(--p-space-8)',
    },
    verticalMargin: {
      marginBlock: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    containerMargin: {
      margin: 'var(--p-space-4)',
    },
  }),
  stylex.create({
    marginBlock: {
      marginBlock: 8,
    },
    nmiPadding: {
      padding: 16,
    },
    nmiSeparator: {
      marginBlock: 8,
    },
  }),
  stylex.create({
    back: {
      marginBottom: 28,
    },
    button: {
      marginTop: 32,
    },
  }),
  stylex.create({
    back: {
      marginBottom: 28,
    },
  }),
  stylex.create({
    background: {
      height: '100vh',
      position: 'fixed',
      width: '100vw',
    },
    footer: {
      bottom: 0,
      boxShadow: '0px 50vh 0px 50vh var(--always-white)',
      position: 'absolute',
      width: '100vw',
    },
    searchView: {
      marginBottom: 400,
    },
  }),
  stylex.create({
    resultList: {
      paddingInline: 0,
      paddingBlock: 20,
      width: 635,
    },
    searchForm: {
      marginTop: 200,
      paddingInline: 32,
      paddingBlock: 20,
      width: 600,
    },
  }),
  stylex.create({
    background: {
      height: '100vh',
      position: 'fixed',
      width: '100vw',
    },
    codeEntryView: {
      marginBottom: 400,
      paddingTop: 173,
    },
    footer: {
      bottom: 0,
      boxShadow: '0px 50vh 0px 50vh var(--always-white)',
      position: 'absolute',
      width: '100vw',
    },
  }),
  stylex.create({
    back: {
      marginBottom: 34,
      marginInline: 16,
    },
    button: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'center',
      marginBottom: 12,
      marginInline: 16,
      width: 600,
    },
    errorBanner: {
      marginInline: 16,
      marginTop: 12,
      width: 600,
    },
    text: {
      height: 30,
      marginInline: 16,
      marginTop: 16,
    },
    textInput: {
      marginBottom: 24,
      marginInline: 16,
      marginTop: 20,
      width: 600,
    },
  }),
  stylex.create({
    backButton: {
      marginBottom: 28,
    },
    chooseMethodText: {
      height: 30,
      marginTop: 25,
    },
    continueButton: {
      marginInline: 16,
      marginTop: 55,
      width: 600,
    },
    name: {
      marginInline: 10,
      marginTop: 25,
    },
    notYouButton: {
      marginInline: 16,
      marginTop: 10,
      width: 600,
    },
    recoveryOptionList: {
      minWidth: 600,
    },
    recoveryOptions: {
      marginTop: 20,
    },
  }),
  stylex.create({
    background: {
      height: '100vh',
      position: 'fixed',
      width: '100vw',
    },
    footer: {
      bottom: 0,
      boxShadow: '0px 50vh 0px 50vh var(--always-white)',
      position: 'absolute',
      width: '100vw',
    },
    initiateView: {
      marginBottom: 400,
      paddingTop: 173,
    },
  }),
  stylex.create({
    reCaptcha: {
      backgroundColor: 'transparent',
      borderStyle: 'none',
      marginTop: 16,
      width: 600,
    },
  }),
  stylex.create({
    radioDisabled: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    radioSelected: {
      backgroundColor: 'var(--accent)',
      borderRadius: '50%',
      height: 12,
      position: 'absolute',
      width: 12,
    },
    root: {
      WebkitTapHighlightColor: 'transparent',
      alignItems: 'center',
      cursor: 'pointer',
      display: 'flex',
      paddingBottom: 8,
      paddingTop: 8,
      position: 'relative',
      touchAction: 'manipulation',
    },
    selectedSizeLarge: {
      start: 4,
      top: 4,
    },
    selectedSizeMedium: {
      start: 2,
      top: 2,
    },
    text: {
      marginInlineStart: 8,
    },
  }),
  stylex.create({
    add_on_link: {
      maxWidth: '158px',
    },
  }),
  stylex.create({
    placeHolderContainer: {
      backgroundColor: 'var(--wash)',
      height: 309,
      padding: 16,
    },
    placeHolderContext: {
      height: '60%',
      marginBottom: '10px',
      marginTop: '10px',
      width: '100%',
    },
    placeHolderTitle: {
      height: '15%',
      width: '100%',
    },
  }),
  stylex.create({
    back: {
      marginBottom: 28,
    },
    checkbox: {
      marginTop: 30,
    },
  }),
  stylex.create({
    background: {
      height: '100vh',
      position: 'fixed',
      width: '100vw',
    },
    footer: {
      bottom: 0,
      position: 'absolute',
      width: '100vw',
    },
    mainView: {
      marginBottom: 400,
      paddingTop: 175,
    },
  }),
  stylex.create({
    form: {
      marginBottom: 'auto',
      marginTop: 'auto',
      paddingInline: 32,
      width: 600,
    },
  }),
  stylex.create({
    errorIconContainer: {
      backgroundColor: 'var(--negative)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--negative)',
      marginInlineEnd: '1px',
      marginInlineStart: '1px',
    },
    errorMessageContainer: {
      backgroundColor: 'var(--always-white)',
      paddingBottom: '10px',
      paddingTop: '10px',
    },
    infoBox: {
      marginBottom: '4px',
      paddingBottom: '12px',
    },
    notification: {
      padding: '10px',
    },
    passwordStrength: {
      marginBottom: 0,
      marginTop: 0,
    },
    washBackgroundColor: {
      backgroundColor: 'var(--web-wash)',
    },
  }),
  stylex.create({
    background: {
      height: '100vh',
      width: '100vw',
    },
    gradient: {
      position: 'absolute',
    },
  }),
  stylex.create({
    card: {
      maxWidth: 558,
      paddingTop: 88,
      width: '95vw',
    },
    cardPadding: {
      padding: 16,
    },
  }),
  stylex.create({
    boxContainer: {
      backgroundColor: 'var(--negative-background)',
      borderColor: 'var(--negative)',
      borderStyle: 'solid',
      borderWidth: 1,
      marginBottom: 16,
      marginInline: 16,
    },
    errorContent: {
      marginBottom: 8,
      marginInline: 8,
    },
    errorTitle: {
      marginBottom: 8,
      marginTop: 16,
      marginBlock: 8,
    },
  }),
  stylex.create({
    loginButton: {
      paddingBottom: 8,
    },
    loginDesc: {
      paddingBottom: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    card: {
      borderRadius: 10,
      boxShadow: '0 2px 12px var(--shadow-2)',
      width: 396,
    },
    contactpointMargins: {
      marginInline: 16,
    },
    fatalErrorContainer: {
      marginTop: 16,
    },
    googleLoginSection: {
      marginInline: 16,
    },
    paddingFormBottom: {
      paddingTop: 16,
    },
    regButton: {
      alignSelf: 'center',
      width: 200,
    },
    regButtonMargins: {
      marginInline: 16,
      marginBlock: 16,
    },
    sectionMargins: {
      marginInline: 16,
      marginBlock: 16,
    },
  }),
  stylex.create({
    knownUserBlock: {
      display: 'flex',
      justifyContent: 'center',
      paddingBottom: 32,
    },
    paddingAroundHeader: {
      paddingTop: 16,
    },
    profilePhoto: {
      alignSelf: 'center',
      paddingTop: 16,
    },
    sectionMargins: {
      marginInline: 16,
      marginBlock: 16,
    },
  }),
  stylex.create({
    background: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    footer: {
      boxShadow: '0px 50vh 0px 50vh var(--always-white)',
      paddingTop: 220,
      position: 'relative',
      width: '100vw',
    },
    form: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginTop: 32,
    },
    loginFormPlaceholder: {
      height: 416,
      width: 396,
    },
    logo: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginTop: 48,
    },
  }),
  stylex.create({
    column: {
      margin: '0 auto',
      width: 520,
    },
    footer: {
      maxHeight: 212,
    },
    page: {
      maxHeight: 'calc(100vh - 212px)',
      minHeight: 700,
    },
  }),
  stylex.create({
    page: {
      maxHeight: 'calc(100vh - 212px)',
      minHeight: 700,
    },
  }),
  stylex.create({
    column: {
      alignSelf: 'center',
      display: 'flex',
      height: 'calc(100vh - 212px)',
      maxHeight: 'calc(100vh - 212px)',
      maxWidth: 480,
      minHeight: 700,
      width: 'calc(100vw - 20px)',
    },
  }),
  stylex.create({
    footer: {
      maxHeight: 212,
    },
    page: {
      maxHeight: 'calc(100vh - 212px)',
      minHeight: 700,
    },
  }),
  stylex.create({
    AYMHComponent: {
      paddingInlineEnd: 86,
    },
    background: {
      height: '100vh',
      position: 'fixed',
      width: '100vw',
    },
    branding: {
      margin: '0 40px 0 40px',
      paddingInlineEnd: 36,
      paddingTop: 180,
      '@media screen and (max-width: 1110px)': {
        paddingInlineEnd: 0,
      },
    },
    defaultView: {
      display: 'flex',
      flexDirection: 'row',
      margin: '0 auto 0 auto',
      maxWidth: 1110,
      paddingBottom: 100,
      paddingTop: 84,
      '@media screen and (max-width: 1110px)': {
        flexDirection: 'column',
        flexWrap: 'wrap',
      },
    },
    footer: {
      position: 'relative',
      width: '100vw',
    },
    leftComponent: {
      paddingInlineEnd: 24,
    },
    placeholderLoginForm: {
      height: 498,
      width: 500,
    },
    rightComponent: {
      margin: '0 40px',
      paddingTop: 36,
    },
  }),
  stylex.create({
    texts: {
      display: 'inline-block',
      width: 'wrap',
    },
  }),
  stylex.create({
    AYMHComponent: {
      paddingInlineEnd: 86,
    },
    background: {
      height: '100%',
      position: 'fixed',
      width: '100%',
    },
    defaultView: {
      columnGap: 66,
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap-reverse',
      height: '100%',
      justifyContent: 'center',
      paddingTop: 152,
      width: '100%',
    },
    footer: {
      paddingBottom: 50,
      paddingTop: 50,
      position: 'relative',
      width: '100vw',
    },
    leftComponent: {
      paddingInlineEnd: 24,
    },
    placeholderAppsell: {
      height: 120,
      width: 500,
    },
    placeholderBranding: {
      height: 90,
      width: 500,
    },
    placeholderLoginForm: {
      height: 446,
      width: 500,
    },
    rightComponent: {
      paddingBottom: 36,
      paddingTop: 36,
    },
  }),
  stylex.create({
    card: {
      maxWidth: 500,
      width: 'calc(100vw - 20px)',
    },
  }),
  stylex.create({
    background: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    branding: {
      paddingInlineEnd: 48,
      paddingTop: 104,
    },
    content: {
      paddingTop: 84,
    },
    footer: {
      boxShadow: '0px 50vh 0px 50vh var(--always-white)',
      paddingTop: 270,
      position: 'relative',
      width: '100vw',
    },
    leftComponent: {
      paddingInlineEnd: 44,
    },
    placeholderLoginForm: {
      height: 380,
      width: 396,
    },
    rightComponent: {
      paddingTop: 36,
    },
  }),
  stylex.create({
    dividerContainer: {
      width: '100%',
    },
    footer: {
      maxWidth: 1110,
    },
    wrap: {
      overflowWrap: 'anywhere',
    },
  }),
  stylex.create({
    forgotLink: {
      margin: '0 0 2px 4px',
    },
    logInForm: {
      display: 'none',
      '@media (min-width: 950px)': {
        display: 'flex',
      },
    },
    redirectToLogInButton: {
      display: 'flex',
      '@media (min-width: 950px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    disabled: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    primaryButton: {
      backgroundColor: 'var(--primary-button-background)',
      borderRadius: 6,
      height: 40,
      overflow: 'hidden',
      padding: '0 12px 2px 12px',
    },
    primaryOverlayPressed: {
      backgroundColor: 'rgba(9, 30, 66, 0.15)',
    },
    secondaryButton: {
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: 6,
      height: 40,
      overflow: 'hidden',
      padding: '0 12px 2px 12px',
    },
  }),
  stylex.create({
    forgotLink: {
      margin: '0 0 2px 4px',
    },
    headerExpButton: {
      display: 'flex',
    },
    logInForm: {
      display: 'none',
      '@media (min-width: 950px)': {
        display: 'flex',
      },
    },
    redirectToLogInButton: {
      display: 'flex',
      '@media (min-width: 950px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    container: {
      color: 'var(--secondary-text)',
      fontSize: 15,
      lineHeight: 1.3333333333333333,
    },
    disabled: {
      cursor: 'not-allowed',
    },
    input: {
      backgroundColor: 'transparent',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 6,
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      fontSize: 'inherit',
      fontWeight: 500,
      height: 40,
      padding: '8px 12px 10px',
      textAlign: 'start',
      width: '188px',
      '::placeholder': {
        color: 'var(--placeholder-text)',
      },
      ':focus': {
        borderColor: 'var(--accent)',
      },
      ':focus::placeholder': {
        color: 'var(--disabled-text)',
      },
      ':hover': {
        borderColor: 'var(--placeholder-text)',
      },
    },
    label: {
      paddingBottom: 4,
    },
  }),
  stylex.create({
    accountSwitcher: {
      width: 536,
    },
  }),
  stylex.create({
    branding: {
      maxWidth: 500,
      width: '100%',
    },
    tagline: {
      width: 500,
    },
  }),
  stylex.create({
    closeButton: {
      end: 4,
      position: 'absolute',
      top: 4,
      zIndex: 1,
    },
    image: {
      alignItems: 'center',
      backgroundColor: 'var(--web-wash)',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'center',
    },
    name: {
      marginInline: 10,
      marginTop: -40,
    },
    normalCard: {
      height: 206,
      width: 160,
    },
    normalImage: {
      height: 160,
      marginTop: 20,
      width: 160,
    },
    notificationLabel: {
      marginTop: 15,
    },
    smallCard: {
      height: 130,
      marginTop: 12,
      width: 100,
    },
    smallImage: {
      height: 100,
      width: 100,
    },
  }),
  stylex.create({
    subtitle: {
      fontSize: 15,
      fontWeight: 500,
    },
    tagline: {
      fontSize: 28,
      fontWeight: 500,
    },
  }),
  stylex.create({
    primary: {
      color: 'var(--primary-text)',
    },
    secondary: {
      color: 'var(--secondary-text)',
    },
  }),
  stylex.create({
    default: {
      fontFamily: 'var(--font-family-default) !important',
    },
  }),
  stylex.create({
    checkbox: {
      width: '100%',
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    forgotPassword: {
      marginTop: 16,
    },
    genericErrorBanner: {
      marginBottom: 35,
    },
    horizontalMargin: {
      marginInline: 20,
    },
    loginButton: {
      marginTop: 48,
    },
    logIntoAnotherAccountButton: {
      marginTop: 16,
    },
    name: {
      marginInline: 10,
      marginTop: 15,
    },
    passwordInput: {
      marginTop: 25,
    },
  }),
  stylex.create({
    description: {
      marginInline: 40,
    },
  }),
  stylex.create({
    background: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
  }),
  stylex.create({
    content: {
      paddingTop: 129,
      width: 640,
    },
  }),
  stylex.create({
    content: {
      width: '100%',
    },
  }),
  stylex.create({
    anchor: {
      minHeight: 'initial',
    },
  }),
  stylex.create({
    dividerContainer: {
      width: '100%',
    },
    footer: {
      maxWidth: 1110,
    },
  }),
  stylex.create({
    rectangle: {
      height: '100px',
      width: '200px',
    },
  }),
  stylex.create({
    content: {
      width: '100%',
    },
  }),
  stylex.create({
    footer: {
      height: '200',
      width: '100vw',
    },
  }),
  stylex.create({
    content: {
      width: 600,
    },
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--wash)',
      margin: '0 auto',
      padding: 48,
      width: 876,
    },
    rootFloating: {
      backgroundColor: 'var(--wash)',
      margin: '0 auto',
      padding: 48,
      width: 400,
    },
  }),
  stylex.create({
    dialog: {
      margin: 12,
      marginTop: 24,
    },
  }),
  stylex.create({
    cardHeader: {
      alignItems: 'flex-start',
      flexDirection: 'column',
      paddingInline: 16,
    },
    icon: {
      marginInline: 0,
      marginBlock: 8,
    },
  }),
  stylex.create({
    privacyDeclaration: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'nowrap',
      paddingBottom: 8,
      paddingInline: 0,
      paddingTop: 0,
    },
    privacyIcon: {
      marginInlineEnd: 8,
      marginInline: 0,
      marginBlock: 8,
      paddingInlineEnd: 8,
    },
  }),
  stylex.create({
    button: {
      flexGrow: 1,
      minWidth: '50%',
      paddingInline: 12,
      paddingBlock: 10,
    },
    cardContainer: {
      borderRadius: 16,
    },
    composer: {
      paddingTop: 16,
    },
    questionContainer: {
      marginBlock: 0,
      paddingInline: 16,
    },
    questionTitle: {
      paddingBottom: 0,
      paddingTop: 12,
    },
    textPairing: {
      paddingInline: 16,
      paddingBlock: 8,
    },
  }),
  stylex.create({
    list: {
      listStyle: 'disc',
      marginInlineStart: 10,
      paddingInlineStart: 10,
    },
    listMargin: {
      marginBottom: 16,
    },
    subtitle: {
      paddingBlock: 6,
    },
  }),
  stylex.create({
    icon: {
      display: 'flex',
      justifyContent: 'center',
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      marginBlock: 96,
    },
  }),
  stylex.create({
    column: {
      maxWidth: '880px',
    },
  }),
  stylex.create({
    importButtonContainer: {
      marginTop: 40,
    },
  }),
  stylex.create({
    failedImportText: {
      color: 'var(--base-cherry)',
    },
    ongoingImportText: {
      color: 'var(--base-lemon)',
    },
    successfulImportText: {
      color: 'var(--base-lime)',
    },
  }),
  stylex.create({
    table: {
      width: '400px',
    },
  }),
  stylex.create({
    submit: {
      padding: '0px 16px 16px 16px',
    },
  }),
  stylex.create({
    description: {
      paddingTop: '8px',
    },
  }),
  stylex.create({
    table: {
      width: '300px',
    },
  }),
  stylex.create({
    cell: {
      padding: '4px',
    },
    leftColumn: {
      textAlign: 'end',
      width: '120px',
    },
  }),
  stylex.create({
    importButtonContainer: {
      marginTop: 40,
    },
  }),
  stylex.create({
    importButtonContainer: {
      marginTop: 40,
    },
  }),
  stylex.create({
    validationList: {
      marginInlineStart: 16,
    },
  }),
  stylex.create({
    card: {
      maxHeight: 'calc(100vh - 60px)',
      maxWidth: 'calc(100vw - 24px)',
      width: 360,
    },
    content: {
      marginTop: 8,
    },
    legalFooter: {
      padding: '12px 16px 16px',
    },
    root: {
      marginTop: 5,
    },
    section: {
      padding: '4px 0 16px 0',
    },
  }),
  stylex.create({
    anchor: {
      display: 'flex',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    wrapper: {
      maxWidth: 584,
      width: '100%',
    },
  }),
  stylex.create({
    header: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      boxSizing: 'content-box',
      height: 60,
    },
    headerContent: {
      alignItems: 'center',
      boxSizing: 'content-box',
      display: 'flex',
      height: '100%',
      justifyContent: 'flex-start',
      paddingInline: 16,
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 7,
      height: 14,
      marginBottom: 14,
    },
    bodyGlimmerContainer: {
      paddingBottom: '150px',
    },
    bodyGlimmerFirst: {
      width: '80%',
    },
    bodyGlimmerSecond: {
      width: '40%',
    },
    headerGlimmer: {
      borderRadius: 10,
      height: 20,
      marginBottom: 7,
      marginTop: 8,
      width: 100,
    },
  }),
  stylex.create({
    contentContainer: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
      width: '100%',
      zIndex: 0,
    },
    negativeMarginMobile: {
      marginTop: '-68px',
      '@media (min-width: 584px)': {
        marginTop: 0,
      },
    },
    paddingDefault: {
      '@media (min-width: 584px)': {
        paddingTop: 24,
      },
    },
  }),
  stylex.create({
    item: {
      marginBottom: 8,
      marginTop: 8,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: -8,
      marginTop: -8,
    },
  }),
  stylex.create({
    paragraph: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    hiddenInput: {
      clip: 'rect(0, 0, 0, 0)',
      clipPath: 'inset(50%)',
      height: 1,
      overflow: 'hidden',
      position: 'absolute',
      width: 1,
    },
  }),
  stylex.create({
    profilePhoto: {
      display: 'flex',
      marginInlineEnd: 6,
      marginInlineStart: -8,
    },
    root: {
      alignItems: 'center',
      borderRadius: 18,
      display: 'flex',
      flexDirection: 'row',
      height: 36,
      justifyContent: 'center',
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      position: 'relative',
    },
  }),
  stylex.create({
    container: {
      alignSelf: 'center',
      flexDirection: 'column',
      marginTop: '12px',
      maxWidth: '584px',
      '@media (min-width: 584px)': {
        width: '100%',
      },
    },
  }),
  stylex.create({
    outcomesContainer: {
      marginInlineStart: -8,
    },
  }),
  stylex.create({
    iframe: {
      borderStyle: 'none',
    },
  }),
  stylex.create({
    resendCodeProgressRingContainer: {
      height: 12,
    },
  }),
  stylex.create({
    responsive: {
      '@media (max-width: 564px)': {
        flexDirection: 'column',
      },
    },
  }),
  stylex.create({
    errorLabel: {
      bottom: 16,
      end: 32,
      position: 'absolute',
      start: 32,
    },
  }),
  stylex.create({
    matchingAccountCard: {
      marginBlock: 16,
      width: 250,
    },
    matchingAccountIcon: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 28,
      height: 56,
      width: 56,
    },
  }),
  stylex.create({
    body: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: -8,
      marginTop: 16,
      paddingInlineEnd: 24,
    },
  }),
  stylex.create({
    matchingAccountCard: {
      marginBlock: 16,
      width: 250,
    },
    matchingAccountIcon: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 28,
      height: 56,
      width: 56,
    },
  }),
  stylex.create({
    item: {
      marginBottom: 8,
      marginTop: 8,
    },
    listItem: {
      marginTop: 4,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 8,
      marginInlineEnd: 16,
      marginInlineStart: -10,
      marginTop: 12,
    },
  }),
  stylex.create({
    headerContainer: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'space-between',
      minHeight: 60,
      position: 'relative',
    },
    headerItem: {
      marginInline: 16,
    },
    headerPlaceholder: {
      height: 36,
      width: 36,
    },
  }),
  stylex.create({
    column: {
      height: 114,
    },
    link: {
      padding: '16px',
    },
  }),
  stylex.create({
    user_card: {
      backgroundColor: 'var(--card-background-flat)',
      borderRadius: 8,
      padding: '10px 16px',
    },
  }),
  stylex.create({
    column: {
      paddingInline: 16,
      paddingBlock: 32,
    },
    notFirstItem: {
      marginTop: 16,
    },
  }),
  stylex.create({
    button: {
      marginTop: 32,
    },
  }),
  stylex.create({
    deviceIconBorder: {
      alignItems: 'center',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      width: 60,
    },
  }),
  stylex.create({
    body: {
      paddingBottom: 25,
      paddingInline: 16,
      paddingTop: 12,
    },
  }),
  stylex.create({
    defaultGeometryStyle: {
      height: 614,
    },
    loggedInSpinner: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      padding: 16,
    },
    loggedInSpinnerText: {
      marginTop: 16,
    },
  }),
  stylex.create({
    child: {
      marginTop: 8,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginTop: 19,
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--card-background)',
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    spinner: {
      height: '100%',
      position: 'relative',
    },
  }),
  stylex.create({
    child: {
      marginTop: 8,
    },
    disabledNexButton: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    nextButton: {
      backgroundColor: 'var(--primary-button-background)',
      borderStyle: 'none',
      borderRadius: 'var(--button-corner-radius)',
      height: 'var(--button-height-large)',
      paddingInline: 12,
      width: '100%',
    },
    nextButtonWrapper: {
      width: '100%',
    },
  }),
  stylex.create({
    errorMessage: {
      marginTop: 8,
    },
  }),
  stylex.create({
    tabBar: {
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
      zIndex: 1,
    },
  }),
  stylex.create({
    navbar: {
      alignItems: 'center',
      backgroundColor: 'var(--fds-blue-70)',
      display: 'flex',
      flexGrow: 1,
      height: '44px',
      justifyContent: 'center',
      position: 'fixed',
      start: 0,
      top: 0,
      width: '100%',
      zIndex: 1,
    },
    title: {
      maxWidth: '100%',
      paddingInline: 12,
    },
  }),
  stylex.create({
    buttonText: {
      paddingInlineStart: 2,
    },
    defaultContainer: {
      alignItems: 'center',
      color: 'var(--secondary-text)',
      display: 'flex',
      flexDirection: 'row',
      fontSize: 13,
      lineHeight: 1.2,
      paddingBottom: 8,
      paddingInlineStart: 12,
      paddingTop: 2,
    },
  }),
  stylex.create({
    birthdayVoterRegistrationContainer: {
      alignItems: 'center',
      color: 'var(--secondary-text)',
      display: 'flex',
      flexDirection: 'row',
      fontSize: 13,
      lineHeight: 1.2,
      paddingBottom: 8,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    buttonText: {
      paddingInlineStart: 2,
    },
    defaultContainer: {
      alignItems: 'center',
      color: 'var(--secondary-text)',
      display: 'flex',
      flexDirection: 'row',
      fontSize: 13,
      lineHeight: 1.2,
      paddingBottom: 12,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    disclaimerPaddingTop: {
      paddingTop: 2,
    },
    disclaimerText: {
      paddingInlineEnd: 2,
    },
    rtl: {
      justifyContent: 'flex-end',
    },
    socialContextContainer: {
      paddingBottom: 4,
    },
    ukraineHubQPContainer: {
      alignItems: 'center',
      color: 'var(--secondary-text)',
      display: 'flex',
      flexDirection: 'row',
      fontSize: 13,
      lineHeight: 1.2,
      paddingBottom: 8,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      paddingTop: 4,
    },
    voterRegistrationPostContainer: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
    },
    voterRegistrationQPContainer: {
      alignItems: 'center',
      color: 'var(--secondary-text)',
      display: 'flex',
      flexDirection: 'row',
      fontSize: 13,
      lineHeight: 1.2,
      paddingBottom: 16,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
  }),
  stylex.create({
    icon: {
      alignItems: 'center',
      backgroundColor: 'var(--web-wash)',
      borderRadius: 'inherit',
      display: 'flex',
      height: 36,
      justifyContent: 'center',
      width: 36,
    },
    iconActive: {
      backgroundColor: 'var(--card-background)',
    },
  }),
  stylex.create({
    row: {
      marginInlineEnd: 16,
      marginInlineStart: 16,
      paddingBottom: 8,
    },
  }),
  stylex.create({
    addressEditLink: {
      marginInlineStart: 12,
    },
    addressEditSection: {
      alignItems: 'baseline',
      display: 'flex',
      paddingTop: 30,
    },
    addressInputSection: {
      height: 68,
    },
    card: {
      padding: 16,
    },
    middot: {
      marginInline: 3,
    },
    spacing: {
      marginTop: 12,
    },
    typeahead: {
      paddingBlock: 16,
      width: '100%',
    },
    wrapper: {
      marginBottom: 12,
      marginTop: 12,
    },
  }),
  stylex.create({
    buttons: {
      padding: '0 0 15px 0',
    },
    dialogBody: {
      color: 'var(--secondary-text)',
      padding: '15px 15px 0px 15px',
    },
  }),
  stylex.create({
    addressEntry: {
      paddingBlock: 15,
      textAlign: 'center',
      width: '100%',
    },
    dialogBody: {
      backgroundColor: 'var(--surface-background)',
      color: 'var(--secondary-text)',
      padding: '10px 15px 0px 15px',
    },
  }),
  stylex.create({
    edit: {
      color: 'var(--primary-deemphasized-button-text)',
    },
  }),
  stylex.create({
    buttonsContainer: {
      display: 'flex',
      justifyContent: 'flex-end',
      marginInlineEnd: -16,
    },
    container: {
      padding: 16,
    },
    topText: {
      paddingBottom: 16,
      paddingTop: 24,
    },
  }),
  stylex.create({
    bottomText: {
      paddingBottom: 4,
      paddingTop: 20,
    },
    buttonsContainer: {
      display: 'flex',
      justifyContent: 'flex-end',
      marginInlineEnd: -16,
    },
    container: {
      padding: 16,
    },
    topText: {
      paddingBottom: 20,
      paddingTop: 4,
    },
  }),
  stylex.create({
    item: {
      marginBottom: 6,
      marginTop: 6,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: -6,
      marginTop: -6,
    },
    seeOnlyEnglishButton: {
      marginBlock: 9,
    },
    title: {
      color: 'var(--primary-text)',
      fontSize: 20,
      fontWeight: 500,
    },
  }),
  stylex.create({
    root: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      margin: '0 10px',
      padding: 10,
      textAlign: 'center',
    },
  }),
  stylex.create({
    link: {
      marginTop: 12,
    },
    listItem: {
      marginBottom: 16,
      marginInlineEnd: 16,
      marginInlineStart: 16,
      marginTop: 12,
    },
    text: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: -5,
      marginTop: -5,
    },
    textItem: {
      marginBottom: 5,
      marginTop: 5,
    },
  }),
  stylex.create({
    footerText: {
      display: 'flex',
      marginBottom: 16,
      marginTop: 4,
    },
    learnMoreLink: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    countryLink: {
      padding: 16,
      width: '100%',
    },
    scrollableArea: {
      maxHeight: 500,
    },
  }),
  stylex.create({
    dateChangeText: {
      fontSize: '16px',
    },
    nextStepText: {
      fontSize: '20px',
      paddingInline: '12px',
      paddingTop: '12px',
    },
    nonInternBody: {
      fontSize: '20px',
    },
  }),
  stylex.create({
    value: {
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    selector: {
      display: 'inline-block',
      marginInlineEnd: 12,
      minWidth: 232,
    },
  }),
  stylex.create({
    root: {
      padding: 40,
    },
    tableWrapper: {
      paddingTop: 12,
    },
  }),
  stylex.create({
    cell: {
      alignItems: 'center',
      display: 'flex',
      height: '100%',
      overflow: 'hidden',
    },
    cellWrapper: {
      paddingInline: 16,
    },
    cellWrapperNoReview: {
      width: '33%',
    },
    cellWrapperReview: {
      width: '20%',
    },
    row: {
      color: 'var(--primary-text)',
      display: 'flex',
      flexDirection: 'row',
      paddingBlock: 16,
    },
  }),
  stylex.create({
    cell: {
      paddingInline: 16,
    },
    cellNoReview: {
      width: '33%',
    },
    cellReview: {
      width: '20%',
    },
    row: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
      display: 'flex',
      flexDirection: 'row',
      paddingBlock: 16,
    },
  }),
  stylex.create({
    disabled: {
      color: 'var(--negative)',
    },
    enabled: {
      color: 'var(--positive)',
    },
    header: {
      display: 'flex',
      flexDirection: 'row',
      paddingBottom: 20,
    },
    headerButton: {
      marginInlineEnd: 16,
      marginTop: 16,
      width: '20%',
    },
    headerText: {
      width: '80%',
    },
    updated: {
      color: 'var(--warning)',
    },
  }),
  stylex.create({
    wrapper: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    selector: {
      marginInlineEnd: 12,
      minWidth: 232,
    },
    wrapper: {
      display: 'flex',
      marginTop: 24,
    },
  }),
  stylex.create({
    container: {
      marginInlineEnd: -16,
      marginInlineStart: -16,
    },
    module: {
      marginTop: 12,
    },
    root: {
      padding: 40,
    },
  }),
  stylex.create({
    scrollableArea: {
      maxHeight: 500,
    },
    stateLink: {
      padding: 16,
      width: '100%',
    },
  }),
  stylex.create({
    scrollableArea: {
      maxHeight: 500,
    },
  }),
  stylex.create({
    stateLink: {
      padding: 16,
      width: '100%',
    },
  }),
  stylex.create({
    cell: {
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
    },
    headerRow: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
    },
    lg: {
      width: 220,
    },
    md: {
      width: 180,
    },
    row: {
      alignItems: 'stretch',
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      flexDirection: 'row',
      paddingBottom: 16,
      paddingTop: 16,
    },
    sm: {
      width: 120,
    },
  }),
  stylex.create({
    button: {
      marginInlineStart: 12,
    },
    inlineItem: {
      display: 'inline-block',
      verticalAlign: 'middle',
    },
    monthRow: {
      textAlign: 'end',
      width: '30%',
    },
    row: {
      marginTop: 24,
    },
    selector: {
      display: 'inline-block',
      marginInlineEnd: 12,
    },
    selectorRow: {
      width: '70%',
    },
    wrapper: {
      paddingBottom: 24,
    },
  }),
  stylex.create({
    background: {
      backgroundColor: 'var(--wash)',
      height: '100%',
      opacity: 0.7,
      position: 'absolute',
      width: '100%',
    },
    modalContent: {
      padding: 16,
    },
    scrollArea: {
      maxHeight: 420,
      paddingBottom: 6,
      paddingTop: 6,
    },
    wrapper: {
      height: '100%',
      position: 'fixed',
      width: '100%',
      zIndex: 2,
    },
  }),
  stylex.create({
    backgroundHeaderRow: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
      height: 62,
      position: 'absolute',
      top: 0,
      width: '100%',
      zIndex: 6,
    },
    dataCol: {
      display: 'flex',
      flexWrap: 'nowrap',
    },
    downloadButton: {
      end: 8,
      position: 'absolute',
      top: 8,
      width: 120,
      zIndex: 6,
    },
    tableContent: {
      maxHeight: '70vh',
      position: 'relative',
    },
    wrapper: {
      backgroundColor: 'var(--surface-background)',
      borderRadius: 8,
      fontSize: 15,
      overflow: 'hidden',
      position: 'relative',
    },
  }),
  stylex.create({
    cell: {
      overflowWrap: 'break-word',
    },
    dataCell: {
      color: 'var(--always-black)',
      paddingBottom: 8,
      paddingTop: 16,
    },
    headingSeparator: {
      height: 32,
      marginBottom: 8,
    },
    pressable: {
      width: '100%',
    },
    promoType: {
      fontSize: '13px',
      fontWeight: 600,
      height: 42,
      marginBottom: 6,
      paddingInlineStart: 12,
    },
  }),
  stylex.create({
    afterCurrentDate: {
      color: 'var(--secondary-text)',
    },
    beforeCurrentDate: {
      color: 'var(--disabled-text)',
    },
    cell: {
      overflowWrap: 'break-word',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    column: {
      backgroundColor: 'var(--surface-background)',
      color: 'var(--primary-text)',
      flexBasis: '240px',
      flexGrow: 1,
      flexShrink: 0,
      maxWidth: '240px',
      minWidth: '240px',
      position: 'relative',
      scrollSnapAlign: 'start',
    },
    currentDate: {
      backgroundColor: 'var(--primary-button-background)',
      color: 'var(--always-white)',
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      textAlign: 'center',
      width: 18,
    },
    dataCell: {
      overflow: 'hidden',
      paddingBottom: 16,
      paddingTop: 16,
    },
    dateCell: {
      backgroundColor: 'var(--surface-background)',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
      display: 'flex',
      height: 30,
      position: 'sticky',
      top: 0,
      width: '100%',
      zIndex: 5,
    },
    headline: {
      height: 12,
      marginBottom: 4,
    },
    stickColumn: {
      borderInlineEndWidth: 1,
      borderInlineEndStyle: 'solid',
      borderInlineEndColor: 'var(--wash)',
      flexBasis: '200px',
      maxWidth: '200px',
      minWidth: '200px',
      position: 'sticky',
      start: 0,
      zIndex: 6,
    },
    subHeadline: {
      fontSize: '13px',
      height: 26,
      paddingTop: 4,
    },
    subHeadlineInner: {
      borderRadius: '100%',
      height: 18,
      paddingBottom: 4,
      paddingTop: 4,
    },
  }),
  stylex.create({
    cell: {
      overflowWrap: 'break-word',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    dataCell: {
      paddingBottom: 8,
      paddingTop: 16,
    },
    promoTypes: {
      color: 'var(--secondary-text)',
      fontSize: '13px',
      fontWeight: 600,
      paddingInlineStart: 16,
      paddingTop: 12,
    },
    stateName: {
      height: 32,
      marginBottom: 8,
    },
  }),
  stylex.create({
    learnMoreNotice: {
      color: 'var(--secondary-text)',
      padding: 16,
    },
    menuItems: {
      paddingBottom: 18,
      paddingTop: 18,
    },
    saveButton: {
      width: 100,
    },
    saveButtonDiv: {
      display: 'flex',
      justifyContent: 'flex-end',
      paddingBottom: 12,
      paddingInlineEnd: 16,
      paddingTop: 12,
    },
    verifiedNotice: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    banner: {
      marginTop: -4,
    },
    border: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      margin: '0 16px',
      paddingTop: 4,
    },
  }),
  stylex.create({
    addPhotoInnerContentIcon: {
      display: 'flex',
      justifyContent: 'center',
      paddingBottom: 3,
    },
    addPhotoInnerContentText: {
      paddingTop: 3,
      textAlign: 'center',
    },
    addPhotoOuterContent: {
      backgroundColor: 'var(--wash)',
      borderRadius: 5,
      display: 'flex',
      flexDirection: 'column',
      height: 100,
      justifyContent: 'center',
      position: 'relative',
      width: 100,
    },
  }),
  stylex.create({
    checkmark: {
      marginInlineStart: 1,
      marginTop: 1,
    },
    checkmarkCircle: {
      backgroundColor: 'var(--always-white)',
      borderRadius: 11,
      height: 22,
      position: 'absolute',
      start: 35,
      top: 35,
      width: 22,
      zIndex: 2,
    },
    divider: {
      backgroundColor: 'var(--divider)',
      height: 1,
    },
    headerComponent: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      marginInline: 24,
      marginBlock: 16,
    },
    headerComponentTextBlockSpacing: {
      height: 20,
    },
    icon: {
      zIndex: 1,
    },
    iconBar: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      marginBlock: 16,
    },
    iconCircle: {
      alignItems: 'center',
      backgroundColor: 'var(--base-cherry)',
      borderRadius: 27,
      display: 'flex',
      height: 54,
      justifyContent: 'center',
      position: 'relative',
      width: 54,
    },
    moreOptionsContainer: {
      cursor: 'pointer',
      display: 'flex',
      justifyContent: 'flex-start',
    },
    moreOptionsIcon: {
      display: 'flex',
      marginTop: 6,
      minWidth: 24,
    },
    moreOptionsPressableContent: {
      alignItems: 'center',
      display: 'inline-flex',
      height: 70,
      paddingInlineStart: 12,
    },
    moreOptionsTextBody: {
      marginTop: 5,
    },
    moreOptionsTextColumn: {
      marginInlineStart: 12,
    },
    moreOptionsTextTitle: {
      marginBottom: 5,
    },
    otherActionsHeader: {
      marginTop: 24,
    },
    otherActionsHeaderText: {
      marginInlineStart: 16,
      marginBlock: 16,
    },
    pill: {
      backgroundColor: 'var(--base-cherry)',
      borderRadius: 50,
    },
    pillCheckmark: {
      paddingInlineEnd: 8,
      paddingInlineStart: 4,
      paddingTop: 4,
    },
    pillContents: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      height: 32,
      justifyContent: 'center',
      paddingInlineEnd: 12,
      paddingInlineStart: 8,
    },
    pressable: {
      width: '100%',
    },
  }),
  stylex.create({
    attachEvidenceDivider: {
      backgroundColor: 'var(--divider)',
      flexDirection: 'column',
      height: 1,
      marginBlock: 18,
    },
    attachEvidenceRow: {
      display: 'inline-flex',
      maxWidth: '100%',
      overflowX: 'scroll',
    },
    caption: {
      fontSize: 12,
      marginBottom: 8,
    },
    covid: {
      display: 'flex',
      flexDirection: 'column',
    },
    error: {
      fontSize: 12,
      marginTop: 12,
    },
    header: {
      fontSize: 15,
      fontWeight: 'bold',
      lineHeight: 20,
      paddingBottom: 10,
    },
    optionalEvidenceRoot: {
      position: 'relative',
    },
    optionalText: {
      end: 0,
      fontSize: 12,
      position: 'absolute',
      top: 0,
    },
    subtext: {
      marginTop: 8,
    },
  }),
  stylex.create({
    buttons: {
      paddingBlock: 2,
    },
    loadingOverlay: {
      alignItems: 'center',
      backgroundColor: 'var(--web-wash)',
      bottom: 0,
      display: 'flex',
      end: 0,
      justifyContent: 'center',
      opacity: 0.7,
      position: 'absolute',
      start: 0,
      top: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    subtitle: {
      backgroundColor: 'var(--web-wash)',
      padding: 12,
    },
    subtitleBodyText: {
      fontSize: 15,
      paddingBlock: 12,
    },
    subtitleHeaderText: {
      fontSize: 17,
      fontWeight: 'bold',
      paddingBottom: 2,
    },
    subtitleIcon: {
      marginBottom: 8,
    },
  }),
  stylex.create({
    pressedStyle: {
      backgroundColor: 'var(--accent)',
      borderRadius: 4,
      bottom: 2,
      end: 4,
      opacity: 0.1,
      pointerEvents: 'none',
      position: 'absolute',
      start: 4,
      top: 2,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    radio: {
      display: 'flex',
      marginInlineStart: 14,
    },
    radioBorder: {
      borderRadius: '50%',
      display: 'inline-block',
      flexShrink: 0,
      height: 20,
      position: 'relative',
      width: 20,
    },
    radioSelected: {
      backgroundColor: 'var(--accent)',
      borderRadius: '50%',
      height: 12,
      position: 'absolute',
      width: 12,
    },
    root: {
      WebkitTapHighlightColor: 'transparent',
      alignItems: 'center',
      display: 'flex',
      paddingBottom: 8,
      paddingTop: 8,
      position: 'relative',
      touchAction: 'manipulation',
    },
    selectedBorder: {
      borderWidth: 2,
      borderStyle: 'solid',
      borderColor: 'var(--accent)',
    },
    selectedSize: {
      start: 4,
      top: 4,
    },
    size: {
      height: 20,
      width: 20,
    },
    text: {
      display: 'flex',
      flexDirection: 'column',
      height: 64,
      justifyContent: 'center',
      marginInlineStart: 14,
    },
    textLine: {
      paddingBlock: 6,
    },
    unselectedBorder: {
      borderWidth: 2,
      borderStyle: 'solid',
      borderColor: 'var(--primary-icon)',
    },
  }),
  stylex.create({
    attachEvidenceSection: {
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    explanationText: {
      fontSize: 12,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 24,
    },
    loadingOverlay: {
      alignItems: 'center',
      backgroundColor: 'var(--web-wash)',
      bottom: 0,
      display: 'flex',
      end: 0,
      justifyContent: 'center',
      opacity: 0.7,
      position: 'absolute',
      start: 0,
      top: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    sectionPadding: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 24,
    },
  }),
  stylex.create({
    image: {
      display: 'flex',
    },
    imageContainer: {
      display: 'inline-block',
      height: 100,
      marginInlineEnd: 8,
      position: 'relative',
      width: 100,
    },
    imageDelete: {
      cursor: 'pointer',
      display: 'flex',
      justifyContent: 'flex-end',
      paddingInlineEnd: 3,
      paddingTop: 3,
    },
    overlay: {
      height: '100%',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    card: {
      marginBottom: 16,
      minHeight: 355,
      padding: 16,
    },
    innerStoryCard: {
      borderRadius: 16,
      height: 250,
      padding: 16,
    },
    storyCard: {
      height: 236,
      transform: 'scale(0.95)',
      transformOrigin: 'center center',
      transition: 'transform 200ms ease-out 50ms',
    },
    storyCardSelected: {
      transform: 'scale(1)',
    },
  }),
  stylex.create({
    button: {
      width: 125,
    },
  }),
  stylex.create({
    body: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      height: 200,
      justifyContent: 'space-between',
      padding: 16,
      paddingTop: 12,
    },
    cardRoot: {
      marginInlineEnd: 8,
    },
    coverPhoto: {
      backgroundColor: 'var(--media-pressed)',
      display: 'flex',
      flexGrow: 0,
      height: 150,
      overflow: 'hidden',
      width: '100%',
    },
    details: {
      paddingBottom: 16,
    },
    icon: {
      margin: 'auto',
    },
    pressable: {
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      minWidth: 300,
    },
    progressBar: {
      paddingBlock: 16,
    },
  }),
  stylex.create({
    card: {
      marginBottom: 16,
      padding: 16,
    },
    commentWrapper: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'space-between',
      margin: '24px 0 16px 0',
    },
    reactions: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
    },
    reactionsCount: {
      marginInlineStart: 4,
    },
    storiesContainer: {
      margin: '24px 0  0 -8px',
    },
    storyCard: {
      height: 244,
      padding: 16,
    },
    storyCardHeader: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'flex-start',
      padding: '0 0 24px 0',
    },
    storyCardIcon: {
      backgroundColor: 'var(--card-background)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: '50%',
      display: 'block',
      height: 60,
      marginInlineEnd: 8,
      width: 60,
    },
    storyContainer: {
      width: 270,
    },
  }),
  stylex.create({
    card: {
      margin: '16px',
    },
  }),
  stylex.create({
    backgroundImage: {
      display: 'block',
      height: '100%',
      objectFit: 'contain',
      position: 'absolute',
      start: 100,
      top: 64,
      width: '100%',
      zIndex: -1,
      '@media (max-width: 940px)': {
        objectFit: 'cover',
      },
    },
    buttons: {
      display: 'flex',
      paddingInline: 30,
      paddingBlock: 30,
    },
    content: {
      backgroundColor: '#FAE07C',
      borderBottomEndRadius: 8,
      borderBottomStartRadius: 8,
      display: 'flex',
      height: 348,
      marginTop: '-10px',
      maxWidth: '100%',
      overflow: 'hidden',
      width: '100%',
    },
    contentContainer: {
      alignItems: 'flex-end',
      display: 'flex',
      justifyContent: 'space-between',
      '@media (max-width: 899px)': {
        alignItems: 'start',
        flexDirection: 'column',
        justifyContent: 'flex-end',
        paddingTop: 8,
      },
      '@media (min-width: 836px)': {
        width: '100%',
      },
    },
    contentWrapper: {
      alignItems: 'flex-end',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      height: '100%',
      justifyContent: 'space-between',
      width: '100%',
      '@media (max-width: 836px)': {
        alignItems: 'flex-start',
        flexDirection: 'column',
        flexWrap: 'wrap',
      },
    },
    root: {
      backgroundImage:
        'linear-gradient(180deg, rgba(250, 224, 124, 0.5) 0%, rgba(250, 224, 124, 0) 100%)',
      height: 348,
      overflow: 'hidden',
      position: 'relative',
    },
    text: {
      alignItems: 'center',
      display: 'flex',
      maxWidth: 500,
      paddingInline: 30,
      paddingBlock: 50,
      '@media (max-width: 835px)': {
        marginTop: -8,
      },
    },
  }),
  stylex.create({
    cardRoot: {
      marginBottom: 20,
    },
    contentItem: {
      display: 'flex',
      flexDirection: 'row',
      marginBottom: 8,
      marginTop: 16,
      width: '95%',
    },
    contentRoot: {
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 28,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 20,
      width: '100%',
    },
    header: {
      marginTop: 16,
    },
    linkItem: {
      marginInlineStart: 30,
      marginTop: 10,
    },
    photo: {
      height: 15,
      marginInlineEnd: 16,
      width: 15,
    },
  }),
  stylex.create({
    button: {
      marginTop: 16,
      width: '100%',
    },
    cardRoot: {
      marginBottom: 8,
    },
    contentRoot: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 20,
    },
    header: {
      alignSelf: 'flex-start',
      height: 20,
      marginBottom: 4,
      marginTop: 16,
      paddingInlineStart: 4,
    },
    photo: {
      borderRadius: '6px',
      height: 120,
      objectFit: 'cover',
      width: '100%',
    },
    subHeader: {
      paddingInlineStart: 4,
    },
  }),
  stylex.create({
    card: {
      padding: 16,
    },
    playlistItem: {
      display: 'flex',
      flexDirection: 'row',
      height: 90,
      marginBottom: 12,
      ':last-child': {
        marginBottom: 0,
      },
    },
    playlistsContainer: {
      padding: '16px 0',
    },
    thumbnail: {
      borderRadius: 8,
      height: 90,
      marginInlineEnd: 8,
      objectFit: 'cover',
      width: 169,
    },
  }),
  stylex.create({
    body: {
      alignItems: 'stretch',
      display: 'flex',
      flexDirection: 'column',
      width: '100%',
    },
    button: {
      paddingTop: 16,
      width: '100%',
    },
    cardRoot: {
      marginBottom: 8,
    },
    contentRoot: {
      display: 'flex',
      flexDirection: 'column',
      maxWidth: 560,
      paddingBottom: 12,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 20,
    },
    header: {
      marginBottom: 16,
    },
    progressBar: {
      marginTop: 50,
    },
    question: {
      paddingBottom: 20,
      paddingTop: 40,
    },
    questionBody: {
      paddingInlineStart: 0,
    },
    quizResult: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      marginBottom: 38,
    },
    quizResultComment: {
      marginTop: 16,
    },
    quizResultPhoto: {
      marginBottom: 30,
    },
    quizResultScore: {
      marginBottom: 8,
    },
    result: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
    },
    resultAnotherSecondaryButton: {
      paddingInlineStart: 10,
      width: '49%',
    },
    resultSecondaryButton: {
      width: '49%',
    },
    resultSecondaryButtons: {
      display: 'flex',
      flexDirection: 'row',
      paddingTop: 12,
      width: '100%',
    },
  }),
  stylex.create({
    cardBottomMargin: {
      marginBottom: 16,
    },
    container: {
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      paddingTop: 16,
    },
    feed: {
      maxWidth: '100%',
    },
  }),
  stylex.create({
    button: {
      width: 40,
    },
  }),
  stylex.create({
    card: {
      marginBottom: 16,
      padding: 16,
    },
    hscroll: {
      height: '100%',
      marginTop: 8,
    },
    storiesContainer: {
      margin: '24px 0  0 0',
    },
  }),
  stylex.create({
    cardRoot: {
      marginBottom: 8,
    },
    contentRoot: {
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 16,
      paddingTop: 20,
    },
    divider: {
      maxWidth: 630,
      paddingInlineEnd: 16,
      paddingInlineStart: 26,
    },
    hscroll: {
      paddingInlineEnd: 16,
    },
    nextSectionButton: {
      marginTop: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 66,
      width: '80%',
    },
    takeQuizButton: {
      marginTop: 20,
      paddingInlineEnd: 16,
      paddingInlineStart: 66,
      width: '80%',
    },
    title: {
      marginBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    image: {
      height: '131',
      width: '131',
    },
  }),
  stylex.create({
    root: {
      alignItems: 'flex-start',
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      marginBottom: 12,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    video: {
      height: 281,
    },
  }),
  stylex.create({
    root: {
      alignItems: 'flex-start',
      display: 'flex',
      flexDirection: 'column',
    },
    title: {
      marginBottom: 12,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      width: 200,
    },
    video: {
      height: 281,
      width: '100%',
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    defaultAnchor: {
      minHeight: 675,
      paddingBlock: 12,
      '@media (max-width: 564px)': {
        paddingBlock: 0,
      },
    },
  }),
  stylex.create({
    defaultAnchor: {
      minHeight: 675,
      paddingBlock: 12,
      '@media (max-width: 564px)': {
        paddingBlock: 0,
      },
    },
  }),
  stylex.create({
    footer: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--media-inner-border)',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      paddingBottom: 16,
    },
    stretchButtons: {
      flexGrow: 1,
    },
  }),
  stylex.create({
    card: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    headline: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    description: {
      marginTop: 8,
    },
  }),
  stylex.create({
    actions: {
      marginTop: 8,
      paddingInlineStart: 4,
    },
    card: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    headline: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    body: {
      padding: 16,
    },
  }),
  stylex.create({
    card: {
      padding: 16,
    },
  }),
  stylex.create({
    attachmentArea: {
      padding: '0 8px',
    },
    error: {
      marginInlineEnd: 16,
      marginInlineStart: 16,
    },
    postButton: {
      alignItems: 'center',
      boxSizing: 'border-box',
      display: 'flex',
      justifyContent: 'space-between',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      width: '100%',
    },
    postButtonWithToolbar: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    toolbarLabel: {
      flexGrow: 1,
      padding: 8,
    },
  }),
  stylex.create({
    image: {
      borderRadius: 8,
    },
    row: {
      backgroundColor: 'var(--nav-bar-background)',
      boxShadow: '0 1px 2px var(--shadow-1)',
      flexShrink: 0,
    },
  }),
  stylex.create({
    card: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    faqList: {
      marginTop: 8,
    },
    headline: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    card: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    headline: {
      marginBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    card: {
      paddingBottom: 8,
      paddingTop: 16,
    },
    headline: {
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    button: {
      marginTop: 8,
      width: 120,
    },
    container: {
      padding: 8,
      width: '100%',
    },
    image: {
      borderRadius: 8,
    },
    pressable: {
      marginInlineEnd: 8,
      marginInlineStart: 8,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
    },
  }),
  stylex.create({
    card: {
      paddingBottom: 8,
      paddingTop: 16,
    },
    headline: {
      paddingBottom: 8,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    column: {
      margin: '0 auto',
      maxWidth: 980,
      padding: '15px 20px 48px',
      width: '100%',
    },
    darkContainer: {
      color: 'var(--primary-text)',
    },
  }),
  stylex.create({
    wrapperDiv: {
      display: 'inline-block',
    },
  }),
  stylex.create({
    videoContainer: {
      marginInline: 'auto',
    },
  }),
  stylex.create({
    container: {
      maxWidth: 400,
      minWidth: 400,
      position: 'relative',
    },
    headerStyle: {
      marginTop: 15,
    },
  }),
  stylex.create({
    above: {
      marginBottom: 8,
    },
    below: {
      marginTop: 8,
    },
    end: {
      marginInlineStart: 8,
    },
    start: {
      marginInlineEnd: 8,
    },
  }),
  stylex.create({
    mask: {
      height: '100vh',
      position: 'fixed',
      start: 0,
      top: 0,
      width: '100vw',
      zIndex: 2,
    },
    maskOverlay: {
      fill: 'var(--overlay-alpha-80)',
    },
  }),
  stylex.create({
    outerElement: {
      position: 'relative',
      zIndex: 4,
    },
  }),
  stylex.create({
    root: {
      padding: 16,
    },
  }),
  stylex.create({
    body: {
      padding: '20px 0',
    },
    bullet: {
      marginBottom: '4px',
    },
    bullets: {
      listStyleType: 'disc',
      marginInlineStart: '16px',
    },
    card: {
      padding: '16px',
    },
    footerList: {
      margin: '0 -16px',
    },
    footerTitle: {
      marginBottom: '12px',
    },
    paragraph: {
      paddingBottom: '20px',
    },
  }),
  stylex.create({
    card: {
      padding: '16px',
    },
  }),
  stylex.create({
    card: {
      padding: '16px',
    },
    header: {
      display: 'flex',
      flexDirection: 'row',
      height: 36,
      padding: 12,
    },
  }),
  stylex.create({
    button: {
      marginInlineStart: '12px',
    },
    buttonGroup: {
      display: 'flex',
      flexDirection: 'row',
      margin: '16px',
    },
    cardBody: {
      marginBottom: '20px',
      padding: '4px 12px',
    },
    footerContainer: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--media-inner-border)',
      display: 'flex',
      justifyContent: 'flex-end',
    },
  }),
  stylex.create({
    circle: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      display: 'flex',
      height: '28px',
      justifyContent: 'center',
      width: '28px',
    },
  }),
  stylex.create({
    footer: {
      paddingInline: 16,
      paddingTop: 16,
    },
    groupContext: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    footer: {
      paddingInline: 16,
      paddingTop: 16,
    },
    footerDivider: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
    },
  }),
  stylex.create({
    toastMessage: {
      maxWidth: 300,
    },
  }),
  stylex.create({
    dialogBody: {
      maxHeight: '75vh',
    },
  }),
  stylex.create({
    dialogBody: {
      maxHeight: '75vh',
    },
  }),
  stylex.create({
    cardBig: {
      flexGrow: 6,
      width: 475,
    },
    cardContent: {
      display: 'flex',
      minWidth: 200,
      padding: 16,
    },
    cardSmall: {
      flexGrow: 4,
      width: 325,
    },
    coverPhoto: {
      borderRadius: 4,
      height: 160,
      width: 325,
    },
    innerContent: {
      minWidth: 500,
      width: 876,
    },
    listItem: {
      marginTop: 0,
      width: 475,
    },
  }),
  stylex.create({
    cardBig: {
      flexGrow: 6,
      width: 471,
    },
    cardContent: {
      display: 'flex',
      minHeight: 200,
      minWidth: 200,
      padding: 16,
    },
    cardSmall: {
      flexGrow: 4,
      width: 325,
    },
    root: {
      minWidth: 500,
      width: 876,
    },
  }),
  stylex.create({
    button: {
      paddingInline: '10px',
      paddingTop: '20px',
      paddingBlock: '16px',
    },
    image: {
      height: 256,
      width: 500,
    },
    info: {
      paddingInline: '12px',
      paddingBlock: '16px',
    },
  }),
  stylex.create({
    button: {
      paddingBlock: '12px',
    },
    image: {
      height: 256,
      width: 500,
    },
    info: {
      paddingInline: '16px',
      paddingBlock: '0px',
    },
    loadingOverlay: {
      alignItems: 'center',
      bottom: 0,
      display: 'flex',
      end: 0,
      height: '100%',
      justifyContent: 'center',
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 999999,
    },
    spinner: {
      position: 'relative',
      start: -30,
      top: -30,
    },
  }),
  stylex.create({
    buttonRow: {
      marginBottom: 8,
    },
    card: {
      marginBottom: '16px',
      minHeight: 370,
      paddingInlineEnd: '16px',
      width: 876,
    },
    leftImage: {
      height: 355,
      width: 220,
    },
  }),
  stylex.create({
    button: {
      paddingInline: '16px',
      paddingBlock: '12px',
    },
    info: {
      paddingInline: '6px',
    },
  }),
  stylex.create({
    button: {
      paddingInline: '16px',
      paddingBlock: '12px',
    },
    createCatalog: {
      paddingInline: '8px',
      paddingBlock: '12px',
    },
    info: {
      paddingInline: '6px',
    },
  }),
  stylex.create({
    info: {
      padding: '0pt 16px 20px 16px',
    },
  }),
  stylex.create({
    info: {
      padding: '0pt 16px 20px 16px',
    },
  }),
  stylex.create({
    info: {
      padding: '0pt 16px 20px 16px',
    },
  }),
  stylex.create({
    button: {
      paddingInline: '16px',
      paddingBlock: '12px',
    },
    checkBox: {
      marginTop: '12px',
      paddingInline: '16px',
    },
    helpDialog: {
      marginTop: '4px',
      paddingInline: '64px',
    },
    info: {
      padding: '0pt 16px 0px 16px',
    },
  }),
  stylex.create({
    buttonRow: {
      paddingBlock: '12px',
    },
    image: {
      height: '256px',
      width: '500px',
    },
    info: {
      padding: '0pt 12px 16px 16px',
    },
  }),
  stylex.create({
    button: {
      paddingTop: '20px',
      paddingBlock: '12px',
    },
    image: {
      height: 256,
      width: 500,
    },
    info: {
      padding: '0pt 12px 16px 16px',
    },
  }),
  stylex.create({
    actionListSection: {
      paddingBottom: '16px',
    },
    image: {
      height: 256,
      width: 500,
    },
    info: {
      padding: '0pt 12px 16px 16px',
    },
    sectionHeader: {
      padding: '8px 0px 12px 16px',
    },
  }),
  stylex.create({
    content: {
      marginTop: 50,
    },
  }),
  stylex.create({
    container: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    sectionCard: {
      flexGrow: 4,
      padding: 16,
      width: 325,
    },
  }),
  stylex.create({
    container: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    sectionCard: {
      flexGrow: 4,
      padding: 16,
      width: 325,
    },
  }),
  stylex.create({
    body: {
      marginBottom: 32,
      marginInline: 20,
      marginTop: 18,
    },
    container: {
      maxWidth: 500,
    },
    cover: {
      backgroundColor: 'var(--fds-spectrum-teal-dark-1)',
      height: 220,
    },
    footer: {
      marginBlock: 5,
    },
  }),
  stylex.create({
    tagBoxContainer: {
      pointerEvents: 'all',
      position: 'absolute',
    },
  }),
  stylex.create({
    image: {},
    imageContainer: {
      alignItems: 'center',
      display: 'flex',
      height: 120,
      justifyContent: 'center',
      width: 120,
    },
    imageLinkActive: {
      backgroundColor: 'var(--toggle-active-background)',
      display: 'block',
      opacity: 0.9,
      textDecoration: 'none',
    },
    inset: {
      bottom: 0,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      height: 1,
      position: 'absolute',
      width: '100%',
    },
    nameContainer: {
      marginBottom: 10,
    },
    priceContainer: {
      alignItems: 'center',
      display: 'flex',
    },
    rootContainer: {
      width: 120,
    },
    rootLiveShoppingContainer: {
      marginInlineStart: 4,
      width: 120,
    },
    textContainer: {
      height: 44,
      padding: '14px 12px',
    },
  }),
  stylex.create({
    dotHint: {
      backgroundClip: 'padding-box',
      backgroundColor: 'var(--always-white)',
      borderRadius: '50%',
    },
    dotHintContainer: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
    },
    dotHintWithIcon: {
      backgroundColor: 'var(--fds-black-alpha-20)',
      borderRadius: '50%',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      flexGrow: 1,
      padding: 8,
    },
    deleteButton: {
      alignSelf: 'flex-start',
    },
    highlighted: {
      backgroundColor: 'var(--toggle-active-background)',
    },
    perUnitPrice: {
      marginInlineStart: 4,
    },
    productName: {
      flexGrow: 1,
      marginBottom: 4,
      paddingBlock: 4,
    },
    productPrice: {
      display: 'flex',
      flexDirection: 'row',
      marginBlock: 4,
      paddingBottom: 4,
    },
    profileImage: {
      borderRadius: 8,
    },
    rightContent: {
      flexGrow: 1,
      marginInlineStart: 12,
    },
    rootLayout: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
    },
  }),
  stylex.create({
    noPointerEvents: {
      pointerEvents: 'none',
    },
    tagLayer: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
  }),
  stylex.create({
    crossButton: {
      end: 8,
      position: 'absolute',
      top: 8,
    },
    exclusiveTag: {
      bottom: 4,
      position: 'absolute',
      start: 4,
    },
    exclusiveText: {
      backgroundColor: 'var(--shadow-8)',
      color: 'var(--always-white)',
      fontSize: 12,
      fontWeight: 600,
      padding: '2px 6px 2px 6px',
    },
    image: {
      objectFit: 'contain',
    },
    imageContainer: {
      display: 'flex',
      height: 120,
      position: 'relative',
      width: 120,
    },
    imageLinkActive: {
      backgroundColor: 'var(--toggle-active-background)',
      display: 'block',
      opacity: 0.9,
      textDecoration: 'none',
    },
    inset: {
      bottom: 0,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      height: 1,
      position: 'absolute',
      width: '100%',
    },
    nameContainer: {
      marginBottom: 10,
    },
    perUnitPriceContainer: {
      paddingTop: 6,
    },
    priceContainer: {
      alignItems: 'center',
      display: 'flex',
    },
    rootContainer: {
      width: 120,
    },
    rootLiveShoppingContainer: {
      marginInlineStart: 4,
      width: 120,
    },
    tallTextContainer: {
      height: 60,
    },
    textContainer: {
      height: 44,
      padding: '14px 12px',
    },
    timestampContainer: {
      marginBottom: 10,
      marginInline: 10,
    },
  }),
  stylex.create({
    image: {
      objectFit: 'cover',
    },
    imageContainer: {
      height: 120,
      position: 'relative',
      width: 120,
    },
    imageLink: {
      display: 'block',
      ':hover': {
        textDecoration: 'none',
      },
    },
    inset: {
      bottom: 0,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      height: 1,
      position: 'absolute',
      width: '100%',
    },
    rootContainer: {
      width: 120,
    },
    rootLiveShoppingContainer: {
      marginInlineStart: 4,
      width: 120,
    },
    tallTextContainer: {
      height: 60,
    },
    textContainer: {
      height: 44,
      padding: '14px 12px',
    },
  }),
  stylex.create({
    dotHintContainer: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      padding: 8,
    },
  }),
  stylex.create({
    rootContainer: {
      height: '100%',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    arrowLeft: {
      borderBottomWidth: 5,
      borderBottomStyle: 'solid',
      borderBottomColor: 'transparent',
      borderInlineEndWidth: 5,
      borderInlineEndStyle: 'solid',
      borderInlineEndColor: 'var(--shadow-8)',
      borderTopWidth: 5,
      borderTopStyle: 'solid',
      borderTopColor: 'transparent',
      height: 0,
      marginInlineStart: 6,
      width: 0,
    },
    iconContainer: {
      paddingInlineEnd: 8,
    },
    pill: {
      alignItems: 'center',
      backgroundColor: 'var(--shadow-8)',
      borderRadius: 17.5,
      bottom: 0,
      boxShadow: '0 8px 20px 0 var(--fds-black-alpha-30), 0 2px 4px 0 var(--fds-black-alpha-10)',
      cursor: 'pointer',
      display: 'flex',
      height: 35,
      marginBottom: 12,
      marginInlineStart: 12,
      padding: '0 16px',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
    },
    pillWithControlsVisible: {
      bottom: 58,
    },
    reminderIconContainer: {
      backgroundColor: 'var(--shadow-8)',
      borderRadius: 17.5,
      padding: 8,
    },
    reminderPill: {
      backgroundColor: 'transparent',
      boxShadow: 'none',
      marginInlineStart: 0,
    },
    reminderTextContainer: {
      backgroundColor: 'var(--shadow-8)',
      borderRadius: 6,
      padding: 8,
    },
  }),
  stylex.create({
    overlayOnTopContainer: {
      height: '18%',
    },
    rootContainer: {
      height: '100%',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    rootContainer: {
      height: '100%',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    pill: {
      alignItems: 'center',
      backgroundColor: 'var(--always-dark-overlay)',
      borderRadius: 14,
      bottom: 0,
      boxShadow: '0 8px 20px 0 var(--always-dark-overlay), 0 2px 4px 0 var(--shadow-1)',
      cursor: 'pointer',
      display: 'flex',
      height: 28,
      marginBottom: 10,
      marginInlineStart: 8,
      padding: '0 8px',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
    },
  }),
  stylex.create({
    button: {
      paddingBottom: 20,
    },
    headerText: {
      marginBottom: 12,
      marginInline: 20,
      marginTop: 18,
    },
    number: {
      alignItems: 'center',
      backgroundColor: 'var(--wash)',
      borderRadius: 32,
      display: 'flex',
      height: 40,
      justifyContent: 'center',
      width: 40,
    },
    numberContainer: {
      display: 'flex',
      flexDirection: 'column',
    },
    numberedComponent: {
      alignItems: 'center',
      display: 'flex',
      marginInlineEnd: 24,
      paddingBottom: 10,
      paddingInline: 24,
      paddingBlock: 10,
    },
    text: {
      alignItems: 'center',
      flexDirection: 'row',
      marginInlineEnd: 12,
      paddingInline: 12,
    },
  }),
  stylex.create({
    hscrollContainer: {
      marginInline: -14,
      paddingBottom: 16,
    },
    titleContainer: {
      paddingBottom: 16,
      paddingTop: 2,
    },
  }),
  stylex.create({
    acceptButton: {
      marginTop: 8,
    },
    aiSuggestionContainer: {
      marginBottom: 8,
    },
    aiSuggestionText: {
      marginBottom: 8,
    },
    hscrollContainer: {
      marginInline: 10,
      paddingBottom: 16,
    },
    titleContainer: {
      paddingBottom: 16,
      paddingTop: 2,
    },
  }),
  stylex.create({
    backButton: {
      alignItems: 'flex-start',
      justifyContent: 'center',
      marginInlineEnd: 20,
      start: 16,
    },
    title: {
      alignSelf: 'center',
      flexBasis: 0,
      flexGrow: 1,
      flexShrink: 1,
      marginInlineEnd: 30,
      minWidth: 0,
      overflow: 'hidden',
      textAlign: 'center',
      textOverflow: 'ellipsis',
      whiteSpace: 'nowrap',
      width: 380,
    },
  }),
  stylex.create({
    deleteTimeRange: {
      marginInlineStart: 12,
      marginTop: 20,
    },
    divider: {
      justifyContent: 'center',
      marginTop: 20,
      paddingInline: 4,
    },
    inputBox: {
      width: '100%',
    },
    timeStampInput: {
      alignItems: 'start',
      display: 'flex',
      marginBlock: 12,
    },
  }),
  stylex.create({
    addHighlight: {
      marginInlineStart: 70,
      marginTop: 12,
      paddingInlineStart: 12,
    },
    divider: {
      marginBlock: 16,
    },
    highlightIntroBody: {
      paddingBottom: 4,
      paddingTop: 12,
    },
  }),
  stylex.create({
    deleteTag: {
      float: 'end',
    },
    hideDeleteTag: {
      display: 'none',
    },
    productDetails: {
      paddingInlineEnd: 8,
      width: 218,
    },
    productImage: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
    },
    sellerName: {
      marginInlineStart: 12,
      marginBlock: 4,
    },
    tokenContainerWithTimeRanges: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 8,
      marginInlineEnd: 8,
      marginTop: 12,
      paddingInline: 12,
      paddingBlock: 12,
    },
    videoProductTaggingTokenContainer: {
      alignItems: 'start',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      height: 75,
    },
  }),
  stylex.create({
    gridItemImage: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
    },
    item: {
      marginBottom: 3,
    },
  }),
  stylex.create({
    list: {
      marginInlineStart: 20,
    },
  }),
  stylex.create({
    toggleIcon: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 6,
      padding: 8,
    },
    toggleIconContainer: {
      padding: 0,
    },
  }),
  stylex.create({
    container: {
      height: 78,
      position: 'relative',
      width: 78,
    },
    firstStackImage: {
      start: 4,
      top: 4,
      zIndex: -1,
    },
    secondStackImage: {
      start: 8,
      top: 8,
      zIndex: -2,
    },
    stackImage: {
      backgroundColor: 'var(--always-white)',
      height: 70,
      position: 'absolute',
      width: 70,
    },
  }),
  stylex.create({
    clickableItem: {
      borderRadius: 8,
      width: '50%',
    },
    container: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      height: 290,
      marginInline: 8,
      marginBlock: 10,
    },
    icon: {
      backgroundColor: 'var(--media-inner-border)',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      borderTopEndRadius: 8,
      borderTopStartRadius: 8,
      padding: 92,
    },
    text: {
      alignItems: 'center',
      display: 'flex',
      paddingInline: 10,
      paddingTop: 12,
    },
  }),
  stylex.create({
    clickableItem: {
      borderRadius: 8,
      width: '100%',
    },
    container: {
      display: 'flex',
      marginBlock: 10,
      paddingInlineEnd: 16,
      paddingInlineStart: 8,
    },
    icon: {
      borderWidth: 1,
      borderStyle: 'dashed',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      padding: 24,
    },
    text: {
      alignItems: 'center',
      display: 'flex',
      marginInlineEnd: 20,
      marginInlineStart: 10,
      paddingBlock: 4,
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--surface-background)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      width: 500,
    },
    scrollView: {
      marginInlineEnd: -16,
    },
    searchHeader: {
      marginInlineEnd: 8,
      marginInlineStart: 8,
      paddingBottom: 8,
    },
    typeahead: {
      justifyContent: 'space-between',
      overflow: 'hidden',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 12,
      width: '100%',
    },
    typeaheadHeader: {
      alignItems: 'center',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      boxSizing: 'border-box',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      overflow: 'hidden',
      width: '100%',
    },
    typeaheadItemImage: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
    },
    videoTypeaheadInput: {
      alignItems: 'center',
      display: 'block',
    },
    videoTypeaheadView: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 8,
      boxShadow: '0 12px 12px var(--shadow-2), inset 0 0 0 1px var(--shadow-inset)',
      marginInline: 16,
      marginTop: -3,
      maxHeight: 512,
      width: 328,
    },
  }),
  stylex.create({
    discoverProductsImage: {
      margin: 19,
    },
    discoverProductsImageContainer: {
      backgroundColor: 'var(--web-wash)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: '50%',
    },
    imageContainer: {
      height: 70,
      width: 70,
    },
    sellerProfilePicPlaceHolder: {
      backgroundClip: 'padding-box',
      backgroundColor: 'var(--always-gray-40)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: '50%',
    },
    wishListImagePadding: {
      padding: 5,
    },
  }),
  stylex.create({
    photoTypeaheadHeight: {
      maxHeight: 323,
    },
    typeaheadList: {
      marginInlineEnd: 16,
      marginTop: 8,
      minHeight: 0,
    },
    videoTypeaheadHeight: {
      maxHeight: 420,
    },
    videotypeaheadList: {
      paddingInline: 8,
    },
  }),
  stylex.create({
    sectionHeaderMargin: {
      marginInline: 6,
      marginBlock: 20,
    },
  }),
  stylex.create({
    gridItemContainer: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      height: 290,
    },
    gridItemImage: {
      borderTopEndRadius: 8,
      borderTopStartRadius: 8,
    },
    gridItemText: {
      paddingInline: 10,
      paddingTop: 10,
    },
    listViewText: {
      marginInlineStart: 8,
    },
    originalPrice: {
      color: 'var(--secondary-text)',
      marginInlineStart: 4,
      textDecoration: 'line-through',
    },
    perUnitPrice: {
      marginInlineStart: 4,
    },
  }),
  stylex.create({
    typeaheadList: {
      marginInlineEnd: 16,
      maxHeight: 320,
      minHeight: 0,
      paddingBottom: 8,
    },
    videotypeaheadList: {
      paddingInline: 8,
    },
  }),
  stylex.create({
    iconStart: {
      position: 'absolute',
      start: 12,
      top: 18,
    },
    root: {
      alignItems: 'center',
      color: 'var(--primary-text)',
      fontSize: 15,
      position: 'relative',
      width: '100%',
    },
    textInput: {
      backgroundColor: 'var(--card-background)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--divider)',
      borderRadius: 8,
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      fontSize: 'inherit',
      fontWeight: 500,
      height: 55,
      padding: '0 18px 0 40px',
      width: '100%',
    },
  }),
  stylex.create({
    backIcon: {
      marginInlineEnd: -2,
      marginTop: 6,
    },
    container: {
      display: 'flex',
      marginInline: 8,
      marginTop: 18,
      overflow: 'hidden',
      width: '100%',
    },
    itemRoot: {
      flexGrow: 1,
      marginInlineEnd: 16,
      overflow: 'hidden',
      paddingBottom: 4,
    },
    text: {
      alignItems: 'center',
      display: 'flex',
      textOverflow: 'ellipsis',
    },
  }),
  stylex.create({
    incentivesContainer: {
      marginBottom: 16,
    },
    titleContainer: {
      paddingBottom: 16,
      paddingTop: 16,
    },
    titleWithShoppingCartContainer: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      justifyContent: 'space-between',
    },
  }),
  stylex.create({
    body: {
      marginBottom: 32,
      marginInline: 20,
      marginTop: 18,
    },
    container: {
      maxWidth: 500,
    },
    cover: {
      backgroundColor: 'var(--fds-spectrum-teal-dark-1)',
      height: 220,
    },
    footer: {
      marginBottom: 12,
    },
  }),
  stylex.create({
    adminMessageText: {
      alignItems: 'flex-start',
      display: 'flex',
      marginInlineEnd: 16,
      marginInlineStart: 16,
      paddingTop: 16,
    },
    buttonGroup: {
      paddingTop: 16,
    },
    headerBodyText: {
      alignItems: 'flex-start',
      display: 'flex',
      marginInlineStart: 16,
      paddingBottom: 16,
    },
    headerImage: {
      alignItems: 'flex-start',
      display: 'flex',
      height: 308,
      justifyContent: 'center',
      overflow: 'hidden',
    },
  }),
  stylex.create({
    bodyText: {
      alignItems: 'flex-start',
      display: 'flex',
      marginInlineStart: 16,
      paddingBottom: 16,
      paddingTop: 16,
    },
    bottomText: {
      alignItems: 'flex-start',
      display: 'flex',
      justifyContent: 'center',
      paddingBottom: 16,
    },
    buttonStyle: {
      marginInlineEnd: 16,
      marginInlineStart: 16,
      paddingBottom: 12,
      paddingTop: 16,
    },
    divider: {
      marginInlineEnd: 16,
      marginInlineStart: 60,
    },
    headerImage: {
      alignItems: 'flex-start',
      display: 'flex',
      height: 308,
      justifyContent: 'center',
      overflow: 'hidden',
    },
    headerText: {
      alignItems: 'flex-start',
      display: 'flex',
      marginInlineStart: 16,
      paddingTop: 16,
    },
    optionStyle: {
      marginInlineEnd: 16,
      marginInlineStart: 16,
    },
    websitePrompt: {
      paddingTop: 48,
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 7,
      height: 14,
      marginBottom: 14,
    },
    bodyGlimmerContainer: {
      padding: '20px 20px 150px 20px',
    },
    bodyGlimmerFirst: {
      width: '80%',
    },
    bodyGlimmerSecond: {
      width: '40%',
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
    },
    header: {
      alignItems: 'center',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      textAlign: 'center',
    },
    headerGlimmer: {
      borderRadius: 7,
      height: 14,
      width: 100,
    },
  }),
  stylex.create({
    cancelResult: {
      padding: '16px',
    },
    cancelTitleSection: {
      paddingTop: '28px',
    },
  }),
  stylex.create({
    orderItem: {
      alignItems: 'center',
      backgroundColor: 'var(--card-background)',
      borderRadius: '8px',
      boxSizing: 'border-box',
      display: 'flex',
      justifyContent: 'space-between',
      paddingTop: '16px',
    },
  }),
  stylex.create({
    root: {
      paddingBottom: 16,
      paddingInline: 16,
    },
  }),
  stylex.create({
    errorContainer: {
      padding: '20px',
      textAlign: 'center',
      width: '550px',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      boxSizing: 'border-box',
      display: 'flex',
      justifyContent: 'flex-start',
      paddingBottom: '4px',
      paddingTop: '16px',
      width: '500px',
    },
    debugId: {
      marginInlineEnd: '10px',
      width: '200px',
    },
  }),
  stylex.create({
    detailsButton: {
      paddingTop: 8,
      width: 'fit-content',
    },
    offerButton: {
      marginTop: -8,
      width: 'fit-content',
    },
  }),
  stylex.create({
    root: {
      marginTop: 8,
    },
  }),
  stylex.create({
    container: {
      marginBottom: 8,
    },
  }),
  stylex.create({
    root: {
      margin: '0 auto',
    },
  }),
  stylex.create({
    bulletListItem: {
      paddingInlineStart: 6,
    },
    element: {
      paddingInlineStart: 12,
    },
    firstElement: {
      paddingInlineStart: 14,
    },
    flexLine: {
      display: 'flex',
      flexDirection: 'row',
    },
    listLine: {
      paddingInlineEnd: 16,
      paddingInlineStart: 8,
      paddingBlock: 8,
    },
    number: {
      paddingInlineStart: 8,
      width: 'auto',
    },
    paragraph: {
      paddingInline: 16,
      paddingBlock: 8,
    },
  }),
  stylex.create({
    bulletListItem: {
      paddingInlineStart: 6,
    },
    element: {
      paddingInlineStart: 12,
    },
    firstElement: {
      paddingInlineStart: 14,
    },
    flexLine: {
      display: 'flex',
      flexDirection: 'row',
    },
    listLine: {
      paddingInlineEnd: 16,
      paddingInlineStart: 8,
      paddingBlock: 8,
    },
    number: {
      paddingInlineStart: 8,
      width: 'auto',
    },
    paragraph: {
      paddingInline: 16,
      paddingBlock: 8,
    },
  }),
  stylex.create({
    message: {
      marginTop: 8,
    },
    messageContainerDefault: {
      marginTop: 8,
    },
  }),
  stylex.create({
    container: {
      paddingInline: 16,
    },
    messageBody: {
      marginTop: '8px',
    },
  }),
  stylex.create({
    messageContainer: {
      marginTop: 0,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      padding: '16px',
    },
    contentsComposerSection: {
      marginTop: '16px',
    },
    hairline: {
      backgroundColor: 'var(--divider)',
      height: 1,
      marginBottom: 4,
      marginTop: 16,
    },
  }),
  stylex.create({
    button: {
      alignSelf: 'flex-end',
      marginTop: '20px',
    },
  }),
  stylex.create({
    issueHeading: {
      marginTop: 20,
    },
    issueTitle: {
      marginBottom: 8,
    },
    messageComposer: {
      borderRadius: 6,
      marginTop: 26,
    },
  }),
  stylex.create({
    container: {
      paddingInline: 16,
    },
    messageBody: {
      marginTop: '8px',
    },
  }),
  stylex.create({
    contentContainer: {
      paddingInline: 16,
    },
    messageBoxSubtitle: {
      marginTop: 8,
    },
  }),
  stylex.create({
    imageAndText: {
      alignItems: 'center',
      display: 'flex',
    },
    itemImage: {
      borderRadius: '8px',
      marginInlineEnd: '12px',
    },
    itemInfo: {
      display: 'flex',
      flexDirection: 'column',
    },
    itemInfoText: {
      marginBottom: '8px',
    },
  }),
  stylex.create({
    contentContainer: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    contentsComposerSection: {
      marginTop: 16,
      paddingInline: 16,
    },
    optionsButtons: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    buttonContainer: {
      alignSelf: 'flex-end',
      display: 'flex',
      flexDirection: 'row',
      marginTop: 16,
      paddingInline: 16,
    },
    secondaryButton: {
      marginInline: 16,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      paddingBlock: 16,
    },
    hairline: {
      backgroundColor: 'var(--divider)',
      height: 1,
      marginBottom: 4,
      marginTop: 16,
    },
  }),
  stylex.create({
    buttonContainer: {
      alignSelf: 'flex-end',
      display: 'flex',
      flexDirection: 'row',
      marginTop: 16,
      paddingInline: 16,
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      paddingBlock: 16,
    },
    contentsComposerSection: {
      marginTop: 16,
    },
    form: {
      paddingInline: 16,
    },
    hairline: {
      backgroundColor: 'var(--divider)',
      height: 1,
      marginBottom: 4,
      marginTop: 16,
    },
    header: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    disclaimer: {
      paddingBottom: 20,
      paddingInline: 16,
    },
  }),
  stylex.create({
    container: {
      paddingInline: 16,
    },
    messageBody: {
      marginTop: '8px',
    },
  }),
  stylex.create({
    container: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    contentsComposerSection: {
      marginTop: 16,
      paddingInline: 16,
    },
    optionsButtons: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    container: {
      paddingBottom: 16,
    },
    readMore: {
      paddingInline: 16,
      paddingTop: 16,
    },
    text: {
      paddingBottom: 8,
      paddingInline: 16,
      paddingTop: 20,
    },
  }),
  stylex.create({
    orderSummary: {
      marginTop: 18,
    },
    row: {
      display: 'flex',
      justifyContent: 'space-between',
      whiteSpace: 'nowrap',
      width: '100%',
    },
    rowSpacing: {
      paddingTop: 14,
    },
  }),
  stylex.create({
    content: {
      alignItems: 'center',
      flexDirection: 'row',
      flexWrap: 'wrap',
      paddingBottom: 16,
      paddingInline: 16,
    },
    footer: {
      paddingBottom: 20,
      paddingInline: 16,
    },
    trackingNumber: {
      marginTop: 4,
    },
  }),
  stylex.create({
    header: {
      paddingBottom: 12,
      paddingInline: 16,
    },
  }),
  stylex.create({
    bulletList: {
      listStyleType: 'disc',
      marginInlineStart: 30,
    },
  }),
  stylex.create({
    commsMessages: {
      marginTop: 8,
      paddingInline: 16,
    },
    container: {
      display: 'flex',
      flexDirection: 'column',
      paddingBlock: 16,
    },
    hairline: {
      backgroundColor: 'var(--divider)',
      height: 1,
      marginBottom: 4,
      marginTop: 16,
    },
    hairlineContainer: {
      paddingInline: 16,
    },
    header: {
      paddingInline: 16,
    },
    megaphone: {
      paddingBottom: 16,
      paddingInline: 16,
    },
    section: {
      marginTop: 16,
    },
  }),
  stylex.create({
    descriptionSpacing: {
      paddingTop: 20,
    },
    headerContainer: {
      paddingBottom: 20,
      paddingInline: 16,
    },
  }),
  stylex.create({
    contentsComposerSection: {
      marginTop: 16,
      paddingInline: 16,
    },
    optionsButtons: {
      marginBlock: 16,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
    },
    headlines: {
      marginTop: 16,
      paddingInline: 16,
    },
    reasonButtons: {
      marginBottom: 32,
    },
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      padding: 24,
    },
  }),
  stylex.create({
    root: {
      margin: '0 auto',
      marginTop: 24,
      width: 600,
    },
  }),
  stylex.create({
    feedbackSubtitle: {
      marginBottom: 32,
      marginTop: 12,
    },
    rightAlignSubmitButton: {
      float: 'end',
    },
    root: {
      margin: 16,
    },
    shareMoreDetailsContainer: {
      marginBottom: 4,
    },
    starContainer: {
      marginBottom: 32,
    },
    starHeader: {
      marginBottom: 24,
    },
    starTextHint: {
      marginTop: 16,
    },
    submitSurvey: {
      float: 'start',
      marginBottom: 16,
    },
  }),
  stylex.create({
    itemContainer: {
      alignItems: 'center',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      display: 'flex',
      marginInline: 16,
      paddingBlock: 16,
    },
    itemDescription: {
      display: 'flex',
      flexDirection: 'column',
      marginInlineStart: 12,
    },
    itemImage: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      height: 60,
      width: 60,
    },
    itemName: {
      marginBottom: 8,
    },
  }),
  stylex.create({
    confirmationHeader: {
      marginBottom: 8,
    },
    confirmationRoot: {
      paddingInline: 16,
      paddingTop: 20,
    },
    exitButton: {
      float: 'start',
      marginTop: 36,
      paddingBottom: 16,
    },
  }),
  stylex.create({
    text: {
      width: '50%',
    },
  }),
  stylex.create({
    pillsContainer: {
      paddingTop: 8,
    },
  }),
  stylex.create({
    slider: {
      paddingTop: 8,
    },
  }),
  stylex.create({
    starContainer: {
      paddingTop: 16,
    },
    starTextHint: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    card: {
      borderWidth: 1,
    },
    container: {
      padding: 16,
    },
    divider: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--wash)',
      paddingTop: 16,
    },
    title: {
      paddingTop: 8,
    },
  }),
  stylex.create({
    button: {
      flexGrow: 1,
      flexShrink: 1,
      margin: 4,
    },
    buttonColumn: {
      flexDirection: 'column',
    },
    buttonGroup: {
      display: 'flex',
      marginBlock: 16,
      paddingTop: 8,
    },
    buttonRow: {
      flexDirection: 'row',
    },
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--wash)',
      margin: '0 auto',
      padding: 48,
      width: 876,
    },
  }),
  stylex.create({
    checkbox: {
      margin: 4,
    },
    checkboxGroup: {
      flexDirection: 'column',
      marginBlock: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    composer: {
      paddingTop: 16,
    },
    submitSurvey: {
      padding: 16,
    },
  }),
  stylex.create({
    root: {
      margin: '0 auto',
      padding: 48,
      width: 876,
    },
  }),
  stylex.create({
    radioButton: {
      margin: 4,
    },
    radioGroup: {
      flexDirection: 'column',
      marginBlock: 8,
      paddingTop: 8,
    },
  }),
  stylex.create({
    wrapper: {
      padding: 4,
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 8,
      height: 15,
      marginBottom: 15,
    },
    bodyGlimmerContainer: {
      padding: 16,
    },
    imageSize40: {
      height: 40,
      width: 40,
    },
    imageStyleCircle: {
      borderRadius: '50%',
    },
    textGlimmerWidth100: {
      width: '100%',
    },
    textGlimmerWidth67: {
      width: '67%',
    },
  }),
  stylex.create({
    fixedHeight: {
      height: 300,
    },
    moreTabXStyle: {
      height: 60,
      padding: '20px 16px',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    tab: {
      minHeight: 20,
      padding: '20px 16px',
    },
    tabRow: {
      boxSizing: 'border-box',
      paddingInlineStart: 16,
      width: '100%',
    },
  }),
  stylex.create({
    fixedHeight: {
      height: 300,
    },
    moreTabXStyle: {
      height: 60,
      padding: '20px 16px',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    tab: {
      minHeight: 20,
      padding: '20px 16px',
    },
    tabRow: {
      boxSizing: 'border-box',
      paddingInlineStart: 16,
      width: '100%',
    },
  }),
  stylex.create({
    fixedContainer: {
      width: '548px',
    },
  }),
  stylex.create({
    row: {
      padding: 4,
    },
  }),
  stylex.create({
    container: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    container: {
      paddingTop: 16,
    },
    maxWidth: {
      width: '100%',
    },
  }),
  stylex.create({
    button: {
      display: 'flex',
      paddingBlock: 5,
    },
  }),
  stylex.create({
    buttons: {
      marginBottom: 24,
      marginInlineEnd: 8,
      marginInlineStart: 8,
    },
    notifications: {
      marginInlineEnd: 8,
      marginInlineStart: 8,
      marginTop: 8,
    },
    text: {
      marginBottom: 25,
      marginInlineEnd: 8,
      marginInlineStart: 16,
      marginTop: 25,
    },
  }),
  stylex.create({
    buttons: {
      minWidth: '35%',
    },
    words: {
      maxWidth: '55%',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      width: 500,
      '@media (max-width: 500px)': {
        width: '100%',
      },
    },
  }),
  stylex.create({
    buttons: {
      minWidth: '55%',
    },
    cardContainer: {
      padding: 8,
      paddingBottom: 8,
    },
    container: {
      marginBottom: -8,
      paddingTop: 24,
    },
    text: {
      minWidth: '35%',
    },
  }),
  stylex.create({
    button: {
      paddingTop: 16,
    },
    commentListBorder: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      margin: '0 16px',
    },
    infoSection: {
      margin: 16,
    },
  }),
  stylex.create({
    progressBarContainer: {
      marginBottom: 8,
      marginInlineEnd: 16,
      marginInlineStart: 16,
      width: '93%',
    },
    root: {
      backgroundColor: 'var(--attachment-footer-background)',
    },
    textContentRoot: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--divider)',
      borderInlineEndStyle: 'none',
      borderInlineStartStyle: 'none',
      borderTopColor: 'var(--divider)',
      paddingBottom: '10px',
      paddingInlineEnd: '270px',
      width: '100%',
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 16,
    },
    header: {
      alignItems: 'stretch',
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      paddingTop: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      paddingBottom: 16,
    },
    root: {
      marginBottom: 16,
    },
    subtitle: {
      marginInlineEnd: 16,
      marginInlineStart: 16,
    },
  }),
  stylex.create({
    content: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    container: {
      width: 500,
    },
    dialogComponent: {
      paddingBottom: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    container: {
      width: 360,
    },
    hoursList: {
      padding: 16,
    },
  }),
  stylex.create({
    chevron: {
      display: 'inline',
      marginInlineStart: 10,
    },
  }),
  stylex.create({
    drop_down: {
      width: 50,
    },
  }),
  stylex.create({
    icon: {
      display: 'inline-block',
      position: 'relative',
      top: -8,
    },
  }),
  stylex.create({
    buttons: {
      float: 'end',
      paddingBottom: 12,
      paddingTop: 16,
    },
    drop_down: {
      width: 50,
    },
  }),
  stylex.create({
    header: {
      marginTop: -36,
    },
  }),
  stylex.create({
    backLink: {
      marginInlineEnd: 16,
      marginInlineStart: 8,
    },
  }),
  stylex.create({
    button: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 8,
    },
  }),
  stylex.create({
    cardContainer: {
      paddingTop: 0,
    },
    description: {
      margin: 12,
      marginInlineStart: 0,
      paddingInline: 12,
    },
  }),
  stylex.create({
    buttonContainer: {
      paddingInline: 16,
    },
    cardContainer: {
      padding: 16,
      paddingTop: 20,
    },
  }),
  stylex.create({
    cardContainer: {
      padding: 16,
      paddingTop: 0,
    },
  }),
  stylex.create({
    button: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 8,
    },
    root: {
      marginTop: 8,
    },
  }),
  stylex.create({
    map: {
      borderRadius: 0,
      marginBottom: 12,
      marginInlineEnd: 12,
      marginInlineStart: 12,
    },
  }),
  stylex.create({
    cardContainer: {
      padding: 16,
      paddingTop: 0,
    },
    donateButton: {
      paddingTop: 24,
    },
  }),
  stylex.create({
    profileBorder: {
      backgroundColor: 'var(--card-background)',
      borderRadius: '50%',
      padding: 4,
    },
  }),
  stylex.create({
    backgroundBlur: {
      filter: 'blur(50px)',
    },
  }),
  stylex.create({
    divider: {
      marginBottom: 16,
      marginInlineEnd: 16,
      marginInlineStart: 16,
      marginTop: 16,
      width: '90%',
    },
    facepile: {
      display: 'flex',
      justifyContent: 'flex-start',
      marginInlineEnd: 20,
      marginInlineStart: 20,
      overflow: 'auto',
      paddingBottom: 4,
    },
  }),
  stylex.create({
    body: {
      alignItems: 'center',
      display: 'flex',
      flexWrap: 'wrap',
      paddingBottom: 10,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 12,
    },
    root: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    feedUnit: {
      marginBottom: -32,
    },
  }),
  stylex.create({
    root: {
      marginTop: 8,
    },
  }),
  stylex.create({
    backLink: {
      marginInlineEnd: 16,
      marginInlineStart: 8,
    },
    privacy: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    footer: {
      paddingBottom: '16px',
    },
    stepper: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      padding: '16px 16px 0px',
    },
  }),
  stylex.create({
    photoPicker: {
      backgroundColor: 'var(--web-wash)',
      height: 171,
      width: 326,
    },
  }),
  stylex.create({
    root: {
      boxSizing: 'border-box',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    buttonsSection: {
      padding: '16px',
    },
    content: {
      display: 'flex',
      flexDirection: 'column',
      width: 500,
    },
    headerRow: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      padding: '16px',
    },
    headerText: {
      width: '100%',
    },
    middleSection: {
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      padding: '8px 0 8px 8px',
      width: '95%',
    },
    subHeaderText: {
      padding: '24px 16px 0 16px',
    },
  }),
  stylex.create({
    footer: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--media-inner-border)',
      paddingBottom: 16,
    },
  }),
  stylex.create({
    contextualMessageWrapper: {
      backgroundColor: 'var(--web-wash)',
      borderRadius: 8,
      padding: 16,
    },
    root: {
      minWidth: 500,
    },
    scrollableArea: {
      maxHeight: '40vh',
    },
  }),
  stylex.create({
    checkboxLabelContainer: {
      paddingInlineEnd: 16,
    },
    root: {
      display: 'flex',
      marginTop: 16,
    },
  }),
  stylex.create({
    ruleDescription: {
      marginInlineStart: 24,
      marginTop: 8,
    },
    ruleHead: {
      display: 'flex',
    },
    ruleIndex: {
      width: 24,
    },
    ruleRow: {
      marginBottom: 20,
      marginTop: 20,
    },
    rulesAgreement: {
      marginTop: 16,
    },
    rulesTitle: {
      marginBottom: 4,
      marginTop: 16,
    },
  }),
  stylex.create({
    attachmentPaddingWithoutRemoveButton: {
      padding: '11px 0',
    },
    pressable: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'space-between',
      padding: 8,
    },
    removeButton: {
      alignSelf: 'flex-start',
      transform: 'scale(0.8)',
    },
    root: {
      margin: '10px 16px',
      width: '100%',
    },
  }),
  stylex.create({
    body: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'space-between',
      padding: 12,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      height: 432,
      width: '100%',
    },
    textPairingContainer: {
      margin: '12px 4px 20px 4px',
    },
  }),
  stylex.create({
    body: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'space-between',
      padding: 12,
    },
    bodyText: {
      borderRadius: 6,
      height: 20,
      width: 427,
    },
    button: {
      borderRadius: 6,
      height: 40,
    },
    headerText: {
      borderRadius: 6,
      height: 14,
      marginBottom: 12,
      marginTop: 12,
      width: 88,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      height: 432,
      width: 508,
    },
    textView: {
      marginInlineStart: 4,
    },
  }),
  stylex.create({
    body: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'space-between',
      padding: 16,
    },
    buttonWrapper: {
      width: 'auto',
    },
    root: {
      margin: 16,
    },
  }),
  stylex.create({
    toast: {
      bottom: '8px',
      paddingInline: '16px',
      position: 'fixed',
    },
  }),
  stylex.create({
    root: {
      marginInlineEnd: -16,
      marginInlineStart: -16,
    },
  }),
  stylex.create({
    button: {
      marginInlineStart: 16,
    },
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
    iconWithTitle: {
      marginTop: -4,
    },
    root: {
      borderColor: 'var(--secondary-button-stroke)',
      borderRadius: 'var(--alert-banner-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 1,
      overflow: 'hidden',
      padding: 20,
    },
    rowItem: {
      width: '100%',
    },
  }),
  stylex.create({
    root: {
      marginInline: 20,
    },
  }),
  stylex.create({
    imagePreview: {
      maxHeight: 80,
      maxWidth: 80,
    },
    item: {
      alignItems: 'center',
      backgroundColor: 'var(--wash)',
      borderRadius: 6,
      display: 'flex',
      height: 80,
      justifyContent: 'center',
      marginInlineEnd: 8,
      overflow: 'hidden',
      position: 'relative',
      width: 80,
    },
    removeButton: {
      end: 0,
      position: 'absolute',
      top: 0,
    },
  }),
  stylex.create({
    root: {
      marginBottom: 12,
    },
    uploadedFiles: {
      display: 'flex',
    },
  }),
  stylex.create({
    root: {
      paddingBottom: 27,
      paddingTop: 11,
    },
  }),
  stylex.create({
    section: {
      marginInlineEnd: 20,
      marginInlineStart: 20,
    },
  }),
  stylex.create({
    section: {
      marginInline: 20,
    },
  }),
  stylex.create({
    li: {
      marginInlineStart: '1.65em',
      position: 'relative',
    },
    marker: {
      position: 'absolute',
      start: -15,
    },
    spacing: {
      marginTop: 16,
    },
    ul: {
      display: 'flex',
      flexDirection: 'column',
      listStyleType: 'none',
      margin: 0,
      padding: 0,
    },
  }),
  stylex.create({
    button: {
      borderRadius: 'var(--button-corner-radius)',
    },
    container: {
      display: 'inline-flex',
      position: 'relative',
    },
    disabled: {
      opacity: 0.4,
    },
    onMedia: {
      backgroundColor: 'var(--overlay-on-media)',
    },
    pressed: {
      opacity: 0.7,
    },
    primary: {
      backgroundColor: 'var(--primary-button-background)',
    },
    secondary: {
      backgroundColor: 'var(--secondary-button-background)',
      borderColor: 'var(--secondary-button-stroke)',
      borderStyle: 'solid',
      borderWidth: 1,
    },
    sizeLarge: {
      height: 'var(--button-height-large)',
      paddingInline: 'var(--button-padding-horizontal-large)',
    },
    sizeLargeWithAddOnEnd: {
      paddingInlineEnd: 'var(--button-icon-padding-large)',
    },
    sizeLargeWithAddOnStart: {
      paddingInlineStart: 'var(--button-icon-padding-large)',
    },
    sizeMedium: {
      height: 'var(--button-height-medium)',
      paddingInline: 'var(--button-padding-horizontal-medium)',
    },
    sizeMediumWithAddOnEnd: {
      paddingInlineEnd: 'var(--button-icon-padding-medium)',
    },
    sizeMediumWithAddOnStart: {
      paddingInlineStart: 'var(--button-icon-padding-medium)',
    },
    text: {
      backgroundColor: 'transparent',
    },
    textWhileLoading: {
      opacity: 0,
    },
    widthModeConstrained: {
      width: 'auto',
    },
    widthModeFlexible: {
      width: '100%',
    },
  }),
  stylex.create({
    spinnerContainer: {
      boxSizing: 'border-box',
      left: '50%',
      position: 'absolute',
      top: '50%',
      transform: 'translate(-50%, -50%)',
      zIndex: 1,
    },
  }),
  stylex.create({
    spinnerContainer: {
      boxSizing: 'border-box',
      left: '50%',
      position: 'absolute',
      top: '50%',
      transform: 'translate(-50%, -50%)',
      zIndex: 1,
    },
  }),
  stylex.create({
    containerNegativeMargin: {
      margin: -6,
    },
    hiddenButton: {
      height: 0,
      visibility: 'hidden',
    },
    itemSpacing: {
      padding: 6,
    },
    resetFlexBasis: {
      flexBasis: 'auto',
    },
    stacked: {
      minWidth: '100%',
    },
  }),
  stylex.create({
    pressed: {
      opacity: 0.75,
      transform: 'scale(0.98)',
    },
    root: {
      alignItems: 'stretch',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow: 'var(--shadow-persistent)',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      isolation: 'isolate',
      justifyContent: 'flex-start',
      position: 'relative',
      '::after': {
        backgroundColor: 'var(--card-background)',
        borderRadius: 'var(--card-corner-radius)',
        content: '""',
        height: '100%',
        position: 'absolute',
        start: 0,
        top: 0,
        width: '100%',
        zIndex: -1,
      },
    },
  }),
  stylex.create({
    bottomAddOnContainer: {
      marginBottom: 'var(--card-padding-vertical)',
      marginInline: 'var(--card-padding-horizontal)',
    },
    bottomAddOnContainerTopMargin: {
      marginTop: 'var(--card-padding-vertical)',
    },
    imageContainer: {
      borderTopEndRadius: 'var(--card-corner-radius)',
      borderTopStartRadius: 'var(--card-corner-radius)',
      overflow: 'hidden',
    },
    imagePosition: {
      justifyContent: 'center',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
    },
    textContainer: {
      flexGrow: 1,
      marginInline: 'var(--card-padding-horizontal)',
      marginBlock: 'var(--card-padding-vertical)',
    },
    textContainerElement: {
      marginBottom: 'var(--card-padding-vertical)',
      ':last-child': {
        marginBottom: 0,
      },
    },
  }),
  stylex.create({
    checkedWrapper: {
      margin: -2,
    },
    circle: {
      borderRadius: '50%',
      display: 'inline-block',
      flexShrink: 0,
      height: 20,
      position: 'relative',
      width: 20,
    },
    disabledBorder: {
      borderWidth: 2,
      borderStyle: 'solid',
      borderColor: 'var(--disabled-icon)',
    },
    root: {
      WebkitTapHighlightColor: 'transparent',
      alignItems: 'center',
      display: 'flex',
      touchAction: 'manipulation',
    },
    text: {
      flexShrink: 1,
      marginInlineStart: 8,
    },
    uncheckedBorder: {
      borderWidth: 2,
      borderStyle: 'solid',
      borderColor: 'var(--placeholder-icon)',
    },
  }),
  stylex.create({
    anchor: {
      paddingInline: 12,
    },
    body: {
      margin: '0 20px',
      paddingBottom: 4,
      paddingTop: 5,
    },
    card: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow:
        '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1), inset 0 0 0 1px var(--shadow-inset)',
      display: 'flex',
      flexDirection: 'column',
      width: 508,
    },
    closeButton: {
      end: 16,
      position: 'absolute',
      top: 20,
    },
    heading: {
      margin: '16px 20px',
      paddingBottom: 6,
      paddingInlineEnd: 24,
      paddingTop: 7,
    },
    item: {
      flexBasis: 0,
      minWidth: 'fit-content',
      paddingBottom: 20,
      paddingInline: 10,
    },
    root: {
      paddingInline: 10,
      paddingTop: 20,
    },
  }),
  stylex.create({
    anchor: {
      alignItems: 'stretch',
      maxHeight: '100vh',
      minHeight: 488,
      paddingInline: 4,
      paddingBlock: 'var(--dialog-anchor-vertical-padding)',
      '@media (max-width: 679px)': {
        maxHeight: 'none',
        minHeight: '100vh',
      },
      '@supports (padding: env(safe-area-inset-bottom, 0))': {
        paddingBottom:
          'calc(var(--dialog-anchor-vertical-padding) + env(safe-area-inset-bottom, 0))',
        paddingTop: 'calc(var(--dialog-anchor-vertical-padding) + env(safe-area-inset-top, 0))',
      },
    },
    anchorDvhWhenNarrow: {
      '@media (max-width: 679px)': {
        minHeight: ['100vh', '100dvh'],
      },
    },
    card: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 'var(--dialog-corner-radius)',
      clipPath: 'inset(0px 0px 0px 0px round var(--dialog-corner-radius))',
      flexGrow: 1,
      position: 'relative',
      '@media (max-width: 679px)': {
        boxShadow: 'none',
        overflow: 'visible',
      },
    },
    dialog: {
      alignItems: 'stretch',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow:
        '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1), inset 0 0 0 1px var(--shadow-inset)',
      display: 'flex',
      overflow: 'visible',
      width: '100%',
    },
    root: {
      '@media (max-width: 679px)': {
        justifyContent: 'flex-start',
      },
    },
  }),
  stylex.create({
    medium: {
      maxWidth: 700,
    },
    small: {
      maxWidth: 548,
    },
  }),
  stylex.create({
    meta: {
      marginInline: 10,
    },
  }),
  stylex.create({
    actionPlaceholder: {
      height: 40,
      width: 40,
    },
    addOnsEnd: {
      display: 'flex',
      marginInline: 8,
    },
    addOnsStart: {
      display: 'flex',
      marginInline: 8,
    },
    companyLogo: {
      alignItems: 'center',
      display: 'flex',
      marginInline: 12,
    },
    title: {
      display: 'flex',
      flexGrow: 1,
      justifyContent: 'center',
    },
  }),
  stylex.create({
    padding: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      paddingBottom: 20,
      paddingTop: 10,
    },
    inert: {
      pointerEvents: 'none',
      userSelect: 'none',
    },
    placeholder: {
      opacity: 0,
      pointerEvents: 'none',
      position: 'relative',
    },
    root: {
      borderRadius: 'inherit',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      maxHeight: 'calc(100vh - (2 * var(--dialog-anchor-vertical-padding)))',
      position: 'relative',
      '@media (max-width: 679px)': {
        maxHeight: 'none',
      },
    },
    scrollableArea: {
      flexGrow: 1,
      overscrollBehaviorY: 'auto',
    },
    scrollSectionObserver: {
      height: 5,
    },
  }),
  stylex.create({
    addOnEnd: {
      marginInline: 8,
    },
    addOnStart: {
      marginInline: 8,
    },
    bottomItem: {
      bottom: 0,
      position: 'absolute',
      width: '100%',
    },
  }),
  stylex.create({
    column: {
      padding: 20,
    },
    content: {
      overflow: 'hidden',
    },
    root: {
      backfaceVisibility: 'hidden',
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
    },
    sticky: {
      '@media (max-width: 679px)': {
        position: 'sticky',
      },
    },
  }),
  stylex.create({
    fixPositioned: {
      position: 'fixed',
    },
    gradient: {
      backgroundColor: 'var(--surface-background)',
      bottom: 0,
      maxWidth: 548,
      start: '50%',
      top: 0,
      transform: 'translateX(-50%)',
      width: '100%',
    },
    gradientContainer: {
      bottom: 0,
      clipPath: 'inset(0 0 0 0)',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    hidden: {
      opacity: 0,
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    rtl: {
      end: '50%',
      start: 'initial',
    },
    shadow: {
      boxShadow: 'var(--scroll-shadow)',
      clipPath: 'unset',
      pointerEvents: 'none',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-in)',
    },
  }),
  stylex.create({
    header: {
      alignItems: 'center',
      display: 'flex',
      end: 0,
      flexShrink: 0,
      height: 64,
      justifyContent: 'space-between',
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 1,
      '@media (max-width: 679px)': {
        position: 'sticky',
      },
    },
  }),
  stylex.create({
    firstLine: {
      height: 10,
      marginBottom: 10,
      marginInline: 20,
      maxWidth: 440,
    },
    glimmer: {
      alignSelf: 'flex-start',
      boxSizing: 'border-box',
      width: 'calc(100% - 40px)',
    },
    heading: {
      height: 20,
      marginBottom: 20,
      marginInline: 20,
      maxWidth: 241,
    },
    secondLine: {
      height: 10,
      marginBottom: 10,
      marginInline: 20,
      maxWidth: 296,
    },
  }),
  stylex.create({
    verticalSpaceLarge: {
      marginBottom: 32,
    },
    verticalSpaceMed: {
      marginBottom: 24,
    },
    verticalSpaceSm: {
      marginBottom: 12,
    },
  }),
  stylex.create({
    28: {
      width: 28,
    },
    40: {
      width: 40,
    },
    60: {
      width: 60,
    },
    100: {
      width: 100,
    },
  }),
  stylex.create({
    28: {
      height: 28,
    },
    40: {
      height: 40,
    },
    60: {
      height: 60,
    },
    100: {
      height: 100,
    },
  }),
  stylex.create({
    28: {
      marginInlineEnd: -7,
    },
    40: {
      marginInlineEnd: -10,
    },
    60: {
      marginInlineEnd: -15,
    },
    100: {
      marginInlineEnd: -25,
    },
  }),
  stylex.create({
    28: {
      paddingInlineEnd: 7,
    },
    40: {
      paddingInlineEnd: 10,
    },
    60: {
      paddingInlineEnd: 15,
    },
    100: {
      paddingInlineEnd: 25,
    },
  }),
  stylex.create({
    moreTab: {
      display: 'none',
      position: 'relative',
      zIndex: 1,
    },
    root: {
      width: 'auto',
    },
  }),
  stylex.create({
    input: {
      backgroundColor: 'transparent',
      borderStyle: 'none',
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      display: 'inline',
      fontSize: '1rem !important',
      fontWeight: 'normal',
      height: 22,
      lineHeight: 1.2941176470588236,
      marginInline: 2,
      marginBlock: 8,
      maxWidth: '100%',
      minWidth: 0,
      outline: 'none',
      textOverflow: 'ellipsis',
      '::-webkit-search-cancel-button': {
        display: 'none',
      },
      '::-webkit-search-results-button': {
        display: 'none',
      },
    },
    inputFocused: {
      marginBlock: 0,
    },
    inputFont: {
      fontFamily: 'Optimistic Display Medium, system-ui, sans-serif !important',
      '::-ms-reveal': {
        display: 'none',
      },
      ':disabled': {
        color: 'var(--secondary-text)',
      },
    },
    root: {
      marginInline: '-4px',
      maxWidth: '100%',
    },
    rootFilled: {
      paddingTop: 16,
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 0.3,
    },
    '100%': {
      opacity: 0.5,
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 0.05,
    },
    '100%': {
      opacity: 0.15,
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 0.03,
    },
    '100%': {
      opacity: 0.1,
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 0.05,
    },
    '100%': {
      opacity: 0.1,
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    dark: {
      animationName: 'x2i8c5m-B',
      backgroundColor: 'var(--always-white)',
      opacity: 0.05,
    },
    darkOnWhiteBackground: {
      animationName: 'x1hgehdp-B',
      backgroundColor: 'var(--accent)',
      opacity: 0.03,
    },
    light: {
      animationName: 'xq9zstv-B',
      backgroundColor: 'var(--always-white)',
      opacity: 0.3,
    },
    lightOnWhiteBackground: {
      animationName: 'xe6bajg-B',
      backgroundColor: 'var(--accent)',
      opacity: 0.05,
    },
    paused: {
      animationPlayState: 'paused',
    },
    root: {
      animationDirection: 'alternate',
      animationDuration: '1s',
      animationIterationCount: 'infinite',
      animationTimingFunction: 'cubic-bezier(0.5, 0.0, 0.5, 1.0)',
      borderRadius: 'var(--glimmer-corner-radius)',
    },
  }),
  stylex.create({
    canvas: {
      backgroundImage:
        'radial-gradient(rgba(255,255,255,.25), rgba(255,255,255,0) 40%), radial-gradient(hsla(44, 100%, 66%, 1) 30%, hsla(338, 68%, 65%, 1), hsla(338, 68%, 65%, 0.4) 41%, transparent 52%), radial-gradient(hsla(272, 100%, 60%, 1) 37%, transparent 46%), linear-gradient(155deg, transparent 65%, hsla(142, 70%, 49%, 1) 95%), linear-gradient(45deg, hsla(213, 100%, 44%, 1), hsla(209, 100%, 53%, 1))',
      backgroundPosition: 'bottom left, 109% 68%, 109% 68%, center, center',
      backgroundRepeat: 'no-repeat',
      backgroundSize: '200% 200%, 285% 500%, 285% 500%, cover, cover',
      bottom: 0,
      end: 0,
      opacity: 0.08,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    disablePointerEvents: {
      pointerEvents: 'none',
    },
    root: {
      backgroundColor: 'var(--surface-background)',
      borderRadius: 'inherit',
      clipPath: 'inset(0 0 0 0)',
      contain: 'strict',
      height: '100%',
      position: 'relative',
      width: '100%',
      zIndex: 0,
    },
  }),
  stylex.keyframes({
    '0%': {
      transform: 'translate(-50%,-50%) rotate(0deg)',
    },
    '100%': {
      transform: 'translate(-50%,-50%) rotate(360deg)',
    },
  }),
  stylex.keyframes({
    '50%, 100%': {
      transform: 'none',
    },
    '75%': {
      transform: 'translateY(36%)',
    },
  }),
  stylex.keyframes({
    '0%, 50%': {
      transform: 'none',
    },
    '25%': {
      transform: 'translate(16%, -53%)',
    },
  }),
  stylex.create({
    '0%, 100%': {
      opacity: 1,
      transform: 'none',
    },
    '25%, 75%': {
      opacity: 0.5,
      transform: 'none',
    },
    '50%': {
      opacity: 0.5,
      transform: 'translate(-42%, 15%)',
    },
  }),
  stylex.create({
    animate: {
      animationDuration: '12s',
      animationIterationCount: 'infinite',
      animationTimingFunction: 'steps(120, end)',
    },
    blue: {
      backgroundImage:
        'radial-gradient(50% 50% at 50% 50%, rgba(24, 119, 242, 0.3) 0%, rgba(24, 119, 242, 0) 50%), radial-gradient(50% 50% at 50% 50%, rgba(24, 119, 242, 0.5) 0%, rgba(24, 119, 242, 0) 75%), radial-gradient(50% 50% at 50% 50%, rgba(24, 119, 242, 0.8) 0%, rgba(24, 119, 242, 0) 100%)',
      bottom: '10%',
      left: 0,
      position: 'absolute',
      right: 0,
      top: '26%',
    },
    blue2: {
      animationName: 'x1m0k86b-B',
      bottom: 0,
      left: 0,
      right: '24.38%',
      top: '51.52%',
    },
    canvas: {
      animationName: 'xej97of-B',
      backgroundColor: 'var(--surface-background)',
      left: '50%',
      opacity: 0.08,
      paddingBottom: '300%',
      pointerEvents: 'none',
      position: 'absolute',
      top: '50%',
      transform: 'translate(-50%,-50%)',
      width: '300%',
    },
    canvasInDarkMode: {
      opacity: 0.1,
    },
    children: {
      height: '100%',
      position: 'relative',
      width: '100%',
    },
    coral: {
      backgroundImage:
        'radial-gradient(50% 50% at 50% 50%, rgba(255, 108, 92, 0.3) 0%, rgba(255, 108, 92, 0) 50%), radial-gradient(50% 50% at 50% 50%, rgba(255, 108, 92, 0.5) 0%, rgba(255, 108, 92, 0) 75%), radial-gradient(50% 50% at 50% 50%, rgba(255, 108, 92, 0.8) 0%, rgba(255, 108, 92, 0) 100%)',
      bottom: '45.27%',
      left: '37.78%',
      position: 'absolute',
      right: '4.9%',
      top: '17.96%',
    },
    disablePointerEvents: {
      pointerEvents: 'none',
    },
    green: {
      animationName: 'xkks8vb-B',
      backgroundImage:
        'radial-gradient(50% 50% at 50% 50%, rgba(37, 211, 102, 0.3) 0%, rgba(37, 211, 102, 0) 50%), radial-gradient(50% 50% at 50% 50%, rgba(37, 211, 102, 0.5) 0%, rgba(37, 211, 102, 0) 75%), radial-gradient(50% 50% at 50% 50%, rgba(37, 211, 102, 0.8) 0%, rgba(37, 211, 102, 0) 100%)',
      bottom: '44%',
      left: '17%',
      position: 'absolute',
      right: '17%',
      top: '13.7%',
    },
    purple: {
      animationName: 'xxlqcyx-B',
      backgroundImage:
        'radial-gradient(50% 50% at 50% 50%, rgba(160, 51, 255, 0.3) 0%, rgba(160, 51, 255, 0) 50%), radial-gradient(50% 50% at 50% 50%, rgba(160, 51, 255, 0.5) 0%, rgba(160, 51, 255, 0) 75%), radial-gradient(50% 50% at 50% 50%, rgba(160, 51, 255, 0.8) 0%, rgba(160, 51, 255, 0) 100%)',
      bottom: '38.4%',
      left: '45.99%',
      position: 'absolute',
      right: '1.86%',
      top: '28.59%',
    },
    root: {
      clipPath: 'inset(0 0 0 0)',
      contain: 'strict',
      height: '100%',
      position: 'relative',
      width: '100%',
    },
    yellow: {
      backgroundImage:
        'radial-gradient(50% 50% at 50% 50%, rgba(245, 206, 51, 0.3) 0%, rgba(245, 206, 51, 0) 50%), radial-gradient(50% 50% at 50% 50%, rgba(245, 206, 51, 0.5) 0%, rgba(245, 206, 51, 0) 75%), radial-gradient(50% 50% at 50% 50%, rgba(245, 206, 51, 0.8) 0%, rgba(245, 206, 51, 0) 100%)',
      bottom: '66.41%',
      left: '36.88%',
      position: 'absolute',
      right: '19.69%',
      top: '5.73%',
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--card-background)',
      borderColor: 'var(--secondary-button-stroke)',
      borderRadius: 'var(--input-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 1,
      display: 'flex',
      flexDirection: 'column',
      overflow: 'hidden',
    },
    rowContainer: {
      display: 'flex',
    },
  }),
  stylex.create({
    button: {
      alignItems: 'center',
      backgroundColor: 'var(--card-background)',
      borderRadius: 18,
      boxShadow: '0 2px 8px var(--shadow-1), 0 0 0 1px var(--shadow-1)',
      display: 'flex',
      height: 36,
      justifyContent: 'center',
      width: 36,
    },
    pressed: {
      transform: 'scale(0.96)',
    },
  }),
  stylex.create({
    child: {
      marginInline: 6,
    },
    container: {
      display: 'flex',
      flexDirection: 'row',
      marginInline: -6,
    },
    expanding: {
      flexGrow: 1,
    },
  }),
  stylex.create({
    padding: {
      padding: 20,
    },
  }),
  stylex.create({
    buttons: {
      maxWidth: '100%',
      minWidth: 'fit-content',
      width: 'calc((600px - 100%) * 9999)',
    },
    child: {
      margin: 6,
    },
    container: {
      alignItems: 'flex-end',
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap',
      margin: -6,
    },
    expanding: {
      flexBasis: 0,
      flexGrow: 1,
    },
  }),
  stylex.create({
    aspectRatioContainer: {
      overflow: 'hidden',
    },
    photo: {
      width: '100%',
    },
  }),
  stylex.create({
    child: {
      marginBlock: 16,
    },
    container: {
      marginBlock: -16,
    },
  }),
  stylex.create({
    icon: {
      display: 'block',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'color, fill, stroke',
      transitionTimingFunction: 'var(--fds-soft)',
    },
    inline: {
      display: 'inline-block',
    },
    shadow: {
      filter: 'drop-shadow(0 2px 8px var(--shadow-1))',
    },
  }),
  stylex.create({
    8: {
      height: 8,
      width: 8,
    },
    10: {
      height: 10,
      width: 10,
    },
    12: {
      height: 12,
      width: 12,
    },
    16: {
      height: 16,
      width: 16,
    },
    18: {
      height: 18,
      width: 18,
    },
    20: {
      height: 20,
      width: 20,
    },
    24: {
      height: 24,
      width: 24,
    },
    28: {
      height: 28,
      width: 28,
    },
    30: {
      height: 30,
      width: 30,
    },
    32: {
      height: 32,
      width: 32,
    },
    36: {
      height: 36,
      width: 36,
    },
    40: {
      height: 40,
      width: 40,
    },
    48: {
      height: 48,
      width: 48,
    },
    52: {
      height: 52,
      width: 52,
    },
    56: {
      height: 56,
      width: 56,
    },
    60: {
      height: 60,
      width: 60,
    },
    72: {
      height: 72,
      width: 72,
    },
    96: {
      height: 96,
      width: 96,
    },
    112: {
      height: 112,
      width: 112,
    },
    132: {
      height: 132,
      width: 132,
    },
  }),
  stylex.create({
    'active-tab': {
      color: 'var(--primary-button-background)',
    },
    black: {
      color: 'var(--always-black)',
    },
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabled: {
      color: 'var(--disabled-icon)',
    },
    highlight: {
      color: 'var(--blue-link)',
    },
    'inactive-tab': {
      color: 'var(--secondary-icon)',
    },
    'list-cell-chevron': {
      color: 'var(--list-cell-chevron)',
    },
    negative: {
      color: 'var(--negative)',
    },
    none: {
      color: 'transparent',
    },
    positive: {
      color: 'var(--positive)',
    },
    primary: {
      color: 'var(--primary-icon)',
    },
    'primary-button': {
      color: 'var(--primary-button-text)',
    },
    secondary: {
      color: 'var(--secondary-icon)',
    },
    'secondary-button': {
      color: 'var(--secondary-button-text)',
    },
    tertiary: {
      color: 'var(--placeholder-icon)',
    },
    'toast-DO_NOT_USE_WILL_BE_DELETED_WITHOUT_NOTICE': {
      color: 'var(--toast-text)',
    },
    warning: {
      color: 'var(--warning)',
    },
    white: {
      color: 'var(--always-white)',
    },
  }),
  stylex.create({
    contained: {
      borderRadius: '50%',
      display: 'inline-flex',
      isolation: 'isolate',
      position: 'relative',
    },
    disabled: {
      opacity: 0.4,
    },
    pressable: {
      borderRadius: '50%',
      display: 'inline-flex',
      padding: 8,
    },
    pressableContained: {
      backgroundColor: 'var(--card-background)',
    },
    pressed: {
      transform: 'scale(0.96)',
    },
    shadow: {
      borderRadius: '50%',
    },
  }),
  stylex.create({
    aspectRatioContainer: {
      overflow: 'hidden',
    },
    imageContainer: {
      margin: 0,
      padding: 0,
      position: 'relative',
    },
    imageContainerFlex: {
      alignItems: 'stretch',
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      paddingTop: 0,
      paddingInlineEnd: 0,
      paddingBottom: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
    },
    imageContainerInline: {
      display: 'inline-block',
      fontSize: 0,
    },
    imageOverlay: {
      height: '100%',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap',
      width: '100%',
    },
  }),
  stylex.create({
    border: {
      borderColor: 'var(--media-inner-border)',
      borderRadius: 'var(--image-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 0.5,
    },
    child: {
      boxSizing: 'border-box',
      overflow: 'hidden',
    },
    pressableSize: {
      height: '100%',
      width: '100%',
    },
    pressed: {
      opacity: 0.75,
    },
  }),
  stylex.create({
    container: {
      height: '100%',
      position: 'relative',
      width: '100%',
    },
    icon: {
      bottom: 4,
      end: 4,
      position: 'absolute',
    },
    overlay: {
      backgroundColor: 'var(--fds-black-alpha-30)',
      borderRadius: 4,
      bottom: 0,
      end: 0,
      opacity: 0,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    overlayVisible: {
      opacity: 1,
    },
  }),
  stylex.create({
    imageSelected: {
      margin: -2,
    },
    pressableSize: {
      height: '100%',
      width: '100%',
    },
    selectedInner: {
      borderStyle: 'solid',
      borderColor: 'var(--surface-background)',
      borderRadius: 4,
      borderWidth: 2,
    },
    selectedOuter: {
      borderStyle: 'solid',
      borderColor: 'var(--primary-icon)',
      borderRadius: 6,
      borderWidth: 2,
    },
  }),
  stylex.create({
    dot: {
      backgroundColor: 'var(--negative)',
      borderRadius: '50%',
      boxSizing: 'border-box',
      display: 'inline-block',
      height: '0.334em',
      marginInlineEnd: '0.4em',
      overflow: 'hidden',
      verticalAlign: 'middle',
      width: '0.334em',
    },
  }),
  stylex.create({
    icon: {
      height: '0.88em',
      marginInlineEnd: '.13em',
      marginInlineStart: '.34em',
      position: 'relative',
      top: '0.06em',
      width: '0.88em',
    },
  }),
  stylex.create({
    container: {
      borderRadius: 'var(--list-cell-corner-radius)',
    },
    footerMargin: {
      marginBottom: 16,
    },
    headerMargin: {
      marginTop: 4,
    },
  }),
  stylex.create({
    content: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      paddingInlineEnd: 'var(--card-padding-horizontal)',
      paddingTop: 'var(--card-padding-vertical)',
    },
    contentContainer: {
      paddingBottom: 'var(--card-padding-vertical)',
      paddingInlineStart: 'var(--card-padding-horizontal)',
    },
    heading: {
      borderRadius: 'initial',
      paddingInline: 'var(--card-padding-horizontal)',
      paddingBlock: 'var(--card-padding-vertical)',
    },
    startAddOnSpacing: {
      marginInlineStart: 12,
    },
    title: {
      marginInlineEnd: 12,
    },
  }),
  stylex.create({
    root: {
      marginTop: 16,
    },
  }),
  stylex.create({
    checkedWrapper: {
      margin: -2,
    },
    circle: {
      borderRadius: '50%',
      display: 'inline-block',
      flexShrink: 0,
      height: 20,
      position: 'relative',
      width: 20,
    },
    disabledBorder: {
      borderWidth: 2,
      borderStyle: 'solid',
      borderColor: 'var(--disabled-icon)',
    },
    uncheckedBorder: {
      borderWidth: 2,
      borderStyle: 'solid',
      borderColor: 'var(--placeholder-icon)',
    },
  }),
  stylex.create({
    addOn: {
      alignItems: 'center',
      display: 'flex',
      height: 59,
      justifyContent: 'center',
      marginInlineStart: 16,
      marginBlock: 14,
      width: 59,
    },
    placeholder: {
      alignItems: 'center',
      borderColor: 'var(--placeholder-image)',
      borderStyle: 'solid',
      borderWidth: 1,
      display: 'flex',
      height: 59,
      justifyContent: 'center',
      width: 59,
    },
  }),
  stylex.create({
    content: {
      marginInlineStart: 16,
    },
    contentConfirmText: {
      marginInlineEnd: 12,
    },
    contentHovered: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      height: 87,
      justifyContent: 'center',
    },
    contentHoveredError: {
      height: 109,
    },
    contentHoveredText: {
      marginInlineStart: 6,
    },
    previewText: {
      marginTop: 10,
    },
  }),
  stylex.create({
    addOnEnd: {
      marginInlineEnd: 16,
    },
    addOnFooter: {
      marginBottom: 12,
      marginInline: 16,
    },
    container: {
      borderColor: 'transparent',
      borderRadius: 0,
      borderStyle: 'solid',
      borderWidth: 1,
      minHeight: 'var(--list-cell-min-height)',
      ':first-child': {
        borderTopEndRadius: 'inherit',
        borderTopStartRadius: 'inherit',
      },
      ':last-child': {
        borderBottomEndRadius: 'inherit',
        borderBottomStartRadius: 'inherit',
      },
    },
    containerHovered: {
      borderColor: 'var(--blue-link)',
      borderStyle: 'dashed',
      borderWidth: 1,
    },
    listCell: {
      minHeight: 'var(--list-cell-min-height)',
      pointerEvents: 'none',
    },
    listCellConfirm: {
      pointerEvents: 'unset',
    },
    listCellError: {
      borderBottomColor: 'var(--negative)',
      borderBottomStyle: 'solid',
      borderBottomWidth: 1,
    },
  }),
  stylex.create({
    content: {
      marginInline: 20,
    },
    image: {
      maxHeight: 550,
      minHeight: 328,
    },
    text: {
      marginBottom: 32,
    },
  }),
  stylex.create({
    checkedIcon: {
      backgroundColor: 'var(--switch-active)',
      borderRadius: '50%',
      display: 'inline-block',
      height: 12,
      margin: 2,
      width: 12,
    },
    checkedIconDisabled: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    deselectedRadioBorder: {
      borderColor: 'var(--placeholder-icon)',
    },
    selectedRadioBorder: {
      borderColor: 'var(--switch-active)',
    },
  }),
  stylex.create({
    16: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    40: {
      alignItems: 'center',
      height: 40,
      justifyContent: 'center',
      width: 40,
    },
  }),
  stylex.create({
    16: {
      width: 16,
    },
    24: {
      width: 24,
    },
    32: {
      width: 32,
    },
    40: {
      width: 40,
    },
    48: {
      width: 48,
    },
  }),
  stylex.create({
    baseListCell: {
      flexBasis: 'auto',
      flexGrow: 1,
      flexShrink: 0,
      minHeight: 'var(--list-cell-min-height)',
      paddingBlock: 'var(--list-cell-padding-vertical)',
    },
    endAddOnSpacing: {
      marginInlineEnd: 12,
    },
    listItemPressable: {
      position: 'relative',
    },
    metaStartAddOnSpacing: {
      marginInlineStart: 4,
    },
    metaText: {
      marginBottom: 2,
      paddingTop: 2,
    },
    metaTextOverride: {
      paddingTop: 9,
    },
    pressable: {
      borderRadius: 0,
      display: 'flex',
      flexDirection: 'column',
      minHeight: 'var(--list-cell-min-height)',
      ':first-child': {
        borderTopEndRadius: 'inherit',
        borderTopStartRadius: 'inherit',
      },
      ':last-child': {
        borderBottomEndRadius: 'inherit',
        borderBottomStartRadius: 'inherit',
      },
    },
    selected: {
      backgroundColor: 'var(--hosted-view-selected-state)',
    },
    startAddOnSpacing: {
      marginInlineStart: 12,
    },
    startIndentStyles: {
      display: 'inline-block',
    },
    subtitleText: {
      marginBottom: 2,
      paddingTop: 2,
    },
    subtitleTextOverride: {
      paddingTop: 10,
    },
    titleText: {
      marginBottom: 2,
      paddingTop: 2,
    },
    withAddon: {
      paddingBlock: 'var(--list-cell-padding-vertical-with-addon)',
    },
  }),
  stylex.create({
    toast: {
      alignItems: 'center',
      backgroundColor: 'var(--card-background)',
      borderRadius: 4,
      boxShadow: '0 8px 20px 0  var(--shadow-2), 0 2px 4px 0 var(--shadow-1)',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'center',
      maxWidth: '100%',
      padding: '16px 20px',
    },
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      bottom: 0,
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      justifyContent: 'center',
      pointerEvents: 'none',
      position: 'fixed',
      start: 0,
      top: 0,
      zIndex: 401,
    },
  }),
  stylex.create({
    child: {
      display: 'flex',
      marginInline: 8,
    },
    container: {
      boxSizing: 'border-box',
      display: 'flex',
      marginInline: -8,
    },
  }),
  stylex.create({
    addonEnd: {
      alignItems: 'center',
      bottom: 0,
      display: 'flex',
      end: 0,
      position: 'absolute',
      top: 0,
    },
    addonStart: {
      alignItems: 'center',
      bottom: 0,
      display: 'flex',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    containerWithPadding: {
      alignItems: 'center',
      boxSizing: 'border-box',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      paddingInline: 104,
      position: 'relative',
      width: '100%',
    },
  }),
  stylex.create({
    pressable: {
      marginInline: -4,
      padding: 4,
    },
  }),
  stylex.create({
    root: {
      borderRadius: 'var(--nav-list-cell-corner-radius)',
      overflow: 'hidden',
    },
  }),
  stylex.create({
    container: {
      alignItems: 'center',
      borderRadius: 'var(--nav-list-cell-corner-radius)',
      display: 'flex',
      minHeight: 'var(--nav-list-cell-min-height)',
      paddingBlock: 'var(--nav-list-cell-padding-vertical)',
      width: '100%',
    },
    disabled: {
      opacity: 0.4,
    },
    fullWidth: {
      width: '100%',
    },
    inCardContainer: {
      borderRadius: 0,
    },
    leftAddOn: {
      marginInlineStart: 16,
    },
    rightAddOn: {
      marginInlineEnd: 16,
    },
    selected: {
      backgroundColor: 'var(--hosted-view-selected-state)',
    },
    subtitleText: {
      marginTop: 8,
    },
    textContent: {
      flexGrow: 1,
      flexShrink: 1,
      marginInline: 16,
    },
    withAddon: {
      paddingBlock: 'var(--nav-list-cell-padding-vertical-with-addon)',
    },
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      backgroundColor: 'var(--notification-badge)',
      borderRadius: '50%',
      boxSizing: 'border-box',
      display: 'inline-flex',
      height: 24,
      justifyContent: 'center',
      minWidth: 24,
      padding: '0 4px',
    },
    rootExtended: {
      borderRadius: 100,
    },
  }),
  stylex.create({
    li: {
      marginInlineStart: '1.8em',
      position: 'relative',
    },
    ol: {
      display: 'flex',
      flexDirection: 'column',
      listStyleType: 'decimal',
      margin: 0,
      padding: 0,
    },
    spacing: {
      marginTop: 16,
    },
  }),
  stylex.create({
    p: {
      display: 'flex',
      flexDirection: 'column',
      margin: 0,
    },
    spacing: {
      marginTop: 20,
    },
  }),
  stylex.create({
    popoverWithArrow: {
      filter: 'drop-shadow(0 0px 6px var(--shadow-2))',
    },
    root: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow: 'var(--shadow-elevated)',
    },
  }),
  stylex.create({
    end: {
      borderBottomEndRadius: 0,
    },
    middle: {},
    start: {
      borderBottomStartRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      borderTopEndRadius: 0,
    },
    middle: {},
    start: {
      borderTopStartRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      borderBottomEndRadius: 0,
    },
    middle: {},
    start: {
      borderTopEndRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      borderBottomStartRadius: 0,
    },
    middle: {},
    start: {
      borderTopStartRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      display: 'flex',
      height: 56,
      justifyContent: 'center',
      maxWidth: 'calc(100vw - 16px)',
      width: 375,
    },
  }),
  stylex.create({
    28: {
      width: 28,
    },
    40: {
      width: 40,
    },
    60: {
      width: 60,
    },
    100: {
      width: 100,
    },
    180: {
      width: 180,
    },
    260: {
      width: 260,
    },
  }),
  stylex.create({
    28: {
      width: 32,
    },
    40: {
      width: 44,
    },
    60: {
      width: 64,
    },
    100: {
      width: 108,
    },
    180: {
      width: 192,
    },
    260: {
      width: 272,
    },
  }),
  stylex.create({
    darkModeIcon: {
      color: 'var(--primary-icon)',
    },
    icon: {
      color: 'var(--fb-logo-color)',
      height: '100%',
      transform: 'scale(1.25)',
      width: '100%',
    },
  }),
  stylex.create({
    icon: {
      color: 'var(--primary-icon)',
      height: '100%',
      width: '100%',
    },
  }),
  stylex.create({
    darkModeIcon: {
      fill: 'var(--primary-icon)',
    },
    icon: {
      height: '100%',
      width: '100%',
    },
    mask: {
      fill: 'var(--always-white)',
    },
  }),
  stylex.create({
    darkModeIcon: {
      fill: 'var(--primary-icon)',
    },
    icon: {
      height: '100%',
      transform: 'scale(1.25)',
      width: '100%',
    },
  }),
  stylex.create({
    icon: {
      alignItems: 'center',
      display: 'flex',
      height: '100%',
      justifyContent: 'space-around',
      width: '100%',
    },
  }),
  stylex.create({
    icon: {
      color: 'var(--primary)',
      height: '100%',
      width: '100%',
    },
  }),
  stylex.create({
    darkModeIcon: {
      color: 'var(--primary-icon)',
    },
    icon: {
      color: 'rgb(37, 211, 102)',
      height: '100%',
      transform: 'scale(1.1)',
      width: '100%',
    },
  }),
  stylex.create({
    completed: {
      backgroundColor: 'var(--accent)',
    },
    root: {
      alignItems: 'stretch',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 0,
      flexShrink: 1,
      flexWrap: 'nowrap',
      justifyContent: 'stretch',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'relative',
      width: '100%',
      zIndex: 0,
    },
    step: {
      height: 2,
    },
    stepSpacedOut: {
      borderBottomEndRadius: 4,
      borderBottomStartRadius: 4,
      borderTopEndRadius: 4,
      borderTopStartRadius: 4,
    },
    stepSpacedOutFirst: {
      borderBottomStartRadius: 0,
      borderTopStartRadius: 0,
    },
    stepSpacedOutLast: {
      borderBottomEndRadius: 0,
      borderTopEndRadius: 0,
    },
    stepWrapper: {
      boxSizing: 'border-box',
    },
    stepWrapperSpacedOut: {
      paddingInlineEnd: 2,
      paddingInlineStart: 2,
    },
    stepWrapperSpacedOutFirst: {
      paddingInlineStart: 0,
    },
    stepWrapperSpacedOutLast: {
      paddingInlineEnd: 0,
    },
    uncompleted: {
      backgroundColor: 'var(--disabled-text)',
    },
  }),
  stylex.create({
    checkedIcon: {
      backgroundColor: 'var(--switch-active)',
      borderRadius: '50%',
      display: 'inline-block',
      margin: 2,
    },
    checkedIconDisabled: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    checkedIconLarge: {
      height: 16,
      width: 16,
    },
    checkedIconMedium: {
      height: 12,
      width: 12,
    },
    deselectedBorder: {
      borderColor: 'var(--placeholder-icon)',
    },
    root: {
      WebkitTapHighlightColor: 'transparent',
      alignItems: 'center',
      display: 'flex',
      touchAction: 'manipulation',
    },
    selectedBorder: {
      borderColor: 'var(--switch-active)',
    },
    textPositionEnd: {
      marginInlineStart: 8,
    },
  }),
  stylex.create({
    bugButtonWrapper: {
      end: 55,
      position: 'absolute',
      top: 14,
    },
    container: {
      display: 'flex',
      flexGrow: 1,
      maxWidth: 1464,
      minWidth: 0,
      position: 'relative',
      '@media (max-width: 1920px)': {
        maxWidth: 'none',
      },
    },
    containerWithHeader: {
      flexDirection: 'column',
    },
    detachedGradient: {
      start: 0,
      top: 'var(--header-height)',
    },
    endButton: {
      end: 0,
      position: 'absolute',
    },
    fixed: {
      bottom: 0,
      end: 0,
      minHeight: 'auto',
      position: 'fixed',
      start: 0,
      top: 'var(--header-height)',
    },
    gradient: {
      position: 'fixed',
    },
    headerButtons: {
      end: 14,
      position: 'absolute',
      start: 14,
      top: 14,
      '@media (max-width: 999px)': {
        end: 8,
        start: 8,
      },
    },
    root: {
      display: 'flex',
      justifyContent: 'center',
    },
    scrollable: {
      minHeight: 'calc(100vh - var(--header-height))',
    },
    startButton: {
      position: 'absolute',
      start: 0,
    },
  }),
  stylex.create({
    hideOnMobile: {
      '@media (max-width: 999px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    children: {
      borderBottomEndRadius: 4,
      borderBottomStartRadius: 4,
      flexBasis: 1105,
      overflow: 'hidden',
      '@media (max-width: 1105px)': {
        borderRadius: 0,
      },
    },
  }),
  stylex.create({
    inFixedLayoutStyle: {
      marginTop: 0,
    },
    inOneColumnLayout: {
      flexBasis: 640,
      marginTop: 24,
    },
    inScrollableLayoutStyle: {
      paddingInline: 20,
      '@media (max-width: 999px)': {
        flexBasis: 680,
        marginTop: 24,
      },
    },
    inTwoColumnLayout: {
      flexBasis: 720,
      marginTop: 62,
    },
    inTwoColumnWithHeaderLayout: {
      flexBasis: 720,
      marginTop: 32,
      '@media (max-width: 999px)': {
        flexGrow: 1,
      },
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      minWidth: 0,
    },
    whenRenderingPage: {
      marginTop: 0,
      paddingInline: 0,
      '@media (max-width: 999px)': {
        flexBasis: 'auto',
        flexGrow: 1,
        marginTop: 0,
      },
    },
    whenSideRailMain: {
      '@media (max-width: 999px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    multiPageView: {
      flexGrow: 1,
      overflow: 'hidden',
      '@media (max-width: 999px)': {
        overflow: 'visible',
      },
    },
  }),
  stylex.create({
    backButtonOnlyMobile: {
      display: 'none',
      '@media (max-width: 999px)': {
        display: 'block',
      },
    },
    content: {
      flexGrow: 1,
      paddingInline: 20,
    },
    footer: {
      '@media (max-width: 999px)': {
        backfaceVisibility: 'hidden',
        backgroundColor: 'var(--background-deemphasized)',
        bottom: 0,
        boxShadow: 'var(--scroll-shadow)',
        position: 'sticky',
      },
    },
    footerContent: {
      paddingInline: 20,
      paddingBlock: 'var(--page-footer-padding-vertical)',
    },
    header: {
      alignItems: 'center',
      display: 'flex',
      height: 56,
      paddingInline: 20,
      '@media (max-width: 999px)': {
        height: 'auto',
        marginBottom: 8,
      },
    },
    headerWithBackButton: {
      paddingInlineStart: 5,
    },
    mobileMaxWidth: {
      '@media (max-width: 999px)': {
        boxSizing: 'border-box',
        marginInline: 'auto',
        maxWidth: 680,
        width: '100%',
      },
    },
    root: {
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      paddingTop: 5,
      position: 'relative',
    },
    rootWithFooter: {
      '@media (min-width: 1024px)': {
        maxHeight: ['calc(100vh - var(--header-height))', 'calc(100dvh - var(--header-height))'],
      },
    },
    scrollableArea: {
      flexGrow: 1,
    },
  }),
  stylex.create({
    content: {
      paddingBottom: 20,
      paddingTop: 28,
      '@media (max-width: 999px)': {
        paddingTop: 60,
      },
    },
    displayNoneOnSmallViewport: {
      '@media (max-width: 999px)': {
        display: 'none',
      },
    },
    divider: {
      backgroundColor: 'var(--divider)',
      height: '100%',
      margin: 0,
      width: 1,
      '@media (min-width: 1160px)': {
        marginInline: 12,
      },
    },
    headerRow: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      paddingInline: 0,
      paddingBlock: 0,
      position: 'relative',
    },
    headerRowEndItem: {
      end: 0,
      position: 'absolute',
      top: '50%',
      transform: 'translateY(-50%)',
    },
    headerRowStartItem: {
      marginInlineEnd: 'auto',
    },
    root: {
      flexBasis: 360,
      flexShrink: 0,
      marginInlineEnd: 0,
      maxHeight: 'calc(100vh - var(--header-height))',
      overflow: 'hidden',
      position: 'sticky',
      top: 'var(--header-height)',
      '@media (max-width: 999px)': {
        alignSelf: 'flex-start',
        flexBasis: 710,
        flexShrink: 1,
        maxHeight: 'none',
        position: 'static',
      },
    },
    scrollableArea: {
      maxHeight: 'inherit',
      paddingInline: 20,
    },
  }),
  stylex.create({
    heading: {
      height: 25,
      marginBottom: 20,
      maxWidth: 300,
    },
    lastLine: {
      marginBottom: 24,
      maxWidth: 296,
    },
    line: {
      height: 10,
      marginBottom: 10,
      maxWidth: 600,
    },
    listItem: {
      height: 10,
      marginBottom: 15,
      maxWidth: 250,
    },
  }),
  stylex.create({
    container: {
      flexGrow: 1,
      position: 'relative',
    },
    icon: {
      position: 'absolute',
      start: 12,
      top: 10,
    },
    input: {
      appearance: 'none',
      backgroundColor: 'var(--hover-overlay)',
      borderWidth: 0,
      borderRadius: 18,
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      fontSize: 16,
      height: 36,
      paddingInlineEnd: 8,
      paddingInlineStart: 36,
      width: '100%',
      '::-webkit-search-cancel-button': {
        display: 'none',
      },
      '::-webkit-search-results-button': {
        display: 'none',
      },
    },
    x: {
      end: 6,
      padding: 8,
      position: 'absolute',
      top: 4,
    },
  }),
  stylex.create({
    action: {
      alignItems: 'flex-end',
      flexBasis: '33%',
      marginInlineStart: 'var(--section-header-addOnEnd-margin-horizontal)',
    },
    addOnEndCompactSize: {
      alignItems: 'flex-start',
      marginInlineStart: 'var(--section-header-addOnEnd-margin-horizontal)',
    },
    addOnStart: {
      marginInlineEnd: 'var(--section-header-addOnStart-margin-horizontal)',
    },
    headingAlignCenter: {
      alignItems: 'center',
    },
    headingCompactSize: {
      alignItems: 'flex-start',
    },
    iconButtonPadding: {
      paddingInline: 'var(--section-header-addOnEnd-button-padding-horizontal)',
      paddingBlock: 'var(--section-header-addOnEnd-button-padding-vertical)',
    },
    root: {
      paddingBottom: 'var(--section-header-padding-vertical)',
    },
    subtitle: {
      marginTop: 'var(--section-header-subtitle-margin-vertical)',
    },
    subtitleWithAddOnEnd: {
      marginTop: 'var(--section-header-subtitle-with-addOnEnd-margin-vertical)',
    },
  }),
  stylex.create({
    buttonDisabled: {
      cursor: 'not-allowed',
    },
    buttonExpanded: {
      minWidth: '100%',
    },
    disabled: {
      backgroundColor: 'var(--background-deemphasized)',
      color: 'var(--disabled-text)',
      cursor: 'not-allowed',
      ':hover': {
        borderColor: 'var(--secondary-button-stroke)',
      },
    },
    emptyOption: {
      paddingTop: 20,
    },
    focusedInputRowBorder: {
      borderColor: 'var(--border-focused)',
    },
    helperText: {
      display: 'flex',
      marginTop: 10,
    },
    icon: {
      alignItems: 'center',
      display: 'flex',
      lineHeight: 0,
      marginInlineStart: 8,
    },
    inputError: {
      borderColor: 'var(--negative)',
      ':hover': {
        borderColor: 'var(--negative)',
      },
    },
    inputRow: {
      backgroundColor: 'var(--card-background)',
      borderColor: 'var(--secondary-button-stroke)',
      borderRadius: 'var(--input-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 1,
      boxSizing: 'border-box',
      display: 'flex',
      flexWrap: 'nowrap',
      height: 64,
      justifyContent: 'space-between',
      overflow: 'hidden',
      paddingInline: 16,
      paddingBlock: 12,
      position: 'relative',
      width: '100%',
      zIndex: 0,
      ':hover': {
        borderColor: 'var(--border-focused)',
      },
    },
    label: {
      color: 'var(--placeholder-text)',
      cursor: 'inherit',
      end: 46,
      fontSize: 17,
      fontWeight: 'normal',
      lineHeight: 1.2941176470588236,
      maxWidth: '100%',
      overflow: 'hidden',
      pointerEvents: 'none',
      position: 'absolute',
      start: 16,
      textOverflow: 'ellipsis',
      top: 22,
      transformOrigin: 'top left',
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionProperty: 'transform',
      transitionTimingFunction: 'var(--fds-animation-move-in)',
      whiteSpace: 'nowrap',
    },
    labelError: {
      color: 'var(--negative)',
    },
    labelRTL: {
      transformOrigin: 'top right',
    },
    labelShrunk: {
      color: 'var(--secondary-text)',
      end: 'auto',
      transform: 'scale(0.75) translateY(-13px)',
      transitionTimingFunction: 'var(--fds-animation-move-out)',
    },
    selectedOption: {
      marginBottom: 5,
      marginTop: 0,
      paddingTop: 21,
    },
  }),
  stylex.create({
    hideOutline: {
      outline: 'none',
    },
    menuStyles: {
      width: '100%',
    },
  }),
  stylex.create({
    blueLink: {
      stroke: 'var(--blue-link)',
    },
    primary: {
      stroke: 'var(--switch-active)',
    },
    'primary-button': {
      stroke: 'var(--primary-button-text)',
    },
    secondary: {
      stroke: 'var(--switch-active)',
    },
    'secondary-button': {
      stroke: 'var(--switch-active)',
    },
  }),
  stylex.create({
    28: {
      width: 28,
    },
    40: {
      width: 40,
    },
    60: {
      width: 60,
    },
    100: {
      width: 100,
    },
    180: {
      width: 180,
    },
    260: {
      width: 260,
    },
  }),
  stylex.create({
    28: {
      marginInlineEnd: 12,
      marginTop: 12,
    },
    40: {
      marginInlineEnd: 20,
      marginTop: 20,
    },
    60: {
      marginInlineEnd: 22,
      marginTop: 22,
    },
    100: {
      marginInlineEnd: 35,
      marginTop: 35,
    },
    180: {
      marginInlineEnd: 64,
      marginTop: 64,
    },
    260: {
      marginInlineEnd: 93,
      marginTop: 93,
    },
  }),
  stylex.create({
    28: {
      marginBottom: 3,
      marginInlineStart: 3,
    },
    40: {
      marginBottom: 11,
      marginInlineStart: 11,
    },
  }),
  stylex.create({
    activeTabUnderline: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--accent)',
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      willChange: 'transform',
    },
  }),
  stylex.create({
    button: {
      flexBasis: 0,
      flexGrow: 1,
      flexShrink: 0,
      maxWidth: '100%',
      minWidth: 0,
    },
    buttonContent: {
      borderRadius: 0,
      height: '100%',
      padding: 12,
    },
  }),
  stylex.create({
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabledText: {
      color: 'var(--disabled-text)',
    },
    negative: {
      color: 'var(--negative)',
    },
    placeholderText: {
      color: 'var(--placeholder-text)',
    },
    placeholderTextOnMedia: {
      color: 'var(--placeholder-text-on-media)',
    },
    positive: {
      color: 'var(--positive)',
    },
    primaryButtonText: {
      color: 'var(--primary-button-text)',
    },
    primaryText: {
      color: 'var(--primary-text)',
    },
    primaryTextOnMedia: {
      color: 'var(--primary-text-on-media)',
    },
    secondaryButtonText: {
      color: 'var(--secondary-button-text)',
    },
    secondaryText: {
      color: 'var(--secondary-text)',
    },
    secondaryTextOnMedia: {
      color: 'var(--secondary-text-on-media)',
    },
    toastText_DO_NOT_USE_WILL_BE_DELETED_WITHOUT_NOTICE: {
      color: 'var(--toast-text)',
    },
    toastTextLink: {
      color: 'var(--toast-text-link)',
    },
    warning: {
      color: 'var(--warning)',
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily: 'var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--font-family-default) !important',
    },
    segoe: {
      fontFamily: 'var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--body-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--body-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--body-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--body-emphasized-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--body-emphasized-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--body-emphasized-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--headline1-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--headline1-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--headline1-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--headline2-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--headline2-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--headline2-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--headline3-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--headline3-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--headline3-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--meta-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--meta-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--meta-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--meta-emphasized-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--meta-emphasized-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--meta-emphasized-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--primary-label-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--primary-label-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--primary-label-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--secondary-label-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--secondary-label-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--secondary-label-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    apple: {
      fontFamily: 'var(--tertiary-label-font-family), var(--font-family-apple) !important',
    },
    default: {
      fontFamily: 'var(--tertiary-label-font-family), var(--font-family-default) !important',
    },
    windows: {
      fontFamily: 'var(--tertiary-label-font-family), var(--font-family-segoe) !important',
    },
  }),
  stylex.create({
    body: {
      fontSize: 'var(--body-font-size)',
      fontWeight: 'var(--body-font-weight)',
    },
    bodyEmphasized: {
      fontSize: 'var(--body-emphasized-font-size)',
      fontWeight: 'var(--body-emphasized-font-weight)',
    },
    headline1: {
      fontSize: 'var(--headline1-font-size)',
      fontWeight: 'var(--headline1-font-weight)',
    },
    headline2: {
      fontSize: 'var(--headline2-font-size)',
      fontWeight: 'var(--headline2-font-weight)',
    },
    headline3: {
      fontSize: 'var(--headline3-font-size)',
      fontWeight: 'var(--headline3-font-weight)',
    },
    meta: {
      fontSize: 'var(--meta-font-size)',
      fontWeight: 'var(--meta-font-weight)',
    },
    metaEmphasized: {
      fontSize: 'var(--meta-emphasized-font-size)',
      fontWeight: 'var(--meta-emphasized-font-weight)',
    },
    primaryLabel: {
      fontSize: 'var(--primary-label-font-size)',
      fontWeight: 'var(--primary-label-font-weight)',
    },
    secondaryLabel: {
      fontSize: 'var(--secondary-label-font-size)',
      fontWeight: 'var(--secondary-label-font-weight)',
    },
    tertiaryLabel: {
      fontSize: 'var(--tertiary-label-font-size)',
      fontWeight: 'var(--tertiary-label-font-weight)',
    },
  }),
  stylex.create({
    input: {
      fontFamily: 'var(--text-input-field-font-family), system-ui, sans-serif !important',
      ':disabled': {
        color: 'var(--secondary-text)',
        opacity: 1,
      },
    },
    inputRow: {
      paddingInlineEnd: 'var(--text-input-multi-padding-scrollbar)',
    },
  }),
  stylex.create({
    root: {
      borderRadius: 'var(--text-badge-corner-radius)',
      display: 'inline-block',
      paddingInline: 'var(--text-badge-padding-horizontal)',
      paddingBlock: 'var(--text-badge-padding-vertical)',
    },
  }),
  stylex.create({
    attention: {
      backgroundColor: 'var(--text-badge-attention-background)',
    },
    critical: {
      backgroundColor: 'var(--text-badge-critical-background)',
    },
    info: {
      backgroundColor: 'var(--text-badge-info-background)',
    },
    success: {
      backgroundColor: 'var(--text-badge-success-background)',
    },
  }),
  stylex.create({
    addOnEndImageWrapper: {
      alignItems: 'center',
      display: 'flex',
      paddingInlineStart: 16,
      paddingTop: 0,
    },
    image: {
      borderRadius: 2,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
    },
    input: {
      fontFamily: 'var(--text-input-field-font-family), system-ui, sans-serif !important',
      '::-ms-reveal': {
        display: 'none',
      },
      ':disabled': {
        color: 'var(--secondary-text)',
        opacity: 1,
      },
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    topText: {
      marginBottom: 16,
    },
  }),
  stylex.create({
    headline1Body: {
      marginTop: 20,
    },
    headline2Body: {
      marginTop: 20,
    },
    headline3Body: {
      marginTop: 16,
    },
    headline3Meta: {
      marginTop: 12,
    },
  }),
  stylex.create({
    link: {
      wordBreak: 'keep-all',
    },
  }),
  stylex.create({
    container: {
      isolation: 'isolate',
    },
    shadow: {
      borderRadius: 4,
    },
  }),
  stylex.create({
    view: {
      borderColor: 'var(--secondary-button-stroke)',
      borderRadius: 'var(--card-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 1,
      boxShadow: 'none',
    },
  }),
  stylex.create({
    view: {
      borderColor: 'var(--secondary-button-stroke)',
      borderRadius: 'var(--card-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 1,
      boxShadow: 'none',
      marginTop: 'var(--typeahead-list-outer-padding-vertical)',
    },
  }),
  stylex.create({
    root: {
      marginBottom: -1,
      width: '100%',
    },
  }),
  stylex.create({
    darkMode: {
      color: '#F1F4F7',
    },
    lightMode: {
      color: '#4A5A68',
    },
    size: {
      height: 12,
      width: 168,
    },
  }),
  stylex.create({
    unused: {
      display: 'flex',
    },
  }),
  stylex.create({
    content: {
      alignItems: 'stretch',
      borderRadius: 'inherit',
      boxSizing: 'border-box',
      display: 'inherit',
      flexDirection: 'column',
      isolation: 'isolate',
      justifyContent: 'flex-start',
      overflow: 'hidden',
      position: 'relative',
    },
  }),
  stylex.create({
    28: {
      borderRadius: '50%',
      height: 28,
      position: 'absolute',
      start: -4.102,
      top: -4.102,
      width: 28,
    },
    40: {
      borderRadius: '50%',
      height: 40,
      position: 'absolute',
      start: -5.86,
      top: -5.86,
      width: 40,
    },
    60: {
      borderRadius: '50%',
      height: 60,
      position: 'absolute',
      start: -8.79,
      top: -8.79,
      width: 60,
    },
    100: {
      borderRadius: '50%',
      height: 100,
      position: 'absolute',
      start: -14.65,
      top: -14.65,
      width: 100,
    },
    180: {
      borderRadius: '50%',
      height: 180,
      position: 'absolute',
      start: -26.37,
      top: -26.37,
      width: 180,
    },
    260: {
      borderRadius: '50%',
      height: 260,
      position: 'absolute',
      start: -38.09,
      top: -38.09,
      width: 260,
    },
  }),
  stylex.create({
    dense: {
      borderRadius: 7,
      height: 14,
      width: 14,
    },
    large: {
      borderRadius: 9,
      height: 18,
      width: 18,
    },
    regular: {
      borderRadius: 8,
      height: 16,
      width: 16,
    },
  }),
  stylex.create({
    shadow: {
      borderWidth: 0,
      boxShadow: 'var(--shadow-elevated)',
      boxSizing: 'content-box',
      height: '100%',
      position: 'absolute',
      start: '0',
      top: '0',
      width: '100%',
      zIndex: -1,
    },
  }),
  stylex.create({
    paragraph: {
      margin: 0,
      wordWrap: 'break-word',
    },
    paragraphSpaced: {
      marginTop: '1em',
    },
  }),
  stylex.create({
    li: {
      marginInlineStart: '2em',
    },
    olList: {
      listStyleType: 'decimal',
    },
    ulList: {
      listStyleType: 'disc',
    },
  }),
  stylex.create({
    button: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    glimmer0: {
      borderRadius: 8,
      height: 92,
      width: 183,
    },
    glimmer1: {
      borderRadius: 8,
      height: 24,
      marginInlineEnd: 32,
      width: 276,
    },
    glimmer2: {
      borderRadius: 8,
      height: 40,
      width: 276,
    },
  }),
  stylex.create({
    facepileWrapper: {
      paddingInline: 44,
      transform: 'translateY(-80%)',
    },
    image: {
      borderRadius: 8,
      marginBottom: 8,
      marginInlineEnd: 8,
      marginTop: 8,
    },
    spacing: {
      paddingTop: 2,
    },
  }),
  stylex.create({
    footer: {
      display: 'flex',
      flexDirection: 'column',
      maxWidth: 900,
      paddingBlock: 36,
      '@media (min-width: 900px)': {
        flexDirection: 'row',
      },
    },
    itemPadding: {
      paddingInline: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    header: {
      padding: '0px 0px 20px 0px',
    },
    helplines_centralize: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginBottom: 8,
      marginTop: 8,
    },
    helplines_divider: {
      backgroundColor: 'var(--comment-background)',
      height: 1,
      marginTop: 16,
      width: '100%',
    },
    helplines_header: {
      padding: '0px 0px 0px 0px',
    },
    helplines_screen: {
      padding: '20px 16px',
    },
    helplines_textContainer: {
      padding: '25px 0px 10px 0px',
    },
    screen: {
      padding: '20px 16px',
    },
    section: {
      padding: '28px 16px 0px 0px',
    },
    separator: {
      marginBottom: 4,
      marginTop: 16,
    },
  }),
  stylex.create({
    emotionalHealthGlimmer: {
      paddingTop: 5,
    },
  }),
  stylex.create({
    botttomLink: {
      padding: '0px 0px 20px 0px',
    },
  }),
  stylex.create({
    emotionalHealthGlimmer: {
      paddingTop: 5,
    },
    image: {
      padding: '0px 0px 20px 0px',
    },
  }),
  stylex.create({
    botttomLink: {
      padding: '0px 0px 20px 0px',
    },
  }),
  stylex.create({
    dialog: {
      width: '100%',
    },
    illustration: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      paddingBottom: 2,
      paddingTop: 20,
    },
  }),
  stylex.create({
    footer: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      paddingBottom: 2,
    },
  }),
  stylex.create({
    header: {
      padding: '0px 16px',
    },
    headerpadded: {
      padding: '0px 16px',
      paddingTop: 20,
    },
    privacyicon: {
      marginBottom: 2,
    },
  }),
  stylex.create({
    list: {
      padding: '20px 0px 40px 0px',
    },
  }),
  stylex.create({
    header: {
      padding: '0px 0px 25px 0px',
    },
    screen: {
      padding: '20px 16px',
    },
  }),
  stylex.create({
    botttomLink: {
      padding: '0px 0px 20px 0px',
    },
    emergencyservicesbox: {
      padding: '16px 16px 0px 16px',
    },
  }),
  stylex.create({
    bulletitem: {
      margin: 4,
    },
    emotionalHealthGlimmer: {
      paddingTop: 20,
    },
  }),
  stylex.create({
    divider: {
      marginInline: 20,
    },
    header: {
      padding: '20px 16px 8px 16px',
    },
    privacyicon: {
      marginBottom: 2,
    },
    privacyText: {
      marginBottom: 16,
    },
    videoContent: {
      height: 264,
    },
    videoWrapper: {
      padding: 16,
    },
  }),
  stylex.create({
    darkMode: {
      filter: 'invert(1) hue-rotate(180deg)',
    },
    form: {
      display: 'none',
    },
    hideMe: {
      position: 'absolute',
      start: -9999,
      top: -9999,
    },
    iframe: {
      borderStyle: 'none',
      borderRadius: '8px',
      width: '100%',
    },
  }),
  stylex.create({
    darkMode: {
      filter: 'invert(1) hue-rotate(180deg)',
    },
    hideMe: {
      position: 'absolute',
      start: -9999,
      top: -9999,
    },
    iframe: {
      borderStyle: 'none',
      borderRadius: '8px',
      width: '100%',
    },
  }),
  stylex.create({
    darkMode: {
      filter: 'invert(1) hue-rotate(180deg)',
    },
    fullHeight: {
      height: '100vh',
    },
    iframe: {
      borderStyle: 'none',
      display: 'block',
      minHeight: 'calc(100vh - var(--header-height))',
      width: '100%',
    },
    parent: {
      height: '100%',
      maxWidth: '100%',
      width: '100vw',
    },
  }),
  stylex.create({
    close: {
      margin: '20px',
      textAlign: 'end',
    },
    row: {
      margin: '20px 20px 20px',
    },
  }),
  stylex.create({
    ellipsis: {
      end: 0,
      overflow: 'hidden',
      position: 'absolute',
    },
    oneLine: {
      textOverflow: 'ellipsis',
      whiteSpace: 'nowrap',
    },
    root: {
      display: 'block',
      overflow: 'hidden',
      position: 'relative',
    },
  }),
  stylex.create({
    badge: {
      borderRadius: '50%',
      display: 'flex',
      end: 1.5,
      overflow: 'hidden',
      position: 'absolute',
      top: 0,
    },
    icon: {
      display: 'flex',
      height: 16,
    },
    iconAfterLabel: {
      marginInlineStart: 6,
    },
    iconBeforeLabel: {
      marginInlineEnd: 6,
    },
    profile: {
      display: 'flex',
      marginInlineEnd: 6,
      marginInlineStart: -8,
    },
    profileDisabled: {
      opacity: 0.3,
    },
  }),
  stylex.create({
    pressed: {
      transform: 'scale(0.96)',
    },
    root: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: 18,
      display: 'flex',
      flexDirection: 'row',
      height: 36,
      justifyContent: 'center',
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      position: 'relative',
      width: '100%',
    },
    selected: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
    },
  }),
  stylex.create({
    buttons: {
      display: 'flex',
      justifyContent: 'space-evenly',
      margin: '20px',
    },
    row: {
      margin: '20px 20px 20px',
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    defaultHoveredStyle: {
      backgroundColor: 'var(--hover-overlay)',
    },
    defaultPressedStyle: {
      backgroundColor: 'var(--press-overlay)',
    },
    overlay: {
      borderRadius: 'inherit',
      bottom: 0,
      end: 0,
      opacity: 0,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    overlayVisible: {
      opacity: 1,
      transitionDuration: '0ms',
    },
  }),
  stylex.create({
    dialogRoot: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
    },
    mask: {
      backgroundColor: 'var(--web-wash)',
      bottom: 0,
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
    },
    pushViewBackgroundWash: {
      backgroundColor: 'var(--web-wash)',
    },
    view: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh',
      position: 'relative',
      zIndex: 0,
    },
    viewWithTabBar: {
      minHeight: 'calc(100vh - var(--header-height))',
      top: 'var(--header-height)',
    },
  }),
  stylex.create({
    buttonFocused: {
      opacity: 1,
    },
    closeButton: {
      height: 40,
      opacity: 0,
      position: 'fixed',
      start: 16,
      top: 'calc((var(--header-height) - 40px)/2)',
      width: 40,
      zIndex: 1,
    },
  }),
  stylex.create({
    block: {
      width: 784,
      '@media (max-width: 784px)': {
        width: 'auto',
      },
    },
    columnPrimary: {
      width: 500,
    },
    columns: {
      display: 'flex',
      height: '100%',
      overflow: 'hidden',
      '@media (max-width: 784px)': {
        flexDirection: 'column',
        height: 'auto',
      },
    },
    columnSecondary: {
      width: 284,
    },
    primaryColumn: {
      '@media (max-width: 784px)': {
        height: '100%',
        width: 'auto',
      },
    },
    reverse: {
      flexDirection: 'row-reverse',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      height: 'calc(75vh - 60px)',
      '@media (max-width: 784px)': {
        height: 'calc(100vh - 60px)',
      },
    },
    secondaryColumn: {
      '@media (max-width: 784px)': {
        display: 'none',
      },
    },
  }),
  stylex.create({
    addOn: {
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      marginInlineStart: 8,
    },
    nonBreakingSpace: {
      visibility: 'hidden',
      width: 0,
    },
    textFlexFixForIE: {
      flexBasis: 'auto',
    },
  }),
  stylex.create({
    ltr: {
      direction: 'ltr',
    },
    rtl: {
      direction: 'rtl',
    },
  }),
  stylex.create({
    apple: {
      MozOsxFontSmoothing: 'grayscale',
      WebkitFontSmoothing: 'antialiased',
      fontFamily: 'var(--font-family-apple)',
    },
    default: {
      fontFamily: 'var(--font-family-default)',
    },
    segoe: {
      fontFamily: 'var(--font-family-segoe)',
    },
  }),
  stylex.create({
    default: {
      fontFamily: 'Facebook Sans, var(--font-family-default) !important',
    },
  }),
  stylex.create({
    animationContainer: {
      bottom: 0,
      left: 0,
      pointerEvents: 'none',
      position: 'fixed',
      right: 0,
      textAlign: 'center',
      top: 'var(--header-height)',
      zIndex: 2,
    },
  }),
  stylex.create({
    noOutline: {
      outline: 'none',
    },
  }),
  stylex.create({
    downArrow: {
      bottom: -10,
      marginInlineStart: -7,
      start: '50%',
    },
    focusArrow: {
      backgroundColor: 'var(--surface-background)',
      borderWidth: 1,
      borderStyle: 'solid',
      borderColor: 'var(--warning)',
      borderRadius: 3,
      height: 8,
      lineHeight: 0,
      padding: '2px 2px 3px 3px',
      position: 'absolute',
      width: 8,
    },
    leftArrow: {
      marginTop: -8,
      start: -10,
      top: '50%',
    },
    rightArrow: {
      end: -10,
      marginTop: -8,
      top: '50%',
    },
    upArrow: {
      marginInlineStart: -7,
      start: '50%',
      top: -10,
    },
  }),
  stylex.create({
    visuallyHidden: {
      clip: 'rect(0, 0, 0, 0)',
      clipPath: 'inset(50%)',
      height: 1,
      overflow: 'hidden',
      position: 'absolute',
      width: 1,
    },
  }),
  stylex.create({
    borderRadiusCorrectionContainer: {
      height: '100%',
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
    boxShadowContainer: {
      height: '100%',
      opacity: 1,
      overflowAnchor: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      transformOrigin: '0 0',
      width: '100%',
    },
    hidden: {
      visibility: 'hidden',
    },
    minHeight: {
      minHeight: 'inherit',
    },
    root: {
      boxSizing: 'border-box',
      position: 'relative',
      transformOrigin: '0 0',
    },
    scaleCorrectionContainer: {
      boxSizing: 'border-box',
      height: '100%',
      position: 'relative',
      transformOrigin: '0 0',
      width: '100%',
    },
    showOverflow: {
      overflow: 'visible',
    },
  }),
  stylex.create({
    borderRadiusCorrectionContainer: {
      height: '100%',
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
    boxShadowContainer: {
      height: '100%',
      opacity: 1,
      position: 'absolute',
      start: 0,
      top: 0,
      transformOrigin: '0 0',
      width: '100%',
    },
    root: {
      boxSizing: 'border-box',
      position: 'relative',
      transformOrigin: '0 0',
    },
    scaleCorrectionContainer: {
      boxSizing: 'border-box',
      height: '100%',
      position: 'relative',
      transformOrigin: '0 0',
      width: '100%',
    },
    showOverflow: {
      overflow: 'visible',
    },
  }),
  stylex.create({
    freezesLayoutOnUnmount: {
      position: 'relative',
    },
    hidden: {
      display: 'none',
    },
    root: {
      alignContent: 'inherit',
      alignItems: 'inherit',
      alignSelf: 'inherit',
      display: 'inherit',
      flexBasis: 'inherit',
      flexDirection: 'inherit',
      flexGrow: 'inherit',
      flexShrink: 'inherit',
      height: 'inherit',
      justifyContent: 'inherit',
      maxHeight: 'inherit',
      maxWidth: 'inherit',
      minHeight: 'inherit',
      minWidth: 'inherit',
      width: 'inherit',
    },
    unmounting: {
      pointerEvents: 'none',
    },
  }),
  stylex.create({
    freezesLayoutOnUnmount: {
      position: 'relative',
    },
    hidden: {
      display: 'none',
    },
    root: {
      alignContent: 'inherit',
      alignItems: 'inherit',
      alignSelf: 'inherit',
      display: 'inherit',
      flexBasis: 'inherit',
      flexDirection: 'inherit',
      flexGrow: 'inherit',
      flexShrink: 'inherit',
      height: 'inherit',
      justifyContent: 'inherit',
      maxHeight: 'inherit',
      maxWidth: 'inherit',
      minHeight: 'inherit',
      minWidth: 'inherit',
      width: 'inherit',
    },
  }),
  stylex.create({
    button: {
      marginInlineStart: 12,
    },
    closeButton: {
      end: 8,
      position: 'absolute',
      top: 8,
    },
    content: {
      alignItems: 'center',
      backgroundColor: 'var(--comment-background)',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      borderTopColor: 'var(--media-inner-border)',
      display: 'flex',
      flexDirection: 'row',
      padding: '20px',
    },
    profilePhoto: {
      marginInlineEnd: 12,
    },
    text: {
      display: 'flex',
      flexGrow: 1,
    },
  }),
  stylex.create({
    absoluteFill: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    absoluteTop: {
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    heightGetter: {
      lineHeight: 0,
      position: 'relative',
    },
    heightSetter: {
      overflow: 'hidden',
      visibility: 'hidden',
    },
    moreTab: {
      alignItems: 'center',
      display: 'inline-flex',
      justifyContent: 'flex-start',
    },
    moreWrapper: {
      display: 'inline-block',
      overflow: 'hidden',
      position: 'relative',
      verticalAlign: 'top',
    },
    root: {
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
      zIndex: 0,
    },
    tab: {
      alignItems: 'center',
      display: 'inline-flex',
      float: 'start',
      justifyContent: 'flex-start',
      verticalAlign: 'top',
    },
  }),
  stylex.create({
    container: {
      height: 0,
      position: 'relative',
      width: '100%',
    },
    content: {
      alignItems: 'stretch',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      bottom: 0,
      boxSizing: 'border-box',
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 0,
    },
  }),
  stylex.create({
    root: {
      boxSizing: 'border-box',
      flexShrink: 0,
      position: 'relative',
    },
  }),
  stylex.create({
    root: {
      left: 0,
      marginRight: -9999,
      position: 'absolute',
      top: 0,
    },
  }),
  stylex.create({
    container: {
      height: '100%',
      width: '100%',
      willChange: 'opacity, transform',
    },
    root: {
      position: 'relative',
    },
  }),
  stylex.create({
    anchor: {
      alignItems: 'flex-start',
      boxSizing: 'border-box',
      display: 'flex',
      justifyContent: 'center',
      minHeight: 0,
      minWidth: 0,
      pointerEvents: 'none',
    },
    dialog: {
      boxSizing: 'content-box',
      display: 'flex',
      flexDirection: 'column',
      outline: 'none',
      overflow: 'hidden',
      pointerEvents: 'all',
    },
    root: {
      alignItems: 'stretch',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
    },
    rootWithDeprecatedStyles: {
      flexGrow: 0,
      minHeight: '100vh',
    },
  }),
  stylex.create({
    divider: {
      backgroundColor: 'var(--divider)',
      boxSizing: 'border-box',
      height: 1,
    },
    reset: {
      backgroundColor: 'transparent',
      borderWidth: 0,
      margin: 0,
    },
  }),
  stylex.create({
    detached: {
      MsOverflowStyle: 'none',
      height: '100%',
      overflow: 'auto',
      position: 'fixed',
      scrollbarWidth: 'none',
      start: 0,
      top: 0,
      width: '100%',
      '::-webkit-scrollbar': {
        display: 'none',
        height: 0,
        width: 0,
      },
    },
  }),
  stylex.create({
    focused: {
      outlineWidth: 2,
      outlineStyle: 'solid',
      outlineColor: 'Highlight',
      '@media (-webkit-min-device-pixel-ratio: 0)': {
        outline: '5px auto -webkit-focus-ring-color',
      },
    },
    newFocused: {
      boxShadow: '0 0px 0px 2px var(--always-white), 0 0 0 4px var(--base-blue)',
      outline: 'none',
    },
    newFocusedInset: {
      boxShadow: 'inset 0 0 0 2px var(--base-blue), inset 0 0px 0px 4px var(--always-white)',
      outline: 'none',
    },
    newFocusedLink: {
      outline: 'var(--base-blue) auto 2px',
    },
    unfocused: {
      outline: 'none',
    },
  }),
  stylex.create({
    hideOutline: {
      outline: 'none',
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap',
      width: '100%',
    },
  }),
  stylex.create({
    buttonWrapper: {
      opacity: 1,
      position: 'absolute',
      top: '50%',
      transitionDuration: '0.3s',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'ease',
      zIndex: 1,
    },
    card: {
      flexGrow: 0,
      flexShrink: 0,
      minWidth: 0,
      scrollSnapAlign: 'start',
    },
    cardExpanding: {
      display: 'flex',
    },
    cardRTL: {
      scrollSnapAlign: 'end',
    },
    containerPaddingDefault: {
      paddingBottom: 8,
      paddingTop: 8,
    },
    hidden: {
      opacity: 0,
      pointerEvents: 'none',
    },
    scrollContainer: {
      marginBottom: -8,
      marginTop: -8,
    },
    scrollView: {
      boxSizing: 'border-box',
      marginBottom: 0,
      paddingBottom: 8,
      paddingTop: 8,
      scrollbarWidth: 'none',
    },
    scrollViewNoScroll: {
      overflow: 'hidden',
    },
    scrollViewSnap: {
      scrollSnapType: 'x mandatory',
    },
  }),
  stylex.create({
    0: {
      marginInlineEnd: 0,
    },
    4: {
      marginInlineEnd: 4,
      ':last-of-type': {
        marginInlineEnd: 0,
      },
    },
    8: {
      marginInlineEnd: 8,
      ':last-of-type': {
        marginInlineEnd: 0,
      },
    },
    12: {
      marginInlineEnd: 12,
      ':last-of-type': {
        marginInlineEnd: 0,
      },
    },
    16: {
      marginInlineEnd: 16,
      ':last-of-type': {
        marginInlineEnd: 0,
      },
    },
  }),
  stylex.create({
    root: {
      color: 'inherit',
      fontSize: 'inherit',
      fontWeight: 'inherit',
      outline: 'none',
    },
  }),
  stylex.create({
    contain: {
      objectFit: 'contain',
    },
    cover: {
      objectFit: 'cover',
    },
    fill: {
      objectFit: 'fill',
    },
  }),
  stylex.create({
    defaultCursor: {
      cursor: 'default',
    },
    disabled: {
      textDecoration: 'none',
    },
    disabledColor: {
      color: 'var(--disabled-text)',
    },
    disabledLink: {
      opacity: 0.5,
    },
    expanding: {
      display: 'inline-flex',
    },
    link: {
      ':hover': {
        textDecoration: 'underline',
      },
    },
    linkColor: {
      color: 'var(--blue-link)',
    },
    root: {
      display: 'inline',
      position: 'relative',
      userSelect: 'none',
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    defaultHoveredStyle: {
      backgroundColor: 'var(--hover-overlay)',
    },
    defaultPressedStyle: {
      backgroundColor: 'var(--press-overlay)',
    },
    overlay: {
      borderRadius: 'inherit',
      bottom: 0,
      end: 0,
      opacity: 0,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    overlayVisible: {
      opacity: 1,
      transitionDuration: '0ms',
    },
  }),
  stylex.create({
    checkbox: {
      cursor: 'pointer',
      height: '100%',
      margin: 0,
      opacity: 0.001,
      outline: 'none',
      padding: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
    wrapper: {
      position: 'relative',
    },
  }),
  stylex.create({
    root: {
      WebkitTapHighlightColor: 'transparent',
      boxSizing: 'border-box',
      touchAction: 'manipulation',
      ':disabled': {
        cursor: 'not-allowed',
      },
    },
    zIndex: {
      zIndex: 1,
    },
  }),
  stylex.create({
    radio: {
      cursor: 'pointer',
      height: '100%',
      margin: 0,
      opacity: 0,
      outline: 'none',
      padding: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
    wrapper: {
      position: 'relative',
    },
  }),
  stylex.create({
    switch: {
      cursor: 'pointer',
      height: '100%',
      margin: 0,
      opacity: 0.001,
      outline: 'none',
      padding: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
    wrapper: {
      position: 'relative',
    },
  }),
  stylex.create({
    unresizable: {
      resize: 'none',
    },
  }),
  stylex.create({
    ellipsis: {
      end: 0,
      overflow: 'hidden',
      position: 'absolute',
    },
    multiLine: {
      display: 'block',
      maxWidth: '100%',
      overflow: 'hidden',
    },
    oneLine: {
      display: 'block',
      maxWidth: '100%',
      overflow: 'hidden',
      textOverflow: 'ellipsis',
      whiteSpace: 'nowrap',
    },
    root: {
      display: 'block',
      overflow: 'visible',
      position: 'relative',
    },
  }),
  stylex.create({
    bottomAddOn: {
      display: 'flex',
      flexDirection: 'column',
    },
    bottomAddOnResponsive: {
      flexGrow: 1,
    },
    item: {
      display: 'flex',
    },
    root: {
      alignItems: 'stretch',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      justifyContent: 'center',
      minWidth: 0,
    },
    textContent: {
      flexGrow: 1,
    },
    textContentContainer: {
      flexBasis: 'auto',
    },
    textWithResponsiveAddOnBottom: {
      flexBasis: '50%',
      maxWidth: '100%',
      minWidth: '50%',
    },
  }),
  stylex.create({
    overlay: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh',
      position: 'relative',
    },
  }),
  stylex.create({
    content: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh',
      position: 'relative',
    },
    contentDvh: {
      '@supports (min-height: 100dvh)': {
        minHeight: '100dvh',
      },
    },
    contentDvhWhenNarrow: {
      '@media (max-width: 679px)': {
        minHeight: ['100vh', '100dvh'],
      },
    },
    hidden: {
      visibility: 'hidden',
    },
    mask: {
      bottom: 0,
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
    },
    maskOverlay: {
      backgroundColor: 'var(--overlay-alpha-80)',
    },
    root: {
      position: 'relative',
    },
    rootStatic: {
      position: 'static',
    },
  }),
  stylex.create({
    'above-everything': {
      zIndex: 1,
    },
    'above-nav': {
      zIndex: 3,
    },
    normal: {
      zIndex: 0,
    },
  }),
  stylex.create({
    mask: {
      backgroundColor: 'var(--overlay-alpha-80)',
      bottom: 0,
      end: 0,
      position: 'fixed',
      start: 0,
      top: 0,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh',
      position: 'relative',
    },
    rootAboveClassicContextualLayer: {
      position: 'relative',
      zIndex: 501,
    },
    rootAboveCometContextualLayer: {
      position: 'relative',
      zIndex: 1,
    },
  }),
  stylex.create({
    overlay: {
      display: 'flex',
      flexDirection: 'column',
      minHeight: '100vh',
      position: 'relative',
      zIndex: 400,
    },
  }),
  stylex.create({
    page: {
      borderRadius: 'inherit',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      transformOrigin: 'top left',
    },
    pageInactive: {
      display: 'none',
      left: 0,
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
      zIndex: 1,
    },
    root: {
      alignItems: 'stretch',
      clipPath: 'inset(0px 0px 0px 0px)',
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
      transformOrigin: 'top left',
    },
  }),
  stylex.create({
    page: {
      opacity: 0,
      pointerEvents: 'none',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'opacity, transform',
      transitionTimingFunction: 'var(--fds-soft)',
    },
    pageAbsolute: {
      position: 'absolute',
      start: 0,
    },
    pageFullWidth: {
      width: '100%',
    },
    pageHidden: {
      visibility: 'hidden',
    },
    pageVisible: {
      opacity: 1,
      pointerEvents: 'all',
    },
    root: {
      height: 'auto',
      outline: 'none',
      overflow: 'hidden',
      position: 'relative',
      width: 'auto',
    },
    rootWithAnimations: {
      transform: 'translateZ(1px)',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'height, width',
      transitionTimingFunction: 'var(--fds-soft)',
    },
  }),
  stylex.create({
    bottom: {
      bottom: 0,
    },
    top: {
      top: 0,
    },
  }),
  stylex.create({
    root: {
      position: 'relative',
    },
  }),
  stylex.create({
    arrow: {
      borderBottomColor: 'transparent',
      borderInlineEndColor: 'transparent',
      borderStartColor: 'transparent',
      borderStyle: 'solid',
      borderTopColor: 'transparent',
      borderWidth: 6,
      pointerEvents: 'none',
      position: 'absolute',
    },
    arrowAlignBottom: {
      bottom: 0,
    },
    arrowAlignEnd: {
      end: 0,
    },
    arrowAlignHorizontalCenter: {
      start: 'calc(50% - 6px)',
    },
    arrowAlignStart: {
      start: 0,
    },
    arrowAlignTop: {
      top: 0,
    },
    arrowAlignVerticalCenter: {
      top: 'calc(50% - 6px)',
    },
  }),
  stylex.create({
    above: {
      marginBottom: 15,
    },
    below: {
      marginTop: 15,
    },
    end: {
      marginInlineStart: 15,
    },
    start: {
      marginInlineEnd: 15,
    },
  }),
  stylex.create({
    above: {
      borderBottomColor: 'var(--card-background)',
      borderStartColor: 'var(--card-background)',
      boxShadow: '-1px 1px 1px var(--shadow-inset)',
      top: '100%',
    },
    below: {
      borderInlineEndColor: 'var(--card-background)',
      borderTopColor: 'var(--card-background)',
      bottom: '100%',
      boxShadow: '1px -1px 1px var(--shadow-inset)',
    },
    end: {
      borderBottomColor: 'var(--card-background)',
      borderStartColor: 'var(--card-background)',
      boxShadow: '-1px 1px 1px var(--shadow-inset)',
      end: '100%',
    },
    start: {
      borderBottomColor: 'var(--card-background)',
      borderInlineEndColor: 'var(--card-background)',
      boxShadow: '1px 1px 1px var(--shadow-inset)',
      start: '100%',
    },
  }),
  stylex.create({
    arrow: {
      position: 'absolute',
    },
    container: {
      position: 'relative',
    },
  }),
  stylex.create({
    above: {
      marginBottom: 15,
    },
    below: {
      marginTop: 15,
    },
    end: {
      marginInlineStart: 15,
    },
    start: {
      marginInlineEnd: 15,
    },
  }),
  stylex.create({
    above: {
      top: 'calc(100% - 1px)',
    },
    below: {
      bottom: 'calc(100% - 1px)',
    },
    end: {
      end: 'calc(100% - 1px)',
    },
    start: {
      start: 'calc(100% - 1px)',
    },
  }),
  stylex.create({
    end: {
      end: 0,
    },
    middle: {
      start: 'calc(50% - 12.5px)',
    },
    start: {
      start: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      bottom: 0,
    },
    middle: {
      top: 'calc(50% - 12.5px)',
    },
    start: {
      top: 0,
    },
    stretch: {},
  }),
  stylex.create({
    arrow: {
      position: 'absolute',
    },
    container: {
      position: 'relative',
    },
  }),
  stylex.create({
    above: {
      marginBottom: 15,
    },
    below: {
      marginTop: 15,
    },
    end: {
      marginInlineStart: 15,
    },
    start: {
      marginInlineEnd: 15,
    },
  }),
  stylex.create({
    above: {
      top: 'calc(100% - 1px)',
    },
    below: {
      bottom: 'calc(100% - 1px)',
    },
    end: {
      end: 'calc(100% - 6px - 1px)',
    },
    start: {
      start: 'calc(100% - 6px - 1px)',
    },
  }),
  stylex.create({
    end: {
      end: 0,
    },
    middle: {
      start: 'calc(50% - 12.5px)',
    },
    start: {
      start: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      bottom: '12.5px',
    },
    middle: {
      top: 'calc(50% - 12.5px)',
    },
    start: {
      top: 0,
    },
    stretch: {},
  }),
  stylex.create({
    addOnContainer: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    overlay: {
      fill: 'var(--media-pressed)',
    },
    svg: {
      height: '100%',
      width: '100%',
    },
  }),
  stylex.create({
    'bottom-left': {
      bottom: '14.6%',
      left: '14.65%',
      position: 'absolute',
      transform: 'translate(-50%, 50%)',
    },
    'bottom-right': {
      bottom: '14.6%',
      position: 'absolute',
      right: '14.65%',
      transform: 'translate(50%, 50%)',
    },
    left: {},
    right: {},
    'top-left': {
      left: '14.65%',
      position: 'absolute',
      top: '14.65%',
      transform: 'translate(-50%, -50%)',
    },
    'top-right': {
      position: 'absolute',
      right: '14.65%',
      top: '14.7%',
      transform: 'translate(50%, -50%)',
    },
  }),
  stylex.create({
    'bottom-left': {
      bottom: 0,
      left: 0,
      position: 'absolute',
      transform: 'translate(-50%, 50%)',
    },
    'bottom-right': {
      bottom: 0,
      position: 'absolute',
      right: 0,
      transform: 'translate(50%, 50%)',
    },
    left: {},
    right: {},
    'top-left': {
      left: 0,
      position: 'absolute',
      top: 0,
      transform: 'translate(-50%, -50%)',
    },
    'top-right': {
      position: 'absolute',
      right: 0,
      top: 0,
      transform: 'translate(50%, -50%)',
    },
  }),
  stylex.create({
    rootDiv: {
      position: 'relative',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 6.3,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 28.3,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 14.1,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 28.3,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 6.3,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 8.8,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 39.6,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 19.8,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 39.6,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 8.8,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 11.3,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 50.9,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 25.4,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 50.9,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 11.3,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 13.8,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 62.2,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 31.1,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 62.2,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 13.8,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 18.8,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 84.8,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 42.4,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 84.8,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 18.8,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 28.9,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 130,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 65,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 130,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 28.9,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 36.4,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 164,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 82,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 164,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 36.4,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      strokeDashoffset: 43.98,
      transform: 'rotate(-90deg)',
    },
    '25%': {
      strokeDashoffset: 197.9,
      transform: 'rotate(162deg)',
    },
    '50%': {
      strokeDashoffset: 98.9,
      transform: 'rotate(72deg)',
    },
    '75%': {
      strokeDashoffset: 197.9,
      transform: 'rotate(162deg)',
    },
    '100%': {
      strokeDashoffset: 43.98,
      transform: 'rotate(-90deg)',
    },
  }),
  stylex.keyframes({
    '0%': {
      transform: 'rotate(-90deg)',
    },
    '25%': {
      transform: 'rotate(90deg)',
    },
    '50%': {
      transform: 'rotate(270deg)',
    },
    '75%': {
      transform: 'rotate(450deg)',
    },
    '100%': {
      transform: 'rotate(990deg)',
    },
  }),
  stylex.create({
    animationFillModeAndTimingFn: {
      animationFillMode: 'both',
      animationTimingFunction: 'cubic-bezier(0, 0, 1, 1)',
    },
    foregroundCircle: {
      animationDuration: '2s',
      animationFillMode: 'both',
      animationIterationCount: 'infinite',
      animationTimingFunction: 'cubic-bezier(0.33, 0, 0.67, 1)',
      transformOrigin: '50% 50%',
    },
    foregroundCircle12: {
      animationName: 'x1pa964l-B',
    },
    foregroundCircle16: {
      animationName: 'x1679snb-B',
    },
    foregroundCircle20: {
      animationName: 'x1xjxcla-B',
    },
    foregroundCircle24: {
      animationName: 'x1r4dvml-B',
    },
    foregroundCircle32: {
      animationName: 'x1qbrl8z-B',
    },
    foregroundCircle48: {
      animationName: 'xmtu0d7-B',
    },
    foregroundCircle60: {
      animationName: 'x1fx0mws-B',
    },
    foregroundCircle72: {
      animationName: 'xhkd20b-B',
    },
    rotationCircle: {
      animationDuration: '2s',
      animationIterationCount: 'infinite',
      animationName: 'x9xws7e-B',
      animationTimingFunction: 'steps(10, end)',
      outline: 'none',
      transformOrigin: '50% 50%',
    },
    stroke: {
      stroke: 'var(--primary-text)',
    },
  }),
  stylex.create({
    leftContainer: {
      bottom: 0,
      end: '50%',
      overflow: 'hidden',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    rightContainer: {
      bottom: 0,
      end: 0,
      overflow: 'hidden',
      position: 'absolute',
      start: '50%',
      top: 0,
    },
    root: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    svgFromLeft: {
      bottom: 0,
      boxSizing: 'border-box',
      end: '-100%',
      position: 'absolute',
      start: 0,
      top: 0,
      transformOrigin: 'center center',
    },
    svgFromRight: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: '-100%',
      top: 0,
      transformOrigin: 'center center',
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 1,
      transform: 'scale(1, 1)',
    },
    '50%': {
      transform:
        'scale(var(--BasePulseEffect_containerScaleXFactorAt50), var(--BasePulseEffect_containerScaleYFactorAt50))',
    },
    '100%': {
      opacity: 0,
      transform:
        'scale(var(--BasePulseEffect_containerScaleXFactor), var(--BasePulseEffect_containerScaleYFactor))',
    },
  }),
  stylex.keyframes({
    '0%': {
      transform: 'scale(1, 1)',
    },
    '50%': {
      transform:
        'scale(calc(1 / var(--BasePulseEffect_containerScaleXFactorAt50)), calc(1 / var(--BasePulseEffect_containerScaleYFactorAt50)))',
    },
    '100%': {
      transform:
        'scale(calc(1 / var(--BasePulseEffect_containerScaleXFactor)), calc(1 / var(--BasePulseEffect_containerScaleYFactor)))',
    },
  }),
  stylex.create({
    animation: {
      animationDuration: '3s',
      animationIterationCount: 'infinite',
      animationTimingFunction: 'linear',
    },
    animationDelay: {
      animationDelay: '1.5s',
    },
    childrenContainer: {
      position: 'relative',
    },
    pulse: {
      alignItems: 'center',
      display: 'inline-flex',
      height: 'var(--BasePulseEffect_height)',
      justifyContent: 'center',
      pointerEvents: 'none',
      width: 'var(--BasePulseEffect_width)',
    },
    pulseAnimater: {
      animationName: 'xyo5iab-B',
    },
    pulseBorder: {
      borderWidth: 12,
      borderStyle: 'solid',
      borderColor: 'var(--BasePulseEffect_pulseColor)',
    },
    pulseBoxShadow: {
      boxShadow: '0 0 0 12px var(--BasePulseEffect_pulseColor)',
    },
    pulseContainer: {
      alignItems: 'center',
      animationName: 'x1iu4u0u-B',
      display: 'inline-flex',
      end: 0,
      height: 'var(--BasePulseEffect_height)',
      justifyContent: 'center',
      opacity: 0,
      overflow: 'hidden',
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
      width: 'var(--BasePulseEffect_width)',
    },
    root: {
      display: 'inline-block',
      position: 'relative',
    },
  }),
  stylex.create({
    card: {
      boxSizing: 'border-box',
      flexBasis: 0,
      flexGrow: 1,
      overflow: 'hidden',
      position: 'relative',
    },
    container: {
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap',
    },
    filler: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    fixedHeightContainer: {
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    innerRoot: {
      position: 'relative',
    },
    outerRoot: {
      overflow: 'hidden',
    },
    placeholder: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
    },
    sizer: {
      display: 'inline-block',
    },
    sizerRow: {
      fontSize: 0,
      lineHeight: 0,
      whiteSpace: 'nowrap',
    },
  }),
  stylex.create({
    expanding: {
      flexBasis: 0,
      flexGrow: 1,
      flexShrink: 1,
      minWidth: 0,
    },
    row: {
      display: 'flex',
      flexShrink: 0,
    },
  }),
  stylex.create({
    center: {
      justifyContent: 'center',
    },
    end: {
      justifyContent: 'flex-end',
    },
    justify: {
      justifyContent: 'space-between',
    },
    start: {
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    bottom: {
      alignItems: 'flex-end',
    },
    center: {
      alignItems: 'center',
    },
    stretch: {
      alignItems: 'stretch',
    },
    top: {
      alignItems: 'flex-start',
    },
  }),
  stylex.create({
    backward: {
      flexDirection: 'row-reverse',
    },
    forward: {
      flexDirection: 'row',
    },
  }),
  stylex.create({
    backward: {
      flexWrap: 'wrap-reverse',
    },
    forward: {
      flexWrap: 'wrap',
    },
    none: {
      flexWrap: 'nowrap',
    },
  }),
  stylex.create({
    expanding: {
      flexBasis: 0,
      flexGrow: 1,
      flexShrink: 1,
    },
    expandingWithWrap: {
      flexBasis: '100%',
    },
    item: {
      display: 'flex',
      flexDirection: 'column',
      flexShrink: 0,
      maxWidth: '100%',
      minWidth: 0,
    },
    item_DEPRECATED: {
      maxWidth: '100%',
      minWidth: 0,
    },
  }),
  stylex.create({
    1: {
      flexBasis: '100%',
    },
    2: {
      flexBasis: '50%',
    },
    3: {
      flexBasis: 'calc(100% / 3)',
    },
    4: {
      flexBasis: '25%',
    },
    5: {
      flexBasis: '20%',
    },
    6: {
      flexBasis: 'calc(100% / 6)',
    },
    7: {
      flexBasis: 'calc(100% / 7)',
    },
    8: {
      flexBasis: '12.5%',
    },
    9: {
      flexBasis: 'calc(100% / 9)',
    },
    10: {
      flexBasis: '10%',
    },
  }),
  stylex.create({
    bottom: {
      alignSelf: 'flex-end',
    },
    center: {
      alignSelf: 'center',
    },
    stretch: {
      alignSelf: 'stretch',
    },
    top: {
      alignSelf: 'flex-start',
    },
  }),
  stylex.create({
    end: {
      bottom: 0,
    },
    start: {
      top: 0,
    },
    target: {
      end: 0,
      opacity: 0,
      position: 'absolute',
      start: 0,
      zIndex: -1,
    },
  }),
  stylex.create({
    baseScroller: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      position: 'relative',
    },
    baseScrollerHorizontal: {
      flexDirection: 'row',
    },
    baseScrollerWithBottomShadow: {
      marginBottom: -66,
    },
    baseScrollerWithTopShadow: {
      marginTop: -50,
    },
    default: {
      MsOverflowStyle: '-ms-autohiding-scrollbar',
      MsScrollChaining: 'none',
      MsScrollRails: 'railed',
      WebkitOverflowScrolling: 'touch',
      display: 'flex',
      flexDirection: 'column',
      overflowX: 'hidden',
      overflowY: 'hidden',
      position: 'relative',
      zIndex: 0,
    },
    expanding: {
      flexBasis: '100%',
      flexGrow: 1,
      flexShrink: 1,
      minHeight: 0,
    },
    expandingIE11: {
      flexBasis: 'auto',
      flexGrow: 1,
      flexShrink: 1,
      minHeight: 0,
    },
    hideScrollbar: {
      MsOverflowStyle: 'none',
      scrollbarWidth: 'none',
      '::-webkit-scrollbar': {
        display: 'none',
        height: 0,
        width: 0,
      },
    },
    horizontalAuto: {
      overflowX: 'auto',
      overscrollBehaviorX: 'contain',
    },
    perspective: {
      perspective: 1,
      perspectiveOrigin: 'right top',
      position: 'relative',
      transformStyle: 'preserve-3d',
    },
    perspectiveRTL: {
      perspectiveOrigin: 'left top',
    },
    verticalAuto: {
      overflowY: 'auto',
      overscrollBehaviorY: 'contain',
    },
  }),
  stylex.create({
    base: {
      boxSizing: 'border-box',
      display: 'none',
      end: 0,
      opacity: 0,
      padding: '0 4px',
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
      transformOrigin: 'right top',
      transitionDuration: '0.3s',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'ease',
      width: 16,
    },
    hovered: {
      opacity: 1,
      transitionDuration: '0',
    },
    inner: {
      backgroundColor: 'var(--scroll-thumb)',
      borderRadius: 4,
      height: '100%',
      width: '100%',
    },
    rtl: {
      transformOrigin: 'left top',
    },
  }),
  stylex.create({
    base: {
      backgroundColor: 'var(--divider)',
      display: 'none',
      end: 0,
      height: '100%',
      opacity: 0,
      position: 'absolute',
      top: 0,
      transitionDuration: '0.5s',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'ease',
      width: 16,
      ':hover': {
        opacity: 0.3,
      },
    },
  }),
  stylex.create({
    cover: {
      display: 'flex',
      end: 0,
      flexDirection: 'column',
      flexShrink: 0,
      height: 50,
      pointerEvents: 'none',
      position: 'sticky',
      start: 0,
      zIndex: 1,
    },
    coverBottom: {
      bottom: -34,
      clipPath: 'inset(0px 0px 16px 0px)',
      justifyContent: 'flex-end',
      marginBottom: 16,
    },
    coverTop: {
      clipPath: 'inset(16px 0px 0px 0px)',
      justifyContent: 'flex-start',
      top: -34,
    },
    shadow: {
      flexShrink: 0,
      height: 16,
      position: 'sticky',
      '::after': {
        boxShadow: 'var(--scroll-shadow)',
        content: '""',
        height: 16,
        position: 'absolute',
        top: -16,
        width: '100%',
      },
    },
    shadowBottom: {
      bottom: 0,
      transform: 'scaleY(-1)',
    },
    shadowTop: {
      top: 0,
    },
  }),
  stylex.create({
    bottom: {
      bottom: 0,
    },
    main: {
      height: 1,
      opacity: 0,
      pointerEvents: 'none',
      position: 'absolute',
      width: 1,
    },
    top: {
      top: 0,
    },
  }),
  stylex.create({
    disabled: {
      cursor: 'not-allowed',
      opacity: 0.4,
    },
    rail: {
      backgroundColor: 'var(--divider)',
      borderRadius: 2,
      display: 'block',
      height: 4,
      position: 'absolute',
      width: '100%',
    },
    root: {
      boxSizing: 'content-box',
      cursor: 'pointer',
      display: 'inline-block',
      height: 4,
      paddingBottom: 8,
      paddingTop: 8,
      position: 'relative',
      touchAction: 'none',
      width: '100%',
    },
    thumb: {
      backgroundColor: 'var(--always-white)',
      borderColor: 'var(--divider)',
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: 1,
      bottom: 0,
      boxShadow: '0px 2px 9px var(--media-inner-border)',
      boxSizing: 'border-box',
      height: 20,
      marginInlineStart: -10,
      outline: 'none',
      position: 'absolute',
      top: 0,
      width: 20,
    },
    thumbFocusVisible: {
      borderColor: 'var(--accent)',
      boxShadow: '0 0 0 3px hsla(var(--accent-h), var(--accent-s), var(--accent-l), 0.2) inset',
    },
    thumbWrapper: {
      marginInlineEnd: 10,
      marginInlineStart: 10,
      marginTop: -8,
      position: 'relative',
    },
    track: {
      backgroundColor: 'var(--accent)',
      borderRadius: 2,
      display: 'block',
      height: 4,
      position: 'absolute',
    },
  }),
  stylex.create({
    root: {
      overflowAnchor: 'none',
      position: 'sticky',
    },
  }),
  stylex.create({
    button: {
      boxSizing: 'border-box',
      display: 'inline-flex',
      flexDirection: 'column',
      justifyContent: 'center',
      position: 'relative',
      width: '100%',
    },
    content: {
      borderRadius: 'var(--button-corner-radius)',
      borderWidth: 0,
      boxSizing: 'border-box',
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    disabled: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    item: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      marginInlineEnd: 'var(--button-inner-icon-spacing-medium)',
      marginInlineStart: 'var(--button-inner-icon-spacing-medium)',
    },
    offset: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginInlineEnd: 'calc(-1*var(--button-inner-icon-spacing-medium))',
      marginInlineStart: 'calc(-1*var(--button-inner-icon-spacing-medium))',
      width: 'calc(100% + 6px)',
    },
    paddingWide: {
      paddingInlineEnd: 40,
      paddingInlineStart: 40,
    },
    sizeLargeItem: {
      marginInlineEnd: 'var(--button-inner-icon-spacing-large)',
      marginInlineStart: 'var(--button-inner-icon-spacing-large)',
    },
    sizeLargeOffset: {
      marginInlineEnd: 'calc(-1*var(--button-inner-icon-spacing-large))',
      marginInlineStart: 'calc(-1*var(--button-inner-icon-spacing-large))',
    },
  }),
  stylex.create({
    checkbox: {
      display: 'flex',
    },
  }),
  stylex.create({
    deselectedBorder: {
      borderColor: 'var(--primary-icon)',
    },
    disabledBorder: {
      borderColor: 'var(--disabled-button-background)',
    },
    radio: {
      display: 'flex',
    },
    radioBorder: {
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: '2px',
      boxSizing: 'border-box',
      display: 'inline-block',
      flexShrink: 0,
      height: 24,
      position: 'relative',
      width: 24,
    },
    selectedBorder: {
      borderColor: 'var(--accent)',
    },
    sizeLarge: {
      height: 24,
      width: 24,
    },
    sizeMedium: {
      height: 20,
      width: 20,
    },
  }),
  stylex.create({
    alignIcon: {
      alignItems: 'center',
    },
    background: {
      backgroundColor: 'var(--switch-active)',
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      opacity: 0,
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
      transitionDuration: 'var(--fds-duration-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-move-out)',
    },
    backgroundActive: {
      opacity: 1,
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionTimingFunction: 'var(--fds-animation-move-in)',
    },
    disabled: {
      opacity: 0.4,
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionTimingFunction: 'var(--fds-animation-move-in)',
    },
    innerShadow: {
      borderRadius: 14,
      boxShadow: 'inset 0 0 0 0.5px var(--media-inner-border)',
      height: 28,
      width: 52,
    },
    slider: {
      backgroundColor: 'var(--always-white)',
      borderRadius: '12px',
      boxShadow: '0 1px 2px var(--shadow-5)',
      height: 24,
      pointerEvents: 'none',
      position: 'absolute',
      start: 2,
      top: 2,
      transitionDuration: 'var(--fds-duration-extra-short-out)',
      transitionProperty: 'transform',
      transitionTimingFunction: 'var(--fds-animation-move-out)',
      width: 24,
    },
    sliderActive: {
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionTimingFunction: 'var(--fds-animation-move-in)',
    },
    sliderActiveLeft: {
      transform: 'translateX(-24px)',
    },
    sliderActiveLeftSmall: {
      transform: 'translateX(-20px)',
    },
    sliderActiveRight: {
      transform: 'translateX(24px)',
    },
    sliderActiveRightSmall: {
      transform: 'translateX(20px)',
    },
    sliderIconContainer: {
      height: '100%',
      width: '100%',
    },
    sliderSmall: {
      height: 20,
      width: 20,
    },
    switch: {
      backgroundColor: 'var(--divider)',
      borderRadius: 14,
      boxSizing: 'border-box',
      display: 'inline-block',
      height: 28,
      opacity: 1,
      overflow: 'hidden',
      padding: 0,
      position: 'relative',
      transitionDuration: 'var(--fds-duration-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-move-out)',
      width: 52,
    },
    switchSmall: {
      borderRadius: 12,
      height: 24,
      width: 44,
    },
  }),
  stylex.create({
    disabled: {
      backgroundColor: 'var(--background-deemphasized)',
      cursor: 'not-allowed',
      ':hover': {
        borderColor: 'var(--secondary-button-stroke)',
      },
    },
    focusedInputRowBorder: {
      borderColor: 'var(--border-focused)',
    },
    groupedHelperText: {
      flexBasis: '100%',
      marginTop: 12,
    },
    groupedInput: {
      borderBottomWidth: 1,
      borderInlineEndWidth: 0,
      borderRadius: 0,
      borderInlineStartWidth: 0,
      borderTopWidth: 0,
      flexWrap: 'wrap',
      minHeight: 63,
    },
    groupedLastInput: {
      borderBottomWidth: 0,
      minHeight: 62,
    },
    helperText: {
      marginTop: 'var(--text-input-caption-margin-top)',
    },
    helperTextError: {
      color: 'var(--negative)',
    },
    inputError: {
      borderColor: 'var(--negative)',
      ':hover': {
        borderColor: 'var(--negative)',
      },
    },
    inputRow: {
      alignItems: 'center',
      backgroundColor: 'var(--input-background)',
      borderColor: 'var(--secondary-button-stroke)',
      borderRadius: 'var(--input-corner-radius)',
      borderStyle: 'solid',
      borderWidth: 1,
      boxSizing: 'border-box',
      display: 'flex',
      flexWrap: 'nowrap',
      justifyContent: 'space-between',
      minHeight: 'var(--text-input-min-height)',
      overflow: 'hidden',
      paddingInline: 16,
      paddingBlock: 'var(--text-input-padding-vertical)',
      position: 'relative',
      width: '100%',
      zIndex: 0,
      ':hover': {
        borderColor: 'var(--border-focused)',
      },
    },
    label: {
      fontSize: 'var(--text-input-label-font-size)',
      fontWeight: 'var(--text-input-field-font-weight)',
      lineHeight: 'var(--text-input-field-line-height)',
      transformOrigin: 'top left',
    },
    labelError: {
      color: 'var(--negative)',
    },
    labelInside: {
      color: 'var(--placeholder-text)',
      cursor: 'inherit',
      end: 16,
      maxWidth: '100%',
      overflow: 'hidden',
      pointerEvents: 'none',
      position: 'absolute',
      start: 16,
      textOverflow: 'ellipsis',
      top: 'var(--text-input-label-top)',
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionProperty: 'transform',
      transitionTimingFunction: 'var(--fds-animation-move-in)',
      whiteSpace: 'nowrap',
    },
    labelOutside: {
      color: 'var(--text-input-outside-label)',
      marginBottom: 8,
      position: 'relative',
    },
    labelRTL: {
      transformOrigin: 'top right',
    },
    labelShrunk: {
      color: 'var(--secondary-text)',
      end: 'auto',
      fontFamily: 'var(--text-input-label-font-family), var(--font-family-default)',
      fontWeight: 'var(--text-input-label-font-weight)',
      lineHeight: 'var(--text-input-label-line-height)',
      transform: 'scale(var(--text-input-label-font-size-scale-multiplier)) translateY(-13px)',
      transitionTimingFunction: 'var(--fds-animation-move-out)',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      width: '100%',
    },
    validationIcon: {
      alignItems: 'center',
      display: 'flex',
      flexGrow: 0,
      flexShrink: 0,
      marginInlineStart: 16,
    },
  }),
  stylex.create({
    focusedTextArea: {
      marginTop: 18,
      paddingTop: 0,
    },
    textArea: {
      backgroundColor: 'transparent',
      borderStyle: 'none',
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      flexBasis: 'calc(100% - 40px)',
      flexGrow: 1,
      fontSize: 'var(--text-input-field-font-size)',
      fontWeight: 'var(--text-input-field-font-weight)',
      height: 'auto',
      lineHeight: 'var(--text-input-field-line-height)',
      marginInline: -2,
      marginTop: 18,
      minHeight: 44,
      minWidth: 0,
      paddingInlineEnd: 'var(--text-input-multi-padding-between-text-scrollbar)',
      paddingTop: 0,
      resize: 'none',
      textOverflow: 'ellipsis',
    },
    textAreaWithNoLabel: {
      marginTop: 0,
      minHeight: 'auto',
      paddingTop: 3,
    },
  }),
  stylex.create({
    focusedInput: {
      marginTop: 0,
      paddingInline: 2,
      paddingTop: 18,
    },
    input: {
      backgroundColor: 'transparent',
      borderStyle: 'none',
      boxSizing: 'border-box',
      color: 'var(--primary-text)',
      flexBasis: 'calc(100% - 40px)',
      flexGrow: 1,
      fontSize: 'var(--text-input-field-font-size) !important',
      fontWeight: 'var(--text-input-field-font-weight)',
      height: 38,
      lineHeight: 'var(--text-input-field-line-height)',
      marginInline: -2,
      minWidth: 0,
      textOverflow: 'ellipsis',
      '::-webkit-search-cancel-button': {
        display: 'none',
      },
      '::-webkit-search-results-button': {
        display: 'none',
      },
      ':autofill': {
        marginTop: 0,
        paddingInline: 2,
        paddingTop: 18,
      },
      ':autofill + label': {
        color: 'var(--secondary-text)',
        end: 'auto',
        fontFamily: 'var(--text-input-label-font-family), var(--font-family-default)',
        fontWeight: 'var(--text-input-label-font-weight)',
        lineHeight: 'var(--text-input-label-line-height)',
        transform: 'scale(var(--text-input-label-font-size-scale-multiplier)) translateY(-13px)',
        transitionTimingFunction: 'var(--fds-animation-move-out)',
      },
    },
  }),
  stylex.create({
    base: {
      maxWidth: '100%',
      minWidth: 0,
      whiteSpace: 'pre-line',
      wordBreak: 'break-word',
      wordWrap: 'break-word',
    },
    block: {
      '::after': {
        content: '""',
        display: 'block',
        height: 0,
      },
      '::before': {
        content: '""',
        display: 'block',
        height: 0,
      },
    },
    inline: {
      display: 'inline',
    },
  }),
  stylex.create({
    1: {
      '::before': {
        marginTop: -1,
      },
    },
    2: {
      '::before': {
        marginTop: -2,
      },
    },
    3: {
      '::before': {
        marginTop: -3,
      },
    },
    4: {
      '::before': {
        marginTop: -4,
      },
    },
    5: {
      '::before': {
        marginTop: -5,
      },
    },
    6: {
      '::before': {
        marginTop: -6,
      },
    },
    7: {
      '::before': {
        marginTop: -7,
      },
    },
    8: {
      '::before': {
        marginTop: -8,
      },
    },
    9: {
      '::before': {
        marginTop: -9,
      },
    },
    10: {
      '::before': {
        marginTop: -10,
      },
    },
    11: {
      '::before': {
        marginTop: -11,
      },
    },
    12: {
      '::before': {
        marginTop: -12,
      },
    },
  }),
  stylex.create({
    1: {
      '::after': {
        marginBottom: -1,
      },
    },
    2: {
      '::after': {
        marginBottom: -2,
      },
    },
    3: {
      '::after': {
        marginBottom: -3,
      },
    },
    4: {
      '::after': {
        marginBottom: -4,
      },
    },
    5: {
      '::after': {
        marginBottom: -5,
      },
    },
    6: {
      '::after': {
        marginBottom: -6,
      },
    },
    7: {
      '::after': {
        marginBottom: -7,
      },
    },
    8: {
      '::after': {
        marginBottom: -8,
      },
    },
    9: {
      '::after': {
        marginBottom: -9,
      },
    },
    10: {
      '::after': {
        marginBottom: -10,
      },
    },
    11: {
      '::after': {
        marginBottom: -11,
      },
    },
    12: {
      '::after': {
        marginBottom: -12,
      },
    },
  }),
  stylex.create({
    1: {
      paddingBottom: 1,
    },
    2: {
      paddingBottom: 2,
    },
    3: {
      paddingBottom: 3,
    },
  }),
  stylex.create({
    auto: {},
    center: {
      textAlign: 'center',
    },
    end: {
      textAlign: 'end',
    },
    start: {
      textAlign: 'start',
    },
  }),
  stylex.create({
    example: {
      fontSize: 17,
      fontWeight: 'normal',
    },
  }),
  stylex.create({
    example: {
      color: 'var(--primary-text)',
    },
  }),
  stylex.create({
    default: {
      fontFamily: 'sans-serif',
    },
  }),
  stylex.create({
    item: {
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 'var(--toast-addon-padding-vertical)',
      paddingInlineEnd: 'var(--toast-addon-padding-horizontal)',
      paddingInlineStart: 'var(--toast-addon-padding-horizontal)',
      paddingTop: 'var(--toast-addon-padding-vertical)',
    },
    itemText: {
      flexGrow: 1,
    },
    link: {
      wordBreak: 'keep-all',
    },
    root: {
      alignItems: 'center',
      backgroundColor: 'var(--toast-background)',
      borderRadius: 'var(--toast-corner-radius)',
      boxShadow: 'var(--shadow-elevated)',
      display: 'flex',
      flexShrink: 0,
      maxWidth: 'var(--toast-container-max-width)',
      minWidth: 'var(--toast-container-min-width)',
      paddingInline: 'var(--toast-container-padding-horizontal)',
      paddingBlock: 'var(--toast-container-padding-vertical)',
    },
    rootFullWidth: {
      width: '100%',
    },
  }),
  stylex.create({
    mount: {
      opacity: 1,
      transform: 'scale(1)',
      transitionDuration: 'var(--fds-duration-short-in)',
      transitionTimingFunction: 'var(--fds-animation-enter-exit-in)',
    },
    root: {
      opacity: 0,
      transform: 'scale(0.8) translateY(300px)',
      transitionDuration: 'var(--fds-duration-short-out)',
      transitionProperty: 'transform, opacity',
      transitionTimingFunction: 'var(--fds-animation-enter-exit-out)',
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--tooltip-background)',
      borderRadius: 'var(--tooltip-border-radius)',
      boxShadow: 'var(--tooltip-box-shadow)',
      display: 'block',
      marginBottom: 2,
      marginTop: 2,
      maxWidth: '334px',
      opacity: 0,
      padding: '12px 12px',
      position: 'relative',
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    containerVisible: {
      opacity: 1,
      transitionDuration: 'var(--fds-duration-extra-extra-short-in)',
      transitionTimingFunction: 'var(--fds-animation-fade-in)',
    },
    contextualLayer: {
      pointerEvents: 'none',
    },
    loadingState: {
      display: 'flex',
      justifyContent: 'center',
    },
  }),
  stylex.create({
    inheritAll: {
      alignContent: 'inherit',
      alignItems: 'inherit',
      alignSelf: 'inherit',
      display: 'inherit',
      flexBasis: 'inherit',
      flexDirection: 'inherit',
      flexGrow: 'inherit',
      flexShrink: 'inherit',
      height: 'inherit',
      justifyContent: 'inherit',
      maxHeight: 'inherit',
      maxWidth: 'inherit',
      minHeight: 'inherit',
      minWidth: 'inherit',
      width: 'inherit',
    },
    wrapperInline: {
      display: 'inline-flex',
    },
  }),
  stylex.create({
    root: {
      display: 'flex',
      height: '100%',
      maxHeight: 'inherit',
      position: 'relative',
      width: '100%',
    },
    video: {
      height: '100%',
      maxHeight: 'inherit',
      width: '100%',
    },
  }),
  stylex.create({
    disabled: {
      cursor: 'not-allowed',
    },
    focusNotVisible: {
      outline: 'none',
    },
    notSelectable: {
      userSelect: 'none',
    },
    root: {
      WebkitTapHighlightColor: 'transparent',
      backgroundColor: 'transparent',
      borderWidth: 0,
      boxSizing: 'border-box',
      cursor: 'pointer',
      display: 'inline',
      listStyle: 'none',
      margin: 0,
      padding: 0,
      textAlign: 'inherit',
      textDecoration: 'none',
      touchAction: 'manipulation',
    },
    rootInGroup: {
      touchAction: 'none',
    },
  }),
  stylex.create({
    cssMask: {
      backgroundColor: 'currentColor',
    },
  }),
  stylex.create({
    hidden: {
      display: 'none',
    },
    root: {
      boxSizing: 'border-box',
      position: 'relative',
      zIndex: 0,
    },
  }),
  stylex.create({
    root: {
      position: 'relative',
      width: '100%',
      top: 0,
    },
    sticky: {
      position: 'sticky',
    },
    sentinel: {
      position: 'absolute',
      start: 0,
      width: '100%',
      height: 1,
      pointerEvents: 'none',
    },
    sentinelTop: {
      top: -1,
    },
    sentinelBottom: {
      bottom: 0,
    },
  }),
  stylex.create({
    disabled: {
      cursor: 'not-allowed',
    },
    focusNotVisible: {
      outlineStyle: 'none',
    },
    root: {
      WebkitTapHighlightColor: 'transparent',
      alignItems: 'stretch',
      backgroundColor: 'transparent',
      borderColor: 'var(--always-dark-overlay)',
      borderStyle: 'solid',
      borderWidth: 0,
      boxSizing: 'border-box',
      cursor: 'pointer',
      display: 'flex',
      flexBasis: 'auto',
      flexDirection: 'column',
      flexShrink: 0,
      listStyle: 'none',
      margin: '0',
      minHeight: '0',
      minWidth: '0',
      padding: '0',
      position: 'relative',
      textAlign: 'inherit',
      textDecoration: 'none',
      touchAction: 'manipulation',
      zIndex: 0,
    },
    rootInGroup: {
      touchAction: 'none',
    },
  }),
  stylex.create({
    root: {
      clip: 'rect(0, 0, 0, 0)',
      clipPath: 'inset(50%)',
      height: 1,
      overflow: 'hidden',
      position: 'absolute',
      width: 1,
    },
  }),
  stylex.create({
    auxiliary: {
      bottom: 0,
      end: 0,
      position: 'absolute',
    },
    circle: {
      borderRadius: '50%',
    },
    image: {
      display: 'block',
    },
    inset: {
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    root: {
      position: 'relative',
    },
    roundedRect: {
      borderRadius: 8,
    },
  }),
  stylex.create({
    expanded: {
      display: 'block',
      width: '100%',
    },
    root: {
      backgroundColor: 'transparent',
      borderWidth: 0,
      color: 'inherit',
      cursor: 'pointer',
      display: 'inline-block',
      fontFamily: 'inherit',
      fontSize: 'inherit',
      lineHeight: 'inherit',
      margin: 0,
      padding: 0,
      textAlign: 'inherit',
      textDecoration: 'inherit',
      ':active': {
        transform: 'scale(0.98)',
        transition: 'none',
      },
      ':hover': {
        color: 'inherit',
        textDecoration: 'inherit',
      },
    },
  }),
  stylex.create({
    root: {
      display: 'inline-flex',
      fontStyle: 'normal',
      fontWeight: 'normal',
      margin: '0 1px',
      verticalAlign: 'middle',
    },
    size128: {
      height: 128,
      width: 128,
    },
    size16: {
      height: 16,
      width: 16,
    },
    size18: {
      height: 18,
      width: 18,
    },
    size20: {
      height: 20,
      width: 20,
    },
    size24: {
      height: 24,
      width: 24,
    },
    size28: {
      height: 28,
      width: 28,
    },
    size30: {
      height: 30,
      width: 30,
    },
    size32: {
      height: 32,
      width: 32,
    },
    size56: {
      height: 56,
      width: 56,
    },
  }),
  stylex.create({
    copyableAltText: {
      color: 'transparent',
      display: 'inline',
      opacity: 0.5,
      '::selection': {
        backgroundColor: 'Highlight',
      },
    },
    root: {
      backgroundRepeat: 'no-repeat',
      backgroundSize: 'contain',
      display: 'inline-block',
      fontStyle: 'normal',
      fontWeight: 'normal',
      margin: '0 1px',
      overflow: 'hidden',
      verticalAlign: 'middle',
    },
    size128: {
      height: 128,
      width: 128,
    },
    size16: {
      height: 16,
      width: 16,
    },
    size18: {
      height: 18,
      width: 18,
    },
    size20: {
      height: 20,
      width: 20,
    },
    size24: {
      height: 24,
      width: 24,
    },
    size28: {
      height: 28,
      width: 28,
    },
    size30: {
      height: 30,
      width: 30,
    },
    size32: {
      height: 32,
      width: 32,
    },
    size56: {
      height: 56,
      width: 56,
    },
  }),
  stylex.create({
    root: {
      marginInlineEnd: 8,
      marginInlineStart: 8,
    },
  }),
  stylex.create({
    coverPhoto: {
      backgroundSize: 'cover',
    },
  }),
  stylex.create({
    badge: {
      bottom: 0,
      display: 'flex',
      flexDirection: 'column',
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
    },
    expanding: {
      display: 'block',
    },
    root: {
      display: 'inline-block',
    },
  }),
  stylex.create({
    center: {
      alignItems: 'center',
      end: -8,
      justifyContent: 'flex-end',
      start: -8,
    },
    left: {
      alignItems: 'flex-start',
      end: -8,
      justifyContent: 'flex-end',
      start: 0,
    },
    right: {
      alignItems: 'flex-end',
      end: 0,
      justifyContent: 'flex-end',
      start: -8,
    },
    topRight: {
      alignItems: 'flex-end',
      end: 0,
      justifyContent: 'flex-start',
      start: -8,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
    },
    content: {
      backgroundColor: 'var(--overlay-alpha-80)',
      borderRadius: 8,
      borderWidth: 1,
      boxShadow: '0 8px 16px var(--shadow-1)',
      paddingInline: 12,
      paddingBlock: 16,
    },
    crossoutButton: {
      marginInlineEnd: -4,
      marginTop: -8,
    },
    item: {
      paddingInline: 6,
      paddingBlock: 6,
    },
  }),
  stylex.create({
    accent: {
      backgroundColor: 'var(--accent)',
    },
    default: {
      backgroundColor: 'var(--popover-background)',
    },
  }),
  stylex.create({
    above: {
      marginBottom: 4,
    },
    below: {
      marginTop: 4,
    },
    end: {
      marginInlineStart: 4,
    },
    start: {},
  }),
  stylex.create({
    arrow: {
      position: 'absolute',
    },
    container: {
      display: 'flex',
    },
    content: {
      backgroundColor: 'var(--overlay-alpha-80)',
      borderRadius: 8,
      borderWidth: 1,
      boxShadow: '0 8px 16px var(--shadow-1)',
      paddingInline: 12,
      paddingBlock: 16,
    },
  }),
  stylex.create({
    accent: {
      backgroundColor: 'var(--accent)',
    },
    default: {
      backgroundColor: 'var(--popover-background)',
    },
  }),
  stylex.create({
    end: {
      borderBottomEndRadius: 0,
      marginBottom: 20,
    },
    middle: {
      marginBottom: 4,
    },
    start: {
      borderBottomStartRadius: 0,
      marginBottom: 20,
    },
  }),
  stylex.create({
    end: {
      borderTopEndRadius: 0,
      marginTop: 20,
    },
    middle: {
      marginTop: 4,
    },
    start: {
      borderTopStartRadius: 0,
      marginTop: 20,
    },
  }),
  stylex.create({
    end: {
      bottom: 9,
      end: 0,
      transform: 'scaleX(-1)',
    },
    middle: {
      bottom: 9,
      end: 0,
    },
    start: {
      bottom: 9,
      start: 0,
    },
  }),
  stylex.create({
    end: {
      bottom: 9,
      end: 0,
      transform: 'scaleX(-1)',
    },
    middle: {
      bottom: 9,
      end: 0,
    },
    start: {
      bottom: 9,
      start: 0,
    },
  }),
  stylex.create({
    end: {
      end: 0,
      top: 9,
      transform: 'rotate(180deg)',
    },
    middle: {
      end: 0,
      top: 9,
      transform: 'rotate(180deg) scaleX(-1)',
    },
    start: {
      start: 0,
      top: 9,
      transform: 'rotate(180deg) scaleX(-1)',
    },
  }),
  stylex.create({
    end: {
      transform: 'rotate(180deg)',
    },
    middle: {
      transform: 'rotate(180deg) scaleX(-1)',
    },
    start: {
      transform: 'rotate(180deg) scaleX(-1)',
    },
  }),
  stylex.create({
    end: {
      end: 0,
      top: 9,
      transform: 'rotate(180deg)',
    },
    middle: {
      end: 0,
      top: 9,
      transform: 'rotate(180deg) scaleX(-1)',
    },
    start: {
      start: 0,
      top: 9,
      transform: 'rotate(180deg) scaleX(-1)',
    },
  }),
  stylex.create({
    end: {
      transform: 'rotate(180deg)',
    },
    middle: {
      transform: 'rotate(180deg) scaleX(-1)',
    },
    start: {
      transform: 'rotate(180deg) scaleX(-1)',
    },
  }),
  stylex.create({
    arrow: {
      position: 'absolute',
    },
    container: {
      display: 'flex',
    },
    content: {
      backgroundColor: 'var(--overlay-alpha-80)',
      borderRadius: 8,
      borderWidth: 1,
      boxShadow: '0 8px 16px var(--shadow-1)',
      paddingInline: 12,
      paddingBlock: 16,
    },
  }),
  stylex.create({
    accent: {
      backgroundColor: 'var(--accent)',
    },
    default: {
      backgroundColor: 'var(--popover-background)',
    },
  }),
  stylex.create({
    above: {
      marginBottom: 16,
    },
    below: {
      marginTop: 16,
    },
    end: {
      marginInlineStart: 16,
    },
  }),
  stylex.create({
    above: {
      bottom: 5,
    },
    below: {
      top: 5,
      transform: 'rotate(180deg) scaleX(-1)',
    },
    end: {},
  }),
  stylex.create({
    above: {
      bottom: 5,
    },
    below: {
      top: 5,
      transform: 'rotate(180deg) scaleX(-1)',
    },
    end: {},
  }),
  stylex.create({
    pressableOverlayPressed: {
      backgroundColor: 'var(--non-media-pressed)',
    },
    pressed: {
      transform: 'scale(0.96)',
    },
    root: {
      alignItems: 'center',
      borderRadius: '50%',
      borderWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      justifyContent: 'center',
      padding: 0,
      position: 'relative',
    },
  }),
  stylex.create({
    24: {
      height: 24,
      width: 24,
    },
    28: {
      height: 28,
      width: 28,
    },
    32: {
      height: 32,
      width: 32,
    },
    36: {
      height: 36,
      width: 36,
    },
    40: {
      height: 40,
      width: 40,
    },
    48: {
      height: 48,
      width: 48,
    },
  }),
  stylex.create({
    'dark-overlay': {
      backgroundColor: 'var(--always-dark-overlay)',
      color: 'var(--always-white)',
    },
    deemphasized: {
      backgroundColor: 'transparent',
    },
    'deemphasized-overlay': {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
    },
    normal: {
      backgroundColor: 'var(--secondary-button-background)',
    },
    overlay: {
      backgroundColor: 'var(--popover-background)',
      boxShadow: '0 0 0 1px var(--shadow-1)',
      color: 'var(--secondary-text)',
    },
    'overlay-floating': {
      backgroundColor: 'var(--secondary-button-background-floating)',
      boxShadow: '0 2px 4px var(--shadow-1), 0 12px 28px var(--shadow-2)',
    },
    'overlay-raised': {
      backgroundColor: 'var(--popover-background)',
      boxShadow: '0 2px 8px var(--shadow-1), 0 0 0 1px var(--shadow-1)',
      color: 'var(--secondary-text)',
    },
    'primary-background-overlay': {
      backgroundColor: 'var(--primary-button-background)',
    },
  }),
  stylex.create({
    'dark-overlay': {
      backgroundColor: 'var(--always-dark-overlay)',
    },
    deemphasized: {
      backgroundColor: 'transparent',
    },
    'deemphasized-overlay': {
      backgroundColor: 'var(--always-light-overlay)',
    },
    normal: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    overlay: {
      backgroundColor: 'var(--progress-ring-on-media-background)',
      borderWidth: 0,
      boxShadow: '0 2px 4px var(--shadow-1)',
      color: 'var(--disabled-text)',
    },
    'primary-background-overlay': {
      backgroundColor: 'var(--primary-button-background)',
    },
  }),
  stylex.create({
    center: {
      textAlign: 'center',
    },
    end: {
      textAlign: 'end',
    },
    start: {
      textAlign: 'start',
    },
  }),
  stylex.create({
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabled: {
      color: 'var(--disabled-text)',
    },
    highlight: {
      color: 'var(--accent)',
    },
    negative: {
      color: 'var(--negative)',
    },
    placeholder: {
      color: 'var(--placeholder-text)',
    },
    positive: {
      color: 'var(--positive)',
    },
    primary: {
      color: 'var(--primary-text)',
    },
    primaryOnMedia: {
      color: 'var(--primary-text-on-media)',
    },
    secondary: {
      color: 'var(--secondary-text)',
    },
    secondaryOnMedia: {
      color: 'var(--secondary-text-on-media)',
    },
    tertiary: {
      color: 'var(--placeholder-text)',
    },
    white: {
      color: 'var(--always-white)',
    },
  }),
  stylex.create({
    11: {
      fontSize: 11,
      lineHeight: 1.1818181818181819,
    },
    12: {
      fontSize: 12,
      lineHeight: 1.3333333333333333,
    },
    13: {
      fontSize: 13,
      lineHeight: 1.2307692307692308,
    },
    15: {
      fontSize: 15,
      lineHeight: 1.3333333333333333,
    },
    17: {
      fontSize: 17,
      lineHeight: 1.1764705882352942,
    },
    20: {
      fontSize: 20,
      lineHeight: 1.2,
    },
    24: {
      fontSize: 24,
      lineHeight: 1.1666666666666667,
    },
    28: {
      fontSize: 28,
      lineHeight: 1.1428571428571428,
    },
    32: {
      fontSize: 32,
      lineHeight: 1.1875,
    },
  }),
  stylex.create({
    bold: {
      fontWeight: 700,
    },
    medium: {
      fontWeight: 500,
    },
    normal: {
      fontWeight: 400,
    },
    semibold: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    root: {
      boxSizing: 'border-box',
      fontFamily: 'var(--font-family-code)',
    },
  }),
  stylex.create({
    expanding: {
      flexBasis: '100%',
      flexGrow: 1,
      flexShrink: 1,
      minHeight: 0,
    },
    expandingIE11: {
      flexBasis: 'auto',
    },
    inner: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      minHeight: 0,
    },
    root: {
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexShrink: 0,
      maxWidth: '100%',
    },
  }),
  stylex.create({
    0: {
      paddingTop: 0,
    },
    4: {
      paddingTop: 4,
    },
    8: {
      paddingTop: 8,
    },
    12: {
      paddingTop: 12,
    },
    16: {
      paddingTop: 16,
    },
    20: {
      paddingTop: 20,
    },
  }),
  stylex.create({
    4: {
      paddingBlock: 4,
    },
    8: {
      paddingBlock: 8,
    },
    12: {
      paddingBlock: 12,
    },
    16: {
      paddingBlock: 16,
    },
    20: {
      paddingBlock: 20,
    },
  }),
  stylex.create({
    bottom: {
      justifyContent: 'flex-end',
    },
    center: {
      justifyContent: 'center',
    },
    'space-between': {
      justifyContent: 'space-between',
    },
  }),
  stylex.create({
    divider: {
      borderTopColor: 'var(--divider)',
      borderTopStyle: 'solid',
      borderTopWidth: 1,
      ':first-child': {
        display: 'none',
      },
    },
    dividerMargin: {
      ':first-child:empty + *': {
        marginTop: 0,
      },
    },
    expanding: {
      flexBasis: '100%',
      flexGrow: 1,
      flexShrink: 1,
      minHeight: 0,
    },
    expandingIE11: {
      flexBasis: 'auto',
    },
    marginFirstChild: {
      ':first-child': {
        marginTop: 0,
      },
    },
    marginLastChild: {
      ':last-child': {
        marginBottom: 0,
      },
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      flexShrink: 0,
      maxWidth: '100%',
    },
  }),
  stylex.create({
    center: {
      alignItems: 'center',
    },
    end: {
      alignItems: 'flex-end',
    },
    start: {
      alignItems: 'flex-start',
    },
  }),
  stylex.create({
    4: {
      paddingInline: 4,
    },
    8: {
      paddingInline: 8,
    },
    12: {
      paddingInline: 12,
    },
    16: {
      paddingInline: 16,
    },
    20: {
      paddingInline: 20,
    },
  }),
  stylex.create({
    0: {
      paddingTop: 0,
    },
    4: {
      paddingTop: 4,
    },
    8: {
      paddingTop: 8,
    },
    12: {
      paddingTop: 12,
    },
    16: {
      paddingTop: 16,
    },
    20: {
      paddingTop: 20,
    },
    40: {
      paddingTop: 40,
    },
  }),
  stylex.create({
    4: {
      paddingBlock: 4,
    },
    8: {
      paddingBlock: 8,
    },
    12: {
      paddingBlock: 12,
    },
    16: {
      paddingBlock: 16,
    },
    20: {
      paddingBlock: 20,
    },
    40: {
      paddingBlock: 40,
    },
  }),
  stylex.create({
    4: {
      marginBlock: 2,
    },
    8: {
      marginBlock: 4,
    },
    12: {
      marginBlock: 6,
    },
    16: {
      marginBlock: 8,
    },
    20: {
      marginBlock: 10,
    },
    24: {
      marginBlock: 12,
    },
    32: {
      marginBlock: 16,
    },
    40: {
      marginBlock: 20,
    },
  }),
  stylex.create({
    bottom: {
      justifyContent: 'flex-end',
    },
    center: {
      justifyContent: 'center',
    },
    'space-between': {
      justifyContent: 'space-between',
    },
  }),
  stylex.create({
    4: {
      marginInline: 4,
    },
    8: {
      marginInline: 8,
    },
    12: {
      marginInline: 12,
    },
    16: {
      marginInline: 16,
    },
    20: {
      marginInline: 20,
    },
  }),
  stylex.create({
    root: {
      marginBottom: -8,
      marginTop: -8,
    },
  }),
  stylex.create({
    addOn: {
      marginInlineStart: 16,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    content: {
      display: 'flex',
      flexDirection: 'column',
      paddingBottom: 8,
      paddingTop: 8,
    },
    overlayPressed: {
      backgroundColor: 'var(--non-media-pressed)',
    },
    root: {
      appearance: 'none',
      backgroundColor: 'transparent',
      borderWidth: 0,
      boxSizing: 'border-box',
      display: 'inline-block',
      margin: 0,
      padding: 0,
      position: 'relative',
      textAlign: 'inherit',
      width: '100%',
    },
  }),
  stylex.create({
    closeButton: {
      marginInlineEnd: -4,
      marginTop: -8,
    },
    icon: {
      marginTop: -4,
    },
    root: {
      borderRadius: 8,
      overflow: 'hidden',
      padding: 4,
      paddingBottom: 16,
    },
  }),
  stylex.create({
    highlight: {
      backgroundColor: 'var(--accent)',
    },
    'highlight-bg': {
      backgroundColor: 'var(--highlight-bg)',
    },
    primary: {
      backgroundColor: 'var(--card-background)',
    },
    secondary: {
      backgroundColor: 'var(--card-background-flat)',
    },
  }),
  stylex.create({
    form: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
    },
    leftColumnActor: {
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
    },
  }),
  stylex.create({
    extraHeaderContent: {
      marginTop: 16,
    },
    extraHeaderContentLeftAligned: {
      marginInlineStart: 16,
    },
    headerContainer: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
    },
    innerCardContainer: {
      borderColor: 'var(--media-inner-border)',
      borderRadius: 8,
      borderStyle: 'solid',
      borderWidth: 1,
      display: 'flex',
      flexBasis: 0,
      flexGrow: 1,
      margin: 16,
      overflowY: 'hidden',
    },
    innerCardContent: {
      height: '100%',
      overflowX: 'hidden',
      overflowY: 'auto',
    },
    innerCardContentBody: {
      height: '100%',
    },
    mobile: {
      width: 564,
    },
    noClick: {
      pointerEvents: 'none',
    },
    outerCard: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 8,
      boxShadow: '0 2px 12px var(--shadow-2)',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      height: '100%',
      margin: '32px 24px',
      maxWidth: 'calc(100% - 48px)',
      minHeight: 0,
      width: 972,
    },
    pushView: {
      marginTop: 'var(--header-height)',
    },
    responsiveToggleContainer: {
      marginInlineStart: 'auto',
    },
    title: {
      flexBasis: 0,
      flexGrow: 1,
      flexShrink: 1,
    },
  }),
  stylex.create({
    layoutControls: {
      display: 'flex',
      justifyContent: 'center',
      marginInlineEnd: 16,
      marginInlineStart: 16,
      marginTop: 16,
    },
    layoutOption: {
      marginInlineEnd: 16,
    },
  }),
  stylex.create({
    root: {
      marginTop: 5,
    },
  }),
  stylex.create({
    root: {
      boxSizing: 'border-box',
      padding: 1,
      position: 'relative',
    },
  }),
  stylex.create({
    draggable: {
      WebkitUserDrag: 'element',
      boxSizing: 'border-box',
      cursor: 'move',
      userDrag: 'element',
      userSelect: 'none',
      willChange: 'auto',
    },
    draggableFullWidth: {
      width: '100%',
    },
    draggablePlaceholder: {
      borderColor: 'var(--base-blue)',
      borderRadius: 8,
      borderStyle: 'dashed',
      borderWidth: 1,
      margin: -1,
    },
    isDraggingStyle: {
      willChange: 'transform',
    },
    item: {
      boxSizing: 'border-box',
    },
    itemPlaceholder: {
      visibility: 'hidden',
    },
    undraggedItem: {
      pointerEvents: 'none',
    },
  }),
  stylex.create({
    root: {
      display: 'none',
    },
  }),
  stylex.create({
    container: {
      height: 40,
      position: 'relative',
      width: 82,
    },
    loading: {
      position: 'absolute',
      start: 21,
      top: 18,
    },
    profile: {
      backgroundColor: 'var(--placeholder-icon)',
      borderRadius: '50%',
      height: 36,
      marginInlineEnd: 6,
      width: 36,
    },
    selector: {
      alignItems: 'center',
      display: 'flex',
      paddingBottom: 2,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      paddingTop: 2,
    },
  }),
  stylex.create({
    background: {
      backgroundColor: 'var(--nav-bar-background)',
    },
    bgFill: {
      backgroundColor: 'var(--card-background)',
    },
    recessed: {
      borderTopEndRadius: '10px',
      borderTopStartRadius: '10px',
      marginInlineEnd: 'auto',
      marginInlineStart: 'auto',
      marginTop: '-70px',
      maxWidth: '1218px',
      paddingBottom: '16px',
      '@media (max-width: 1249px)': {
        width: 'calc(100% - 20px)',
      },
    },
    rootWithDropShadow: {
      boxShadow: '0 1px 2px var(--shadow-1)',
    },
    topRow: {
      paddingBottom: 16,
    },
  }),
  stylex.create({
    actorSelector: {
      borderWidth: 1,
      borderStyle: 'solid',
      borderStartColor: 'var(--divider)',
      marginInlineStart: 16,
      paddingInlineStart: 4,
    },
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--nav-bar-background)',
      boxShadow: '0 1px 2px var(--shadow-1)',
    },
  }),
  stylex.create({
    description: {
      display: 'flex',
      flexGrow: 1,
    },
    root: {
      display: 'flex',
      paddingBottom: 12,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 12,
    },
  }),
  stylex.create({
    container: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    coverDraggable: {
      bottom: 0,
      boxSizing: 'border-box',
      cursor: 'move',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      touchAction: 'pan-x pan-y',
    },
    dragInstructions: {
      alignItems: 'center',
      backgroundColor: 'var(--always-dark-overlay)',
      borderRadius: 8,
      display: 'flex',
      paddingBottom: 8,
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
      paddingTop: 8,
      position: 'absolute',
      start: '50%',
      top: '50%',
    },
    dragInstructionsIcon: {
      display: 'flex',
      marginInlineEnd: 8,
    },
    dragInstructionsLTR: {
      transform: 'translate(-50%, -50%)',
    },
    dragInstructionsRTL: {
      transform: 'translate(50%, -50%)',
    },
  }),
  stylex.create({
    accessory: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
    },
    backgroundCover: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      overflow: 'hidden',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    backgroundMask: {
      backgroundImage: 'var(--nav-bar-background-gradient)',
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    backgroundMaskWash: {
      backgroundImage: 'var(--nav-bar-background-gradient-wash)',
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    coverActionBarContainer: {
      backgroundColor: 'var(--always-dark-overlay)',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      zIndex: 1,
    },
    coverActionBarContainerPushView: {
      top: 60,
    },
    coverButton: {
      marginBottom: 16,
    },
    coverButtonRow: {
      paddingInlineEnd: 20,
      paddingInlineStart: 20,
      position: 'relative',
    },
    coverButtonShade: {
      backgroundImage: 'var(--always-dark-gradient)',
    },
    coverButtonStacked: {
      marginBottom: 8,
    },
    coverButtonStackedOnly: {
      marginBottom: 60,
    },
    coverFooterExternal: {
      overflow: 'hidden',
      width: '100%',
    },
    coverHeaderRow: {
      display: 'flex',
      justifyContent: 'center',
      paddingTop: 60,
      position: 'relative',
    },
    coverMedia: {
      overflow: 'hidden',
      position: 'relative',
      width: '100%',
    },
    coverMediaLarge: {
      maxWidth: 665,
    },
    coverMediaPlaceholder: {
      backgroundColor: 'var(--web-wash)',
    },
    coverMediaRoundCornersBottom: {
      borderBottomEndRadius: 8,
      borderBottomStartRadius: 8,
      '@media (max-width: 939px)': {
        borderBottomEndRadius: 0,
        borderBottomStartRadius: 0,
      },
    },
    coverMediaRoundCornersTop: {
      borderTopEndRadius: 8,
      borderTopStartRadius: 8,
      '@media (max-width: 939px)': {
        borderTopEndRadius: 0,
        borderTopStartRadius: 0,
      },
    },
    coverPhoto: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
    },
    coverTopLeftButtonRow: {
      position: 'absolute',
      start: 0,
      top: 0,
    },
    image: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      height: '100%',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
    mediaOverlayContainer: {
      backgroundColor: 'var(--always-dark-overlay)',
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    mediaOverlayContainerNoShade: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    navBarPushViewBackground: {
      backgroundColor: 'var(--surface-background)',
      height: 60,
    },
    pressable: {
      display: 'block',
    },
  }),
  stylex.create({
    footer: {
      paddingBottom: 16,
      paddingTop: 16,
      position: 'relative',
    },
  }),
  stylex.create({
    background: {
      backgroundColor: 'var(--nav-bar-background)',
    },
    coverPhoto: {
      display: 'flex',
      flexDirection: 'column',
    },
  }),
  stylex.create({
    onlyMobile: {
      '@media (max-width: 900px)': {
        borderWidth: 1,
        borderStyle: 'solid',
        borderBottomColor: 'var(--divider)',
      },
    },
    root: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
    },
  }),
  stylex.create({
    facepile: {
      display: 'inline-block',
    },
    left: {
      marginInlineStart: -6,
    },
  }),
  stylex.create({
    image: {
      borderColor: 'var(--media-outer-border)',
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: 2,
      display: 'block',
    },
    root: {
      display: 'inline-block',
    },
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--nav-bar-background)',
      boxShadow: '0 1px 2px var(--shadow-1)',
      paddingBottom: 24,
      paddingTop: 24,
    },
  }),
  stylex.create({
    container: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    container: {
      transitionDuration: 'var(--fds-duration-extra-extra-short-in)',
      transitionProperty: 'transform',
      transitionTimingFunction: 'var(--fds-animation-quick-move-in)',
    },
    containerSticky: {
      transform: 'translateY(-100%)',
    },
    fadeInTransition: {
      transitionDuration: 'var(--fds-duration-extra-short-in), 0s',
      transitionProperty: 'opacity, visibility',
      transitionTimingFunction: 'var(--fds-animation-fade-in), linear',
    },
    fadeOutTransition: {
      transitionDuration: 'var(--fds-duration-extra-short-in), 1s',
      transitionProperty: 'opacity, visibility',
      transitionTimingFunction: 'var(--fds-animation-fade-in), linear',
    },
    primaryRow: {
      minHeight: 60,
      opacity: 1,
      visibility: 'visible',
    },
    primaryRowSticky: {
      opacity: 0,
      visibility: 'hidden',
    },
    root: {
      overflow: 'hidden',
      position: 'relative',
    },
    stickyRow: {
      height: '100%',
      opacity: 0,
      position: 'absolute',
      top: '100%',
      visibility: 'hidden',
      width: '100%',
    },
    stickyRowSticky: {
      opacity: 1,
      visibility: 'visible',
    },
  }),
  stylex.create({
    base: {
      alignItems: 'center',
      appearance: 'none',
      backgroundColor: 'transparent',
      backgroundImage: 'none',
      borderStyle: 'solid',
      borderWidth: 0,
      boxSizing: 'border-box',
      color: 'inherit',
      cursor: 'pointer',
      display: 'inline-flex',
      height: 60,
      margin: 0,
      padding: 0,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      position: 'relative',
      textAlign: 'inherit',
      textDecoration: 'none',
      zIndex: 0,
    },
    content: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
    },
    disabled: {
      cursor: 'not-allowed',
      pointerEvents: 'none',
    },
    icon: {
      display: 'inline-block',
      flexShrink: 0,
      marginInlineStart: 4,
      marginTop: 2,
    },
    selected: {
      backgroundColor: 'var(--accent)',
      borderTopEndRadius: 1,
      borderTopStartRadius: 1,
      bottom: 0,
      end: 0,
      height: 3,
      position: 'absolute',
      start: 0,
    },
  }),
  stylex.create({
    entityHeaderTab: {
      height: 60,
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    actorWrapper: {
      alignItems: 'flex-end',
      display: 'flex',
      height: 0,
      marginBottom: -12,
      marginInlineEnd: 16,
    },
    actorWrapperFullHeight: {
      height: '100%',
    },
    root: {
      alignItems: 'flex-end',
      display: 'flex',
    },
    title: {
      marginTop: 16,
    },
  }),
  stylex.create({
    auxiliaryColumn: {
      justifyContent: 'flex-end',
    },
    column: {
      flexBasis: 0,
      flexGrow: 9999,
      minWidth: 320,
    },
    columnBottomSpacing: {
      paddingBottom: 16,
    },
    columnNoExpanding: {
      flexBasis: 'auto',
      flexGrow: 1,
    },
    root: {
      width: '100%',
    },
    rootWithExtraSpace: {
      marginTop: 16,
    },
  }),
  stylex.create({
    headerHiddenByDefault: {
      display: 'flex',
      flexDirection: 'column',
      height: 0,
      justifyContent: 'flex-end',
    },
    headerHiddenByDefaultNotSticky: {
      pointerEvents: 'none',
      visibility: 'hidden',
      zIndex: -1,
    },
    headerHiddenByDefaultSticky: {
      position: 'fixed',
      width: '100%',
    },
    row: {
      backgroundColor: 'var(--nav-bar-background)',
      boxShadow: '0 1px 2px var(--shadow-1)',
      flexShrink: 0,
    },
  }),
  stylex.create({
    calendar: {
      alignItems: 'flex-end',
      alignSelf: 'flex-start',
      display: 'flex',
      height: 0,
      marginTop: 16,
    },
    title: {
      alignSelf: 'flex-start',
      width: '100%',
    },
  }),
  stylex.create({
    rootWithActor: {
      paddingTop: 32,
    },
    rootWithEntityHeaderTabs: {
      paddingTop: 32,
    },
    rootWithoutActor: {
      paddingTop: 46,
      '@media (max-width: 899px)': {
        paddingTop: 16,
      },
    },
    rootWithoutEntityHeaderTabsAndWithActor: {
      paddingBottom: 20,
    },
    rootWithoutEntityHeaderTabsAndWithoutActor: {
      paddingBottom: 36,
      '@media (max-width: 899px)': {
        paddingBottom: 0,
      },
    },
  }),
  stylex.create({
    actorWrapper: {
      alignItems: 'flex-end',
      display: 'flex',
      marginInlineEnd: 16,
      marginTop: -30,
    },
    column: {
      flexBasis: 320,
      paddingBottom: 16,
    },
    root: {
      width: '100%',
    },
    title: {
      marginTop: 16,
    },
  }),
  stylex.create({
    actorContainer: {
      alignItems: 'flex-end',
      alignSelf: 'center',
      display: 'flex',
      height: 0,
      marginTop: 16,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    rootAlignStart: {
      alignSelf: 'flex-start',
    },
  }),
  stylex.create({
    root: {
      marginInlineEnd: 8,
      marginTop: 5,
    },
  }),
  stylex.create({
    item: {
      position: 'relative',
    },
    items: {
      display: 'flex',
      flexDirection: 'row',
      marginBlock: -10,
      overflowX: 'hidden',
      paddingBlock: 10,
      position: 'relative',
    },
    itemWithActivity: {
      marginInlineStart: 6,
    },
    itemWithSpacing: {
      marginInlineStart: 4,
    },
    overflow24: {
      height: 24,
      width: 24,
    },
    overflow32: {
      height: 32,
      width: 32,
    },
    overflow40: {
      height: 40,
      width: 40,
    },
    overflow48: {
      height: 48,
      width: 48,
    },
    overflowItem: {
      alignItems: 'center',
      borderRadius: '50%',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'center',
      pointerEvents: 'all',
    },
    overflowItemBg: {
      fill: 'var(--always-dark-overlay)',
    },
    overflowItemContainer: {
      bottom: 10,
      display: 'flex',
      end: 0,
      flexDirection: 'row',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 10,
    },
    overflowItemOverlay: {
      fill: 'var(--always-dark-overlay)',
      opacity: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    overflowItemOverlayHovered: {
      fill: 'var(--hover-overlay)',
      opacity: 1,
      transitionDuration: '0ms',
    },
    overflowItemOverlayPressed: {
      fill: 'var(--media-pressed)',
      opacity: 1,
      transitionDuration: '0ms',
    },
    overflowItemSVG: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    overlappingItem: {
      marginInlineStart: -4,
    },
    pressableItem: {
      borderRadius: '50%',
      display: 'block',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    rootInline: {
      alignItems: 'center',
      flexDirection: 'row',
    },
    text: {
      paddingTop: 12,
    },
    textInline: {
      paddingInlineStart: 4,
      paddingTop: 0,
    },
    wrapper: {
      paddingInlineEnd: 12,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    item: {
      marginBottom: 20,
      position: 'relative',
    },
    items: {
      display: 'flex',
      flexDirection: 'row',
      flexShrink: 0,
      flexWrap: 'wrap',
      overflow: 'hidden',
      position: 'relative',
    },
    itemWithoutMargin: {
      position: 'relative',
    },
    itemWithSpacing: {
      marginInlineStart: -4,
    },
    overflow16: {
      height: 16,
      width: 16,
    },
    overflow16Child: {
      alignItems: 'center',
      display: 'flex',
      justifyContent: 'center',
      marginInlineStart: -6,
    },
    overflow16Frame: {
      marginInlineStart: 6,
      overflow: 'hidden',
      width: 10,
    },
    overflow24: {
      height: 24,
      width: 24,
    },
    overflow32: {
      height: 32,
      width: 32,
    },
    overflow36: {
      height: 36,
      width: 36,
    },
    overflow40: {
      height: 40,
      width: 40,
    },
    overflow48: {
      height: 48,
      width: 48,
    },
    overflowItem: {
      alignItems: 'center',
      borderRadius: '50%',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'center',
      pointerEvents: 'all',
    },
    overflowItemBg: {
      fill: 'var(--always-dark-overlay)',
    },
    overflowItemContainer: {
      bottom: 0,
      boxSizing: 'border-box',
      display: 'flex',
      end: 0,
      flexDirection: 'row',
      pointerEvents: 'none',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    overflowItemOverlay: {
      fill: 'var(--always-dark-overlay)',
      opacity: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    overflowItemOverlayHovered: {
      fill: 'var(--hover-overlay)',
      opacity: 1,
      transitionDuration: '0ms',
    },
    overflowItemOverlayPressed: {
      fill: 'var(--media-pressed)',
      opacity: 1,
      transitionDuration: '0ms',
    },
    overflowItemSVG: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
    },
    rootInline: {
      alignItems: 'center',
      flexDirection: 'row',
    },
    text: {
      display: 'flex',
      flexDirection: 'row',
      paddingTop: 12,
    },
    textInline: {
      flexBasis: 0,
      flexGrow: 1,
      paddingInlineStart: 4,
      paddingTop: 0,
    },
    wrapper: {
      paddingInlineEnd: 12,
      paddingInlineStart: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    16: {
      minWidth: 40,
    },
    24: {
      minWidth: 64,
    },
    32: {
      minWidth: 88,
    },
    36: {
      minWidth: 100,
    },
    40: {
      minWidth: 112,
    },
    48: {
      minWidth: 136,
    },
  }),
  stylex.create({
    center: {
      alignItems: 'center',
    },
    end: {
      alignItems: 'flex-end',
    },
    start: {
      alignItems: 'flex-start',
    },
  }),
  stylex.create({
    center: {
      justifyContent: 'center',
    },
    end: {
      justifyContent: 'flex-end',
    },
    start: {
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    item: {
      position: 'absolute',
    },
    moreItem: {
      alignItems: 'center',
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      justifyContent: 'center',
    },
    moreItemFontWeight: {
      fontWeight: 600,
    },
    moreItemInsetBorder: {
      borderRadius: 'inherit',
      bottom: 0,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    smallBadgeFontSize: {
      fontSize: 10,
    },
  }),
  stylex.create({
    32: {
      height: 32,
      width: 32,
    },
    36: {
      height: 36,
      width: 36,
    },
    40: {
      height: 40,
      width: 40,
    },
    44: {
      height: 44,
      width: 44,
    },
    48: {
      height: 48,
      width: 48,
    },
    56: {
      height: 56,
      width: 56,
    },
    60: {
      height: 60,
      width: 60,
    },
    72: {
      height: 72,
      width: 72,
    },
    80: {
      height: 80,
      width: 80,
    },
    120: {
      height: 120,
      width: 120,
    },
  }),
  stylex.create({
    1: {
      height: '100%',
      width: '100%',
    },
    2: {
      height: 'calc(100% * (2 / 3))',
      width: 'calc(100% * (2 / 3))',
    },
    3: {
      height: 'calc(100% * (17 / 36))',
      width: 'calc(100% * (17 / 36))',
    },
    4: {
      height: 'calc(100% * (17 / 36))',
      width: 'calc(100% * (17 / 36))',
    },
  }),
  stylex.create({
    0: {
      borderColor: 'var(--card-background)',
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: 2,
      bottom: 0,
      boxSizing: 'content-box',
      margin: -2,
      start: 0,
      zIndex: 1,
    },
    1: {
      end: 0,
      top: 0,
    },
  }),
  stylex.create({
    0: {
      borderColor: 'var(--card-background)',
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: 2,
      bottom: 0,
      boxSizing: 'content-box',
      end: 0,
      margin: -2,
      zIndex: 1,
    },
    1: {
      start: 0,
      top: 0,
    },
  }),
  stylex.create({
    0: {
      left: '50%',
      top: 1,
      transform: 'translateX(-50%)',
    },
    1: {
      bottom: 1,
      start: 1,
    },
    2: {
      bottom: 1,
      end: 1,
    },
  }),
  stylex.create({
    0: {
      start: 1,
      top: 1,
    },
    1: {
      bottom: 1,
      start: 1,
    },
    2: {
      end: 1,
      top: 1,
    },
    3: {
      bottom: 1,
      end: 1,
    },
  }),
  stylex.create({
    circle: {},
    rounded: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 10,
    },
    square: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 0,
    },
  }),
  stylex.create({
    grid: {
      alignContent: 'flex-start',
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap',
      justifyContent: 'flex-start',
      margin: -4,
    },
    gridJustifyCenter: {
      justifyContent: 'center',
    },
    item: {
      boxSizing: 'border-box',
      display: 'flex',
      flexBasis: 0,
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
      visibility: 'hidden',
    },
    itemVisible: {
      paddingBottom: 4,
      paddingTop: 4,
      visibility: 'visible',
    },
  }),
  stylex.create({
    item: {
      flex: 1,
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
    },
    justifyCenter: {
      justifyContent: 'center',
    },
    justifyEnd: {
      justifyContent: 'flex-end',
    },
    root: {
      alignItems: 'stretch',
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'nowrap',
      justifyContent: 'flex-start',
      marginInlineEnd: -4,
      marginInlineStart: -4,
      position: 'relative',
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 0.25,
    },
    '100%': {
      opacity: 1,
    },
  }),
  stylex.create({
    dark: {
      backgroundColor: 'var(--placeholder-icon)',
    },
    paused: {
      animationPlayState: 'paused',
    },
    root: {
      animationDirection: 'alternate',
      animationDuration: '1000ms',
      animationIterationCount: 'infinite',
      animationName: 'xmekl8e-B',
      animationTimingFunction: 'steps(10, end)',
      backgroundColor: 'var(--wash)',
      opacity: 0.25,
    },
  }),
  stylex.create({
    firstItem: {
      paddingTop: 0,
    },
    imageSize20: {
      height: 20,
      width: 20,
    },
    imageSize36: {
      height: 36,
      width: 36,
    },
    imageSize40: {
      height: 40,
      width: 40,
    },
    imageSize48: {
      height: 48,
      width: 48,
    },
    imageSize56: {
      height: 56,
      width: 56,
    },
    imageSize60: {
      height: 60,
      width: 60,
    },
    imageStyleCircle: {
      borderRadius: '50%',
    },
    imageStyleRoundedRect: {
      borderRadius: 8,
    },
    item: {
      paddingBlock: 8,
    },
    textGlimmer: {
      borderRadius: 8,
      height: 15,
    },
    textGlimmerWidth100: {
      width: '100%',
    },
    textGlimmerWidth50: {
      width: '50%',
    },
    textGlimmerWidth67: {
      width: '67%',
    },
    textGlimmerWidth83: {
      width: '83%',
    },
  }),
  stylex.create({
    firstItem: {
      paddingTop: 0,
    },
    imageSize20: {
      height: 20,
      width: 20,
    },
    imageSize36: {
      height: 36,
      width: 36,
    },
    imageSize40: {
      height: 40,
      width: 40,
    },
    imageSize48: {
      height: 48,
      width: 48,
    },
    imageSize56: {
      height: 56,
      width: 56,
    },
    imageSize60: {
      height: 60,
      width: 60,
    },
    imageStyleCircle: {
      borderRadius: '50%',
    },
    imageStyleRoundedRect: {
      borderRadius: 8,
    },
    item: {
      paddingBlock: 8,
    },
    textGlimmer: {
      borderRadius: 8,
      height: 15,
    },
    textGlimmerWidth100: {
      width: '100%',
    },
    textGlimmerWidth25: {
      width: '25%',
    },
    textGlimmerWidth30: {
      width: '30%',
    },
    textGlimmerWidth35: {
      width: '35%',
    },
    textGlimmerWidth40: {
      width: '40%',
    },
    textGlimmerWidth50: {
      width: '50%',
    },
    textGlimmerWidth67: {
      width: '67%',
    },
    textGlimmerWidth83: {
      width: '83%',
    },
  }),
  stylex.create({
    hovercard: {
      opacity: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    hovercardVisible: {
      opacity: 1,
      transitionDuration: 'var(--fds-duration-extra-extra-short-in)',
      transitionTimingFunction: 'var(--fds-animation-fade-in)',
    },
  }),
  stylex.create({
    displayInline: {
      display: 'inline',
    },
    displayInlineBlock: {
      display: 'inline-block',
    },
  }),
  stylex.create({
    disablePointerEvents: {
      pointerEvents: 'none',
    },
    hovercard: {
      opacity: 0,
      transitionDuration: 'var(--fds-duration-extra-extra-short-out)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-out)',
    },
    hovercardVisible: {
      opacity: 1,
      transitionDuration: 'var(--fds-duration-extra-extra-short-in)',
      transitionTimingFunction: 'var(--fds-animation-fade-in)',
    },
  }),
  stylex.create({
    disabled: {
      opacity: 0.4,
    },
    insetBorder: {
      bottom: 0,
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    root: {
      backgroundColor: 'var(--card-background)',
      display: 'block',
      overflow: 'hidden',
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    rounded: {
      borderRadius: 8,
    },
    square: {},
  }),
  stylex.create({
    16: {
      height: 16,
      width: 16,
    },
    20: {
      height: 20,
      width: 20,
    },
    24: {
      height: 24,
      width: 24,
    },
    28: {
      height: 28,
      width: 28,
    },
    32: {
      height: 32,
      width: 32,
    },
    36: {
      height: 36,
      width: 36,
    },
    40: {
      height: 40,
      width: 40,
    },
    44: {
      height: 44,
      width: 44,
    },
    48: {
      height: 48,
      width: 48,
    },
    52: {
      height: 52,
      width: 52,
    },
    56: {
      height: 56,
      width: 56,
    },
    60: {
      height: 60,
      width: 60,
    },
    64: {
      height: 64,
      width: 64,
    },
    68: {
      height: 68,
      width: 68,
    },
    72: {
      height: 72,
      width: 72,
    },
    76: {
      height: 76,
      width: 76,
    },
    80: {
      height: 80,
      width: 80,
    },
    132: {
      height: 132,
      width: 132,
    },
    144: {
      height: 144,
      width: 144,
    },
    160: {
      height: 160,
      width: 160,
    },
  }),
  stylex.create({
    disabled: {
      color: 'var(--disabled-text)',
      ':hover': {
        textDecoration: 'none',
      },
    },
    hoverUnderlineDisabled: {
      ':hover': {
        textDecoration: 'none',
      },
    },
    root: {
      color: 'inherit',
      ':hover': {
        textDecoration: 'underline',
      },
    },
  }),
  stylex.create({
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabled: {
      color: 'var(--disabled-text)',
    },
    highlight: {
      color: 'var(--accent)',
    },
    negative: {
      color: 'var(--negative)',
    },
    positive: {
      color: 'var(--positive)',
    },
    primary: {
      color: 'var(--primary-text)',
    },
    secondary: {
      color: 'var(--secondary-text)',
    },
    tertiary: {
      color: 'var(--placeholder-text)',
    },
    white: {
      color: 'var(--always-white)',
    },
  }),
  stylex.create({
    bold: {
      fontWeight: 700,
    },
    medium: {
      fontWeight: 500,
    },
    normal: {
      fontWeight: 400,
    },
    semibold: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    disabled: {
      color: 'var(--disabled-text)',
      ':hover': {
        textDecoration: 'none',
      },
    },
    root: {
      color: 'inherit',
      ':hover': {
        textDecoration: 'underline',
      },
    },
  }),
  stylex.create({
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabled: {
      color: 'var(--disabled-text)',
    },
    highlight: {
      color: 'var(--accent)',
    },
    negative: {
      color: 'var(--negative)',
    },
    positive: {
      color: 'var(--positive)',
    },
    primary: {
      color: 'var(--primary-text)',
    },
    secondary: {
      color: 'var(--secondary-text)',
    },
    tertiary: {
      color: 'var(--placeholder-text)',
    },
    white: {
      color: 'var(--always-white)',
    },
  }),
  stylex.create({
    bold: {
      fontWeight: 700,
    },
    medium: {
      fontWeight: 500,
    },
    normal: {
      fontWeight: 400,
    },
    semibold: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    block: {
      display: 'block',
    },
    'inline-block': {
      display: 'inline-block',
    },
  }),
  stylex.create({
    childListCell: {
      listStyleType: 'none',
      margin: 0,
      padding: 0,
    },
  }),
  stylex.create({
    margins: {
      marginBottom: -16,
      marginTop: -4,
    },
  }),
  stylex.create({
    childListCell: {
      listStyleType: 'none',
      margin: 0,
      padding: 0,
    },
  }),
  stylex.create({
    addOnStartMargin: {
      marginTop: -4,
    },
  }),
  stylex.create({
    4: {
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
    },
    8: {
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
    },
    12: {
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    16: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    4: {
      paddingBottom: 4,
      paddingTop: 4,
    },
    8: {
      paddingBottom: 8,
      paddingTop: 8,
    },
    12: {
      paddingBottom: 12,
      paddingTop: 12,
    },
    16: {
      paddingBottom: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    4: {
      marginInlineEnd: -2,
      marginInlineStart: -2,
    },
    8: {
      marginInlineEnd: -4,
      marginInlineStart: -4,
    },
    12: {
      marginInlineEnd: -6,
      marginInlineStart: -6,
    },
    16: {
      marginInlineEnd: -8,
      marginInlineStart: -8,
    },
    24: {
      marginInlineEnd: -12,
      marginInlineStart: -12,
    },
    32: {
      marginInlineEnd: -16,
      marginInlineStart: -16,
    },
  }),
  stylex.create({
    4: {
      marginBottom: -2,
      marginTop: -2,
    },
    8: {
      marginBottom: -4,
      marginTop: -4,
    },
    12: {
      marginBottom: -6,
      marginTop: -6,
    },
    16: {
      marginBottom: -8,
      marginTop: -8,
    },
    24: {
      marginInlineEnd: -12,
      marginInlineStart: -12,
    },
    32: {
      marginInlineEnd: -16,
      marginInlineStart: -16,
    },
  }),
  stylex.create({
    disabled: {
      cursor: 'not-allowed',
      pointerEvents: 'none',
    },
    root: {
      borderRadius: 8,
      display: 'block',
    },
    selected: {
      backgroundColor: 'var(--hosted-view-selected-state)',
    },
    selectedWashBackground: {
      backgroundColor: 'var(--web-wash)',
    },
  }),
  stylex.create({
    addOn: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
    },
    addOnWithExpander: {
      marginInlineEnd: 8,
    },
    addOnWithIcon: {
      display: 'flex',
    },
    addOnWithText: {
      marginInlineStart: 4,
    },
    bottomAddOn: {
      display: 'flex',
      flexDirection: 'column',
      marginInlineEnd: -12,
      marginInlineStart: -12,
    },
    bottomAddOnInner: {
      maxWidth: '100%',
    },
    bottomAddOnOverrideRow: {
      flexDirection: 'row',
      marginInlineEnd: 0,
      marginInlineStart: 0,
      paddingTop: 6,
    },
    bottomAddOnWithFacepile: {
      marginInlineStart: -16,
    },
    bottomDivider: {
      backgroundColor: 'var(--divider)',
      bottom: 0,
      end: 0,
      height: 1,
      position: 'absolute',
      start: 0,
    },
    content: {
      alignItems: 'stretch',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexBasis: 0,
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingBlock: 12,
      position: 'relative',
      zIndex: 0,
    },
    contentContainer: {
      alignItems: 'center',
      alignSelf: 'stretch',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'relative',
      zIndex: 0,
    },
    contentDense: {
      paddingBlock: 8,
    },
    contentWithMoreSpacing: {
      paddingBlock: 16,
    },
    contentWithMoreSpacingDense: {
      paddingBlock: 12,
    },
    disabled: {
      cursor: 'not-allowed',
      pointerEvents: 'none',
    },
    endAddOn: {
      marginBottom: 12,
      marginInlineStart: 12,
      marginTop: 12,
      position: 'relative',
    },
    endAddOnCenter: {
      marginBottom: 8,
      marginTop: 8,
    },
    endAddOnSmall: {
      marginBottom: 8,
      marginInlineStart: 12,
      marginTop: 8,
      position: 'relative',
    },
    listCellMinHeight: {
      minHeight: 44,
    },
    pressable: {
      borderRadius: 8,
      display: 'block',
    },
    responsiveButtons: {
      flexGrow: 1,
      paddingBottom: 6,
      paddingTop: 6,
    },
    responsiveContent: {
      alignItems: 'center',
      flexDirection: 'row',
      flexWrap: 'wrap',
      marginBottom: -6,
      marginTop: -6,
    },
    responsiveText: {
      boxSizing: 'border-box',
      flexBasis: '50%',
      flexGrow: 1,
      flexShrink: 1,
      maxWidth: '100%',
      minWidth: '50%',
      paddingBottom: 6,
      paddingInlineEnd: 16,
      paddingTop: 6,
    },
    root: {
      alignItems: 'center',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      paddingBottom: 0,
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
      paddingTop: 0,
      position: 'relative',
      zIndex: 0,
    },
    rootWithIncreasedHeight: {
      minHeight: 52,
    },
    selected: {
      backgroundColor: 'var(--hosted-view-selected-state)',
    },
    selectedWashBackground: {
      backgroundColor: 'var(--background-deemphasized)',
    },
    startAddOn: {
      alignSelf: 'flex-start',
      display: 'flex',
      flexDirection: 'column',
      marginInlineEnd: 12,
      marginBlock: 8,
      position: 'relative',
    },
    startAddOnDense: {
      marginBlock: 6,
    },
    startAddOnDensityAware: {
      '@media (max-height: 700px)': {
        marginInlineEnd: 6,
        marginInlineStart: -4,
        marginBlock: 4,
        transform: 'scale(0.77777777)',
      },
    },
    textRight: {
      flexShrink: 0,
    },
    visualSwitch: {
      pointerEvents: 'none',
    },
  }),
  stylex.create({
    center: {
      alignSelf: 'center',
    },
    top: {
      alignSelf: 'flex-start',
    },
  }),
  stylex.create({
    center: {
      alignSelf: 'center',
    },
    top: {
      alignSelf: 'flex-start',
    },
  }),
  stylex.create({
    pill: {
      borderRadius: 9999,
    },
    root: {
      width: '100%',
    },
  }),
  stylex.create({
    checkbox: {
      display: 'flex',
    },
  }),
  stylex.create({
    radio: {
      display: 'flex',
    },
  }),
  stylex.create({
    spacer: {
      width: 4,
    },
    textNode: {
      alignItems: 'center',
      display: 'flex',
    },
  }),
  stylex.create({
    centeredMenuSeparator: {
      margin: '8px 16px',
    },
    listItem: {
      borderRadius: 4,
      display: 'flex',
      flexDirection: 'row',
      margin: '0 8px',
      padding: '12px 8px',
    },
    menuSeparator: {
      margin: '8px 0 8px 16px',
    },
    root: {
      alignItems: 'stretch',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      padding: '8px 0',
    },
    sizeFull: {
      width: '100%',
    },
    sizeNormal: {
      width: 344,
    },
    sizeSmall: {
      width: 328,
    },
  }),
  stylex.create({
    aux: {
      marginInlineStart: 12,
    },
    content: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      flexGrow: 1,
      justifyContent: 'space-between',
      minWidth: 0,
    },
    disabled: {
      cursor: 'not-allowed',
    },
    extraHorizontalPadding: {
      paddingInline: 8,
    },
    listItem: {
      alignItems: 'center',
      appearance: 'none',
      boxSizing: 'border-box',
      cursor: 'pointer',
      display: 'flex',
      flexDirection: 'row',
      flexShrink: 0,
      marginInline: 'var(--menu-item-base-margin-horizontal)',
      marginBlock: '0',
      paddingInline: 'var(--menu-item-base-padding-horizontal)',
      paddingBlock: 12,
      position: 'relative',
      textAlign: 'inherit',
      zIndex: 0,
    },
    listItemAlignedCenter: {
      alignItems: 'center',
    },
    listItemWithIcon: {
      paddingBlock: 8,
    },
  }),
  stylex.create({
    headerPadding: {
      padding: '12px 16px',
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    contained: {
      backgroundColor: 'var(--secondary-button-background)',
      borderRadius: '50%',
      height: 'var(--menu-item-icon-container-size, 36px)',
      minWidth: 'var(--menu-item-icon-container-size, 36px)',
    },
    iconRelativeContainer: {
      position: 'relative',
    },
    inset: {
      boxShadow: 'inset 0 0 0 1px var(--media-inner-border)',
      position: 'absolute',
      start: 0,
      top: 0,
    },
    root: {
      alignItems: 'center',
      alignSelf: 'baseline',
      display: 'flex',
      justifyContent: 'center',
      marginInlineEnd: 12,
    },
    roundedRect: {
      borderRadius: 8,
    },
  }),
  stylex.create({
    selectedAux: {
      marginBottom: -5,
      marginTop: -5,
    },
  }),
  stylex.create({
    selectedAux: {
      marginBottom: -5,
      marginTop: -5,
    },
  }),
  stylex.create({
    root: {
      backgroundColor: 'var(--divider)',
      marginBottom: 8,
      marginInlineEnd: 16,
      marginInlineStart: 16,
      marginTop: 8,
    },
  }),
  stylex.create({
    separator: {
      borderTopWidth: 1,
      borderTopStyle: 'solid',
      borderTopColor: 'var(--divider)',
      margin: '4px 16px',
    },
  }),
  stylex.create({
    listItem: {
      borderRadius: 4,
      display: 'flex',
      flexDirection: 'row',
      margin: '0 8px',
      padding: '12px 8px',
    },
    root: {
      alignItems: 'stretch',
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      paddingInline: '0',
      paddingBlock: 'var(--menu-base-padding-vertical, 8px)',
    },
    sizeFull: {
      marginInlineEnd: 48,
      width: '100%',
    },
    sizeNormal: {
      width: 344,
    },
    sizeSmall: {
      width: 328,
    },
  }),
  stylex.create({
    progress: {
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'space-between',
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      position: 'relative',
      zIndex: 0,
      alignItems: 'center',
      padding: 8,
    },
  }),
  stylex.create({
    root: {
      clip: 'rect(0, 0, 0, 0)',
      height: 1,
      overflow: 'hidden',
      position: 'absolute',
      width: 1,
    },
  }),
  stylex.create({
    bottom: {
      bottom: 60,
    },
    container: {
      display: 'flex',
      justifyContent: 'center',
      position: 'absolute',
      start: '50%',
    },
    deemphasized: {
      backgroundColor: 'var(--secondary-button-background-floating)',
    },
    default: {
      alignItems: 'center',
      borderStyle: 'none',
      borderRadius: 30,
      boxShadow: '0 12px 28px 0 var(--shadow-2),0 2px 4px 0 var(--shadow-1)',
      display: 'flex',
      height: 'auto',
      justifyContent: 'center',
      padding: '16px 12px',
      position: 'absolute',
    },
    emphasized: {
      backgroundColor: 'var(--primary-button-background)',
    },
    static: {
      position: 'static',
    },
    top: {
      top: 0,
    },
  }),
  stylex.create({
    anchor: {
      paddingInline: 8,
      paddingBlock: 'var(--dialog-anchor-vertical-padding)',
      '@media (max-width: 564px)': {
        paddingInline: 0,
      },
    },
    anchorInMobileEnvironment: {
      paddingBlock: 0,
    },
    card: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow:
        '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1), inset 0 0 0 1px var(--shadow-inset)',
      '@media (max-width: 564px)': {
        borderRadius: 0,
      },
    },
    rootInMobileEnvironment: {
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    content: {
      maxWidth: '100%',
    },
    'content-mobile-safe': {
      width: '100%',
    },
    medium: {
      maxWidth: 700,
      width: '100%',
    },
    small: {
      maxWidth: 548,
      width: '100%',
    },
  }),
  stylex.create({
    backButton: {
      position: 'absolute',
      start: 16,
      top: 12,
      zIndex: 1,
    },
    closeButton: {
      end: 16,
      position: 'absolute',
      top: 12,
      zIndex: 1,
    },
    header: {
      alignItems: 'center',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
    },
    headerWithoutBottomBorder: {
      borderBottomColor: 'transparent',
    },
    headerWithPadding: {
      paddingInlineEnd: 60,
      paddingInlineStart: 60,
    },
  }),
  stylex.create({
    1: {
      marginInlineEnd: '1ch',
    },
    0.25: {
      marginInlineEnd: '0.25ch',
    },
    0.5: {
      marginInlineEnd: '0.5ch',
    },
    0.75: {
      marginInlineEnd: '0.75ch',
    },
  }),
  stylex.create({
    button: {
      marginTop: 24,
    },
    image: {
      marginBottom: 20,
    },
    root: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      padding: 24,
    },
  }),
  stylex.create({
    absoluteCenter: {
      left: '50%',
      position: 'absolute',
      top: '50%',
      transform: 'translate(-50%, -50%)',
    },
    hideOverflow: {
      overflow: 'hidden',
    },
  }),
  stylex.create({
    deemphasized: {
      backgroundColor: 'transparent',
    },
    normal: {
      backgroundColor: 'var(--secondary-button-background)',
    },
  }),
  stylex.create({
    selectedOnWashBackground: {
      backgroundColor: 'var(--hosted-view-selected-state)',
    },
  }),
  stylex.create({
    defaultWidth: {
      maxWidth: '50%',
    },
    fullWidth: {
      maxWidth: '100%',
    },
  }),
  stylex.create({
    link: {
      maxWidth: '50%',
    },
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      display: 'flex',
      height: 56,
      justifyContent: 'center',
      minWidth: 334,
      width: '100%',
    },
  }),
  stylex.create({
    card: {
      boxSizing: 'border-box',
    },
    cardBackground: {
      backgroundColor: 'var(--card-background)',
    },
    cardBorderRadius: {
      borderRadius: 'var(--card-corner-radius)',
    },
    cardOverflow: {
      overflow: 'hidden',
    },
    cardShadow: {
      boxShadow: 'var(--card-box-shadow)',
    },
    popoverWithArrow: {
      filter: 'drop-shadow(0 0px 6px var(--shadow-2))',
    },
  }),
  stylex.create({
    end: {
      borderBottomEndRadius: 0,
    },
    middle: {},
    start: {
      borderBottomStartRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      borderTopEndRadius: 0,
    },
    middle: {},
    start: {
      borderTopStartRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      borderBottomEndRadius: 0,
    },
    middle: {},
    start: {
      borderTopEndRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    end: {
      borderBottomStartRadius: 0,
    },
    middle: {},
    start: {
      borderTopStartRadius: 0,
    },
    stretch: {},
  }),
  stylex.create({
    root: {
      alignItems: 'center',
      display: 'flex',
      height: 56,
      justifyContent: 'center',
      minWidth: 334,
      width: '100%',
    },
  }),
  stylex.create({
    button: {
      bottom: 0,
      boxSizing: 'border-box',
      cursor: 'pointer',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    container: {
      display: 'flex',
      position: 'relative',
      zIndex: 0,
      ':not([disabled]) .x1ja2u2z': {
        zIndex: 'unset',
      },
      ':not([disabled]) .x1nhjfyr': {
        zIndex: 'unset',
      },
      ':not([disabled]) .xlt5f95': {
        zIndex: 'unset',
      },
    },
  }),
  stylex.create({
    defaultCursor: {
      cursor: 'default',
    },
    expanding: {
      display: 'flex',
    },
    hideOutline: {
      outline: 'none',
    },
    linkBase: {
      display: 'inline-block',
    },
    root: {
      borderRadius: 'inherit',
      display: 'inline-flex',
      flexDirection: 'row',
      userSelect: 'none',
      ':hover': {
        textDecoration: 'none',
      },
    },
    root_DEPRECATED: {
      borderRadius: 'inherit',
      position: 'relative',
      userSelect: 'none',
      ':hover': {
        textDecoration: 'none',
      },
    },
    zIndex: {
      zIndex: 1,
    },
  }),
  stylex.create({
    wrapper: {
      alignContent: 'inherit',
      alignItems: 'inherit',
      borderRadius: 'inherit',
      display: 'inherit',
      flexDirection: 'inherit',
      height: 'inherit',
      justifyContent: 'inherit',
      position: 'relative',
      width: 'inherit',
    },
  }),
  stylex.create({
    activityBadge: {
      alignItems: 'center',
      borderBottomStyle: 'solid',
      borderBottomWidth: 0,
      borderInlineEndStyle: 'solid',
      borderInlineEndWidth: 0,
      borderRadius: '50%',
      borderInlineStartStyle: 'solid',
      borderInlineStartWidth: 0,
      borderTopStyle: 'solid',
      borderTopWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      flexShrink: 1,
      justifyContent: 'center',
      marginBottom: 0,
      marginInlineEnd: 0,
      marginInlineStart: 0,
      marginTop: 0,
      minHeight: 0,
      minWidth: 0,
      overflow: 'hidden',
      paddingBottom: 0,
      paddingInlineEnd: 0,
      paddingInlineStart: 0,
      paddingTop: 0,
      position: 'relative',
      zIndex: 0,
    },
    activityIcon10: {
      height: 22,
      padding: 5,
      width: 22,
    },
    activityIcon16: {
      height: 26,
      padding: 5,
      width: 26,
    },
    activityIcon8: {
      height: 14,
      width: 14,
    },
    badge: {
      borderRadius: '50%',
      position: 'absolute',
      zIndex: 2,
    },
    badgeWithBorder: {
      borderColor: 'var(--surface-background)',
      borderStyle: 'solid',
    },
    badgeWithLastActiveTime: {
      bottom: 0,
      display: 'flex',
      end: 0,
      justifyContent: 'flex-end',
      start: 0,
    },
    badgeWithShadow: {
      boxShadow: '0 0 6px var(--shadow-1)',
    },
    insetSVG: {
      fill: 'none',
      stroke: 'var(--media-inner-border)',
      strokeWidth: '2',
    },
    photo: {
      verticalAlign: 'bottom',
    },
    photoCircle: {
      borderRadius: '50%',
    },
    photoRoundedRect: {
      borderRadius: 8,
    },
    pressable: {
      color: 'var(--primary-text)',
      display: 'inline-block',
    },
    pressed: {
      transform: 'scale(0.96)',
    },
    storyRingBlue: {
      stroke: 'var(--accent)',
    },
    storyRingGray: {
      stroke: 'var(--divider)',
    },
    storyRingGreen: {
      stroke: 'var(--positive)',
    },
    storyRingRed: {
      stroke: 'var(--notification-badge)',
    },
    storyRingSize2: {
      strokeWidth: 2,
    },
    storyRingSize3: {
      strokeWidth: 3,
    },
    storyRingSize4: {
      strokeWidth: 4,
    },
    svgOverlay: {
      fill: 'var(--media-pressed)',
    },
    videoContainer: {
      WebkitMaskImage: '-webkit-radial-gradient(white, black)',
      overflow: 'hidden',
    },
    videoContainerRectRounded: {
      borderRadius: 8,
    },
    videoContainerRounded: {
      borderRadius: '50%',
    },
    wrapper: {
      display: 'inline-block',
      position: 'relative',
      verticalAlign: 'bottom',
      zIndex: 0,
    },
  }),
  stylex.create({
    availabilityBadge: {
      borderRadius: '50%',
      display: 'flex',
      overflow: 'hidden',
      position: 'relative',
    },
  }),
  stylex.create({
    notificationTextContainer: {
      alignItems: 'center',
      display: 'flex',
      height: '100%',
      justifyContent: 'center',
      whiteSpace: 'nowrap',
      width: '100%',
    },
  }),
  stylex.create({
    photo: {
      display: 'block',
      maxWidth: '100%',
      minHeight: '100%',
      objectFit: 'cover',
    },
  }),
  stylex.create({
    multipleAvailableVoicesBadge: {
      borderRadius: '50%',
      display: 'flex',
      overflow: 'hidden',
      position: 'relative',
    },
    ringBorder: {
      borderRadius: '50%',
      display: 'inline-block',
      padding: 4,
      verticalAlign: 'bottom',
    },
  }),
  stylex.create({
    thumbnailContainer: {
      position: 'relative',
    },
  }),
  stylex.create({
    container: {
      position: 'relative',
    },
    glimmer: {
      borderRadius: '50%',
      height: '100%',
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
    },
  }),
  stylex.create({
    completedPrimary: {
      backgroundColor: 'var(--primary-button-background)',
    },
    completedSecondary: {
      backgroundColor: 'var(--secondary-button-background)',
    },
    root: {
      alignItems: 'stretch',
      borderTopStyle: 'solid',
      borderInlineStartStyle: 'solid',
      borderInlineEndStyle: 'solid',
      borderBottomStyle: 'solid',
      borderTopWidth: 0,
      borderInlineStartWidth: 0,
      borderInlineEndWidth: 0,
      borderBottomWidth: 0,
      boxSizing: 'border-box',
      display: 'flex',
      flexGrow: 1,
      flexShrink: 1,
      marginTop: 0,
      marginInlineEnd: 0,
      marginBottom: 0,
      marginInlineStart: 0,
      minHeight: 0,
      minWidth: 0,
      paddingTop: 0,
      paddingInlineEnd: 0,
      paddingBottom: 0,
      paddingInlineStart: 0,
      position: 'relative',
      zIndex: 0,
      flexDirection: 'row',
      flexWrap: 'nowrap',
      justifyContent: 'stretch',
      width: '100%',
    },
    step: {
      backgroundColor: 'var(--comment-background)',
      height: 8,
    },
    stepFirst: {
      borderBottomStartRadius: 4,
      borderTopStartRadius: 4,
    },
    stepLast: {
      borderBottomEndRadius: 4,
      borderTopEndRadius: 4,
    },
    stepSpacedOut: {
      borderBottomEndRadius: 4,
      borderBottomStartRadius: 4,
      borderTopEndRadius: 4,
      borderTopStartRadius: 4,
    },
    stepWrapper: {
      boxSizing: 'border-box',
    },
    stepWrapperSpacedOut: {
      paddingInlineEnd: 2,
      paddingInlineStart: 2,
    },
    stepWrapperSpacedOutFirst: {
      paddingInlineStart: 0,
    },
    stepWrapperSpacedOutLast: {
      paddingInlineEnd: 0,
    },
  }),
  stylex.create({
    pulseEffect: {
      display: 'block',
      marginInline: 8,
      maxWidth: '100%',
    },
    pulseInner: {
      borderRadius: 6,
      margin: 0,
    },
  }),
  stylex.keyframes({
    '0%': {
      opacity: 1,
      transform: 'scale(0)',
    },
    '100%': {
      opacity: 0,
      transform: 'scale(5)',
    },
  }),
  stylex.create({
    positive: {
      backgroundColor: 'var(--positive)',
    },
    primary: {
      backgroundColor: 'var(--primary-button-background)',
    },
  }),
  stylex.create({
    pulse: {
      animationIterationCount: 'infinite',
      animationName: 'x12m5c87-B',
      animationTimingFunction: 'linear',
      backgroundColor: 'var(--primary-button-background)',
      borderRadius: '100%',
      height: 8,
      position: 'absolute',
      transform: 'scale(1)',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'transform',
      transitionTimingFunction: 'ease-in-out',
      width: 8,
    },
    pulseOne: {
      animationDuration: '3s',
    },
    pulseTwo: {
      animationDelay: '2s',
      animationDuration: '3s',
    },
    pulseZeroHovered: {
      transform: 'scale(2)',
    },
    wrapper: {
      display: 'flex',
      height: 0,
      transform: 'translate(-4px, -4px)',
      width: 0,
    },
  }),
  stylex.create({
    4: {
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
    },
    8: {
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
    },
    12: {
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    16: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    0: {
      paddingTop: 0,
    },
    4: {
      paddingTop: 4,
    },
    8: {
      paddingTop: 8,
    },
    12: {
      paddingTop: 12,
    },
    16: {
      paddingTop: 16,
    },
  }),
  stylex.create({
    4: {
      paddingBottom: 4,
      paddingTop: 4,
    },
    8: {
      paddingBottom: 8,
      paddingTop: 8,
    },
    12: {
      paddingBottom: 12,
      paddingTop: 12,
    },
    16: {
      paddingBottom: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    4: {
      marginInlineEnd: -2,
      marginInlineStart: -2,
    },
    8: {
      marginInlineEnd: -4,
      marginInlineStart: -4,
    },
    12: {
      marginInlineEnd: -6,
      marginInlineStart: -6,
    },
    16: {
      marginInlineEnd: -8,
      marginInlineStart: -8,
    },
    24: {
      marginInlineEnd: -12,
      marginInlineStart: -12,
    },
    32: {
      marginInlineEnd: -16,
      marginInlineStart: -16,
    },
  }),
  stylex.create({
    4: {
      marginBottom: -2,
      marginTop: -2,
    },
    8: {
      marginBottom: -4,
      marginTop: -4,
    },
    12: {
      marginBottom: -6,
      marginTop: -6,
    },
    16: {
      marginBottom: -8,
      marginTop: -8,
    },
    24: {
      marginBottom: -12,
      marginTop: -12,
    },
    32: {
      marginBottom: -16,
      marginTop: -16,
    },
  }),
  stylex.create({
    4: {
      paddingInlineEnd: 2,
      paddingInlineStart: 2,
    },
    8: {
      paddingInlineEnd: 4,
      paddingInlineStart: 4,
    },
    12: {
      paddingInlineEnd: 6,
      paddingInlineStart: 6,
    },
    16: {
      paddingInlineEnd: 8,
      paddingInlineStart: 8,
    },
    24: {
      paddingInlineEnd: 12,
      paddingInlineStart: 12,
    },
    32: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    4: {
      paddingBottom: 2,
      paddingTop: 2,
    },
    8: {
      paddingBottom: 4,
      paddingTop: 4,
    },
    12: {
      paddingBottom: 6,
      paddingTop: 6,
    },
    16: {
      paddingBottom: 8,
      paddingTop: 8,
    },
    24: {
      paddingBottom: 12,
      paddingTop: 12,
    },
    32: {
      paddingBottom: 16,
      paddingTop: 16,
    },
  }),
  stylex.create({
    badge: {
      bottom: -2,
      end: -2,
      position: 'absolute',
    },
    colContainer: {
      position: 'relative',
    },
    container: {
      borderRadius: 4,
      maxWidth: 80,
      padding: 8,
    },
    iconContainer: {
      alignItems: 'center',
      backgroundColor: 'var(--background-deemphasized)',
      borderRadius: '100%',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      width: 60,
    },
  }),
  stylex.create({
    container: {
      marginInline: 16,
      padding: '16px 0',
    },
  }),
  stylex.create({
    center: {
      justifyContent: 'center',
    },
    container: {
      alignItems: 'stretch',
      display: 'flex',
      flexDirection: 'row',
      flexWrap: 'wrap',
    },
    dummy: {
      visibility: 'hidden',
    },
    item: {
      flexBasis: 0,
    },
    start: {
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    roundedRect: {
      borderRadius: 8,
    },
    skittle: {
      alignItems: 'center',
      borderWidth: 0,
      boxSizing: 'border-box',
      display: 'inline-flex',
      justifyContent: 'center',
      position: 'relative',
    },
  }),
  stylex.create({
    accent: {
      backgroundColor: 'var(--accent)',
    },
    blue: {
      backgroundColor: 'var(--base-blue)',
    },
    cherry: {
      backgroundColor: 'var(--base-cherry)',
    },
    grape: {
      backgroundColor: 'var(--base-grape)',
    },
    gray: {
      backgroundColor: 'var(--secondary-button-background)',
    },
    green: {
      backgroundColor: 'var(--positive)',
    },
    lemon: {
      backgroundColor: 'var(--base-lemon)',
    },
    lightblue: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
    },
    lime: {
      backgroundColor: 'var(--base-lime)',
    },
    pink: {
      backgroundColor: 'var(--base-pink)',
    },
    red: {
      backgroundColor: 'var(--negative)',
    },
    seafoam: {
      backgroundColor: 'var(--base-seafoam)',
    },
    teal: {
      backgroundColor: 'var(--base-teal)',
    },
    tomato: {
      backgroundColor: 'var(--base-tomato)',
    },
    transparent: {
      backgroundColor: 'transparent',
    },
    white: {
      backgroundColor: 'var(--always-white)',
    },
  }),
  stylex.create({
    24: {
      height: 24,
      width: 24,
    },
    36: {
      height: 36,
      width: 36,
    },
    40: {
      height: 40,
      width: 40,
    },
    48: {
      height: 48,
      width: 48,
    },
    56: {
      height: 56,
      width: 56,
    },
    60: {
      height: 60,
      width: 60,
    },
  }),
  stylex.create({
    circle: {
      borderRadius: '50%',
    },
    iconBadge: {
      alignItems: 'center',
      backgroundColor: 'var(--accent)',
      borderColor: 'var(--card-background)',
      borderRadius: '50%',
      borderStyle: 'solid',
      borderWidth: 2,
      display: 'flex',
      justifyContent: 'center',
      overflow: 'hidden',
      padding: 2,
      position: 'absolute',
    },
    roundedRect: {
      borderRadius: 8,
    },
    skittle: {
      alignItems: 'center',
      borderWidth: 0,
      boxSizing: 'border-box',
      display: 'inline-flex',
      justifyContent: 'center',
      position: 'relative',
    },
  }),
  stylex.create({
    accent: {
      backgroundColor: 'var(--accent)',
    },
    blue: {
      backgroundColor: 'var(--base-blue)',
    },
    cherry: {
      backgroundColor: 'var(--base-cherry)',
    },
    grape: {
      backgroundColor: 'var(--base-grape)',
    },
    gray: {
      backgroundColor: 'var(--secondary-button-background)',
    },
    green: {
      backgroundColor: 'var(--positive)',
    },
    lemon: {
      backgroundColor: 'var(--base-lemon)',
    },
    lightblue: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
    },
    lime: {
      backgroundColor: 'var(--base-lime)',
    },
    pink: {
      backgroundColor: 'var(--base-pink)',
    },
    red: {
      backgroundColor: 'var(--negative)',
    },
    seafoam: {
      backgroundColor: 'var(--base-seafoam)',
    },
    teal: {
      backgroundColor: 'var(--base-teal)',
    },
    tomato: {
      backgroundColor: 'var(--base-tomato)',
    },
    white: {
      backgroundColor: 'var(--always-white)',
    },
  }),
  stylex.create({
    32: {
      height: 32,
      width: 32,
    },
    36: {
      height: 36,
      width: 36,
    },
    40: {
      height: 40,
      width: 40,
    },
    48: {
      height: 48,
      width: 48,
    },
    56: {
      height: 56,
      width: 56,
    },
    60: {
      height: 60,
      width: 60,
    },
  }),
  stylex.create({
    aspectRatioContainer: {
      paddingTop: '100%',
    },
    aspectRatioContainerContent: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    item: {
      float: 'start',
      height: '100%',
      width: '100%',
    },
    root: {
      overflow: 'hidden',
    },
  }),
  stylex.create({
    2: {
      margin: -1,
    },
    4: {
      margin: -2,
    },
  }),
  stylex.create({
    0: {
      height: '50%',
      width: '50%',
    },
    2: {
      height: 'calc(50% - 2px)',
      margin: 1,
      width: 'calc(50% - 2px)',
    },
    4: {
      height: 'calc(50% - 4px)',
      margin: 2,
      width: 'calc(50% - 4px)',
    },
  }),
  stylex.create({
    0: {
      height: '100%',
      width: '50%',
    },
    2: {
      height: 'calc(100% - 2px)',
      margin: 1,
      width: 'calc(50% - 2px)',
    },
    4: {
      height: 'calc(100% - 4px)',
      margin: 2,
      width: 'calc(50% - 4px)',
    },
  }),
  stylex.create({
    image: {
      maxWidth: '100%',
      minHeight: '100%',
      objectFit: 'cover',
      opacity: 1,
      transitionDuration: 'var(--fds-duration-extra-short-in)',
      transitionProperty: 'opacity',
      transitionTimingFunction: 'var(--fds-animation-fade-in)',
    },
    imageDisabled: {
      opacity: 0.4,
    },
    root: {
      overflow: 'hidden',
    },
  }),
  stylex.create({
    transparentBackground: {
      backgroundColor: 'transparent',
    },
  }),
  stylex.create({
    header: {
      transform: 'translateY(0)',
      transitionDuration: 'var(--fds-slow)',
      transitionProperty: 'transform',
    },
    headerHidden: {
      pointerEvents: 'none',
      transform: 'translateY(-100%)',
      transitionDuration: 'var(--fds-slow)',
    },
  }),
  stylex.create({
    icon: {
      alignItems: 'center',
      display: 'inline-flex',
      verticalAlign: 'middle',
    },
    iconContainer: {
      display: 'inline',
      whiteSpace: 'nowrap',
    },
  }),
  stylex.create({
    image: {
      verticalAlign: '-0.25em',
    },
  }),
  stylex.create({
    accent: {
      filter: 'var(--filter-accent)',
    },
    blueLink: {
      filter: 'var(--filter-blue-link-icon)',
    },
    disabled: {
      filter: 'var(--filter-disabled-icon)',
    },
    negative: {
      filter: 'var(--filter-negative)',
    },
    placeholder: {
      filter: 'var(--filter-placeholder-icon)',
    },
    positive: {
      filter: 'var(--filter-positive)',
    },
    primary: {
      filter: 'var(--filter-primary-icon)',
    },
    secondary: {
      filter: 'var(--filter-secondary-icon)',
    },
    warning: {
      filter: 'var(--filter-warning-icon)',
    },
    white: {
      filter: 'var(--filter-always-white)',
    },
  }),
  stylex.create({
    0: {
      paddingBottom: 0,
    },
    8: {
      paddingBottom: 8,
    },
    12: {
      paddingBottom: 12,
    },
    16: {
      paddingBottom: 16,
    },
    20: {
      paddingBottom: 20,
    },
  }),
  stylex.create({
    primary: {
      backgroundColor: 'var(--card-background)',
    },
    transparent: {
      backgroundColor: 'transparent',
    },
  }),
  stylex.create({
    root: {
      paddingBottom: 20,
    },
    rootWithAddOn: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
      paddingTop: 20,
    },
  }),
  stylex.create({
    action: {
      backgroundColor: 'none',
      borderStyle: 'none',
      display: 'inline-block',
      margin: 0,
      padding: 0,
      position: 'relative',
      verticalAlign: 'bottom',
    },
    actionButton: {
      color: 'var(--blue-link)',
      cursor: 'pointer',
    },
    actionHidden: {
      opacity: 0,
    },
    hairline: {
      backgroundColor: 'var(--divider)',
      height: 1,
      marginBottom: -1,
    },
    root: {
      paddingBottom: 4,
    },
    showActionOnHover: {
      visibility: 'hidden',
      '@media (pointer: coarse)': {
        visibility: 'visible',
      },
    },
  }),
  stylex.create({
    0: {
      paddingTop: 0,
    },
    8: {
      paddingTop: 8,
    },
    12: {
      paddingTop: 12,
    },
    16: {
      paddingTop: 16,
    },
    20: {
      paddingTop: 20,
    },
  }),
  stylex.create({
    contentDisabled: {
      opacity: 0.3,
    },
    darkOverlay: {
      backgroundColor: 'var(--always-dark-overlay)',
      color: 'var(--always-white)',
    },
    darkOverlayPressed: {
      backgroundColor: 'var(--non-media-pressed)',
    },
    disabled: {
      backgroundColor: 'var(--disabled-button-background)',
    },
    fdsOverrideBlack: {
      backgroundColor: 'var(--always-black)',
    },
    fdsOverrideCollaborativePostCTA: {
      backgroundColor: 'var(--always-white)',
      mixBlendMode: 'lighten',
    },
    fdsOverrideNegative: {
      backgroundColor: 'var(--negative)',
    },
    fdsOverridePositive: {
      backgroundColor: 'var(--positive)',
    },
    overlay: {
      backgroundColor: 'var(--always-white)',
    },
    overlayDeemphasized: {
      backgroundColor: 'var(--always-light-overlay)',
    },
    overlayDeemphasizedOverlayPressed: {
      backgroundColor: 'var(--always-light-overlay)',
    },
    overlayDisabled: {
      backgroundColor: 'var(--progress-ring-on-media-background)',
    },
    overlayOverlayPressed: {
      backgroundColor: 'var(--shadow-1)',
    },
    paddingIconOnly: {
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
    primary: {
      backgroundColor: 'var(--primary-button-background)',
    },
    primaryDeemphasized: {
      backgroundColor: 'var(--primary-deemphasized-button-background)',
    },
    primaryDeemphasizedOverlayPressed: {
      backgroundColor: 'var(--primary-deemphasized-button-pressed-overlay)',
    },
    primaryOverlayPressed: {
      backgroundColor: 'var(--press-overlay)',
    },
    secondary: {
      backgroundColor: 'var(--secondary-button-background)',
    },
    secondaryDeemphasized: {
      backgroundColor: 'transparent',
    },
    secondaryDeemphasizedOverlayPressed: {
      backgroundColor: 'var(--primary-deemphasized-button-pressed-overlay)',
    },
    secondaryOverlayPressed: {
      backgroundColor: 'var(--press-overlay)',
    },
    sizeLarge: {
      height: 'var(--button-height-large)',
    },
    sizeMedium: {
      height: 'var(--button-height-medium)',
    },
  }),
  stylex.create({
    sizeLarge: {
      borderRadius: 'var(--button-corner-radius-large)',
      height: 'var(--blueprint-button-height-large)',
    },
    sizeMedium: {
      borderRadius: 'var(--button-corner-radius-medium)',
      height: 'var(--blueprint-button-height-medium)',
    },
  }),
  stylex.create({
    hiddenButton: {
      height: 0,
      overflow: 'hidden',
      visibility: 'hidden',
    },
    resetFlexBasis: {
      flexBasis: 'auto',
    },
  }),
  stylex.create({
    base: {
      maxWidth: '100%',
      minWidth: 0,
      wordBreak: 'break-word',
      wordWrap: 'break-word',
    },
    block: {
      display: 'block',
      '::after': {
        content: '""',
        display: 'block',
        height: 0,
      },
      '::before': {
        content: '""',
        display: 'block',
        height: 0,
      },
    },
    heading: {
      maxWidth: '100%',
      minWidth: 0,
    },
    preserveNewLines: {
      whiteSpace: 'pre-line',
    },
  }),
  stylex.create({
    center: {
      textAlign: 'center',
    },
    end: {
      textAlign: 'end',
    },
    start: {
      textAlign: 'start',
    },
  }),
  stylex.create({
    blueLink: {
      color: 'var(--blue-link)',
    },
    disabled: {
      color: 'var(--disabled-text)',
    },
    disabledButton: {
      color: 'var(--disabled-button-text)',
    },
    highlight: {
      color: 'var(--accent)',
    },
    negative: {
      color: 'var(--negative)',
    },
    placeholder: {
      color: 'var(--placeholder-text)',
    },
    positive: {
      color: 'var(--positive)',
    },
    primary: {
      color: 'var(--primary-text)',
    },
    primaryButton: {
      color: 'var(--primary-button-text)',
    },
    primaryDeemphasizedButton: {
      color: 'var(--primary-deemphasized-button-text)',
    },
    primaryOnMedia: {
      color: 'var(--primary-text-on-media)',
    },
    secondary: {
      color: 'var(--secondary-text)',
    },
    secondaryButton: {
      color: 'var(--secondary-button-text)',
    },
    secondaryOnMedia: {
      color: 'var(--secondary-text-on-media)',
    },
    tertiary: {
      color: 'var(--placeholder-text)',
    },
    white: {
      color: 'var(--always-white)',
    },
  }),
  stylex.create({
    12: {
      fontSize: 12,
      lineHeight: 1.3333333333333333,
    },
    13: {
      fontSize: 13,
      lineHeight: 1.2307692307692308,
    },
    14: {
      fontSize: 14,
      lineHeight: 1.2857142857142858,
    },
    15: {
      fontSize: 15,
      lineHeight: 1.3333333333333333,
    },
    16: {
      fontSize: 16,
      lineHeight: 1.25,
    },
    17: {
      fontSize: 17,
      lineHeight: 1.1764705882352942,
    },
    20: {
      fontSize: 20,
      lineHeight: 1.2,
    },
    24: {
      fontSize: 24,
      lineHeight: 1.1666666666666667,
    },
    28: {
      fontSize: 28,
      lineHeight: 1.1428571428571428,
    },
    32: {
      fontSize: 32,
      lineHeight: 1.1875,
    },
  }),
  stylex.create({
    12: {
      fontSize: 12,
      lineHeight: 1.3333333333333333,
    },
    13: {
      fontSize: 12,
      lineHeight: 1.2307692307692308,
    },
    15: {
      fontSize: 14,
      lineHeight: 1.3333333333333333,
    },
    17: {
      fontSize: 16,
      lineHeight: 1.1764705882352942,
    },
    20: {
      fontSize: 20,
      lineHeight: 1.2,
    },
    24: {
      fontSize: 24,
      lineHeight: 1.1666666666666667,
    },
    28: {
      fontSize: 28,
      lineHeight: 1.1428571428571428,
    },
    32: {
      fontSize: 32,
      lineHeight: 1.1875,
    },
  }),
  stylex.create({
    bold: {
      fontWeight: 700,
    },
    medium: {
      fontWeight: 500,
    },
    normal: {
      fontWeight: 400,
    },
    semibold: {
      fontWeight: 600,
    },
  }),
  stylex.create({
    1: {
      '::before': {
        marginTop: -1,
      },
    },
    2: {
      '::before': {
        marginTop: -2,
      },
    },
    3: {
      '::before': {
        marginTop: -3,
      },
    },
    4: {
      '::before': {
        marginTop: -4,
      },
    },
    5: {
      '::before': {
        marginTop: -5,
      },
    },
    6: {
      '::before': {
        marginTop: -6,
      },
    },
    7: {
      '::before': {
        marginTop: -7,
      },
    },
    8: {
      '::before': {
        marginTop: -8,
      },
    },
    9: {
      '::before': {
        marginTop: -9,
      },
    },
    10: {
      '::before': {
        marginTop: -10,
      },
    },
  }),
  stylex.create({
    1: {
      '::after': {
        marginBottom: -1,
      },
    },
    2: {
      '::after': {
        marginBottom: -2,
      },
    },
    3: {
      '::after': {
        marginBottom: -3,
      },
    },
    4: {
      '::after': {
        marginBottom: -4,
      },
    },
    5: {
      '::after': {
        marginBottom: -5,
      },
    },
    6: {
      '::after': {
        marginBottom: -6,
      },
    },
    7: {
      '::after': {
        marginBottom: -7,
      },
    },
    8: {
      '::after': {
        marginBottom: -8,
      },
    },
    9: {
      '::after': {
        marginBottom: -9,
      },
    },
    10: {
      '::after': {
        marginBottom: -10,
      },
    },
  }),
  stylex.create({
    1: {
      paddingBottom: 1,
    },
    2: {
      paddingBottom: 2,
    },
    3: {
      paddingBottom: 3,
    },
  }),
  stylex.create({
    item: {
      marginBottom: 5,
      marginTop: 5,
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      marginBottom: -5,
      marginTop: -5,
    },
  }),
  stylex.create({
    1: {
      marginBottom: -7,
      marginTop: -7,
    },
    2: {
      marginBottom: -6,
      marginTop: -6,
    },
    entityHeader1: {
      marginBottom: -8,
      marginTop: -8,
    },
    entityHeader2: {
      marginBottom: -8,
      marginTop: -8,
    },
  }),
  stylex.create({
    1: {
      marginBottom: 7,
      marginTop: 7,
    },
    2: {
      marginBottom: 6,
      marginTop: 6,
    },
    entityHeader1: {
      marginBottom: 8,
      marginTop: 8,
    },
    entityHeader2: {
      marginBottom: 8,
      marginTop: 8,
    },
  }),
  stylex.create({
    isNoneProfileBadge: {
      marginInlineEnd: 8,
    },
    normalBorderRadius: {
      borderRadius: '50%',
    },
    root: {
      display: 'inline-flex',
    },
  }),
  stylex.create({
    dark: {
      borderColor: 'var(--comment-background)',
    },
    none: {
      borderWidth: 0,
    },
    white: {
      borderColor: 'var(--card-background)',
    },
  }),
  stylex.create({
    6: {
      borderRadius: 3,
      borderStyle: 'solid',
      borderWidth: '1.5px',
      height: 6,
      width: 6,
    },
    7: {
      borderRadius: 3.5,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 7,
      width: 7,
    },
    8: {
      borderRadius: 4,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 8,
      width: 8,
    },
    9: {
      borderRadius: 4.5,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 9,
      width: 9,
    },
    10: {
      borderRadius: 5,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 10,
      width: 10,
    },
    12: {
      borderRadius: 6,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 12,
      width: 12,
    },
    14: {
      borderRadius: 7,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 14,
      width: 14,
    },
    15: {
      borderRadius: 7.5,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 15,
      width: 15,
    },
    18: {
      borderRadius: 9,
      borderStyle: 'solid',
      borderWidth: '2px',
      height: 18,
      width: 18,
    },
    20: {
      borderRadius: 10,
      borderStyle: 'solid',
      borderWidth: '4px',
      height: 20,
      width: 20,
    },
    22: {
      borderRadius: 11,
      borderStyle: 'solid',
      borderWidth: '4px',
      height: 22,
      width: 22,
    },
    24: {
      borderRadius: 12,
      borderStyle: 'solid',
      borderWidth: '4px',
      height: 24,
      width: 24,
    },
    32: {
      borderRadius: 16,
      borderStyle: 'solid',
      borderWidth: '4px',
      height: 32,
      width: 32,
    },
    41: {
      borderRadius: 20.5,
      borderStyle: 'solid',
      borderWidth: '4px',
      height: 41,
      width: 41,
    },
  }),
  stylex.create({
    6: {
      marginInlineStart: 3,
      width: 9,
    },
    7: {
      marginInlineStart: 3.5,
      width: 10.5,
    },
    8: {
      marginInlineStart: 4,
      width: 12,
    },
    9: {
      marginInlineStart: 4.5,
      width: 13.5,
    },
    10: {
      marginInlineStart: 5,
      width: 15,
    },
    12: {
      marginInlineStart: 6,
      width: 18,
    },
    14: {
      marginInlineStart: 7,
      width: 21,
    },
    15: {
      marginInlineStart: 7.5,
      width: 22.5,
    },
    18: {
      marginInlineStart: 9,
      width: 27,
    },
    20: {
      marginInlineStart: 10,
      width: 30,
    },
    22: {
      marginInlineStart: 11,
      width: 33,
    },
    24: {
      marginInlineStart: 12,
      width: 36,
    },
    32: {
      marginInlineStart: 16,
      width: 48,
    },
    41: {
      marginInlineStart: 20.5,
      width: 61.5,
    },
  }),
  stylex.create({
    6: {
      marginInlineStart: 6,
      width: 12,
    },
    7: {
      marginInlineStart: 7,
      width: 14,
    },
    8: {
      marginInlineStart: 8,
      width: 16,
    },
    9: {
      marginInlineStart: 9,
      width: 18,
    },
    10: {
      marginInlineStart: 10,
      width: 20,
    },
    12: {
      marginInlineStart: 12,
      width: 24,
    },
    14: {
      marginInlineStart: 14,
      width: 28,
    },
    15: {
      marginInlineStart: 15,
      width: 30,
    },
    18: {
      marginInlineStart: 18,
      width: 36,
    },
    20: {
      marginInlineStart: 20,
      width: 40,
    },
    22: {
      marginInlineStart: 22,
      width: 44,
    },
    24: {
      marginInlineStart: 24,
      width: 48,
    },
    32: {
      marginInlineStart: 32,
      width: 64,
    },
    41: {
      marginInlineStart: 41,
      width: 82,
    },
  }),
  stylex.create({
    disabled: {
      backgroundColor: 'var(--disabled-icon)',
    },
    icon: {
      display: 'flex',
      marginBottom: -1,
      marginInline: 2,
    },
    label: {
      marginInline: 2,
      marginBlock: 1,
    },
    labelFontWeight: {
      fontWeight: 500,
    },
    root: {
      borderColor: 'var(--card-background)',
      borderRadius: 12,
      borderStyle: 'solid',
      borderWidth: 2,
      display: 'inline-flex',
      margin: -2,
      paddingInline: 4,
      paddingBlock: 4,
    },
  }),
  stylex.create({
    accent: {
      backgroundColor: 'var(--accent)',
    },
    'event-date': {
      backgroundColor: 'var(--event-date)',
    },
    'notification-badge': {
      backgroundColor: 'var(--notification-badge)',
    },
    positive: {
      backgroundColor: 'var(--positive)',
    },
    'secondary-badge': {
      backgroundColor: 'var(--secondary-icon)',
    },
  }),
  stylex.create({
    badgeCount: {
      alignItems: 'center',
      color: 'var(--primary-button-text)',
      display: 'inline-flex',
      height: '100%',
      justifyContent: 'center',
      padding: '0 5px',
      width: '100%',
    },
    badgeCountLightBlue: {
      color: 'var(--accent)',
    },
    rectangle: {
      borderRadius: 100,
    },
    root: {
      borderRadius: '50%',
      display: 'inline-flex',
      fontSize: 13,
      fontWeight: 500,
      height: 19,
      lineHeight: 1,
      minWidth: 19,
    },
  }),
  stylex.create({
    blue: {
      backgroundColor: 'var(--accent)',
    },
    darkGray: {
      backgroundColor: 'var(--secondary-icon)',
    },
    gray: {
      backgroundColor: 'var(--disabled-icon)',
    },
    green: {
      backgroundColor: 'var(--positive)',
    },
    lightBlue: {
      backgroundColor: 'var(--highlight-bg)',
    },
    red: {
      backgroundColor: 'var(--notification-badge)',
    },
    yellow: {
      backgroundColor: 'var(--base-lemon)',
    },
  }),
  stylex.create({
    buttonShadow: {
      borderRadius: '50%',
      boxShadow: '0px 2px 6px var(--shadow-1)',
    },
    menuButtonContainer: {
      pointerEvents: 'auto',
      position: 'absolute',
      top: '50%',
      transform: 'translateY(-50%)',
    },
    menuButtonContainerFar: {
      end: 36,
    },
    menuButtonContainerNear: {
      end: 8,
    },
    visuallyHidden: {
      WebkitClipPath: 'circle(1px at 0% 0%)',
      clip: 'rect(1px, 1px, 1px, 1px)',
      clipPath: 'circle(1px at 0% 0%)',
      height: 1,
      overflow: 'hidden',
      width: 1,
    },
  }),
  stylex.create({
    badge: {
      end: 0,
      pointerEvents: 'none',
      position: 'absolute',
      top: 0,
      transform: 'translate(50%, -50%)',
      zIndex: 1,
    },
    container: {
      position: 'relative',
    },
  }),
  stylex.create({
    addon: {
      marginInlineEnd: 8,
    },
    dot: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'center',
      width: '1em',
    },
    item: {
      alignItems: 'center',
      display: 'flex',
      flexDirection: 'row',
      marginBlock: 4,
    },
    title: {
      marginInlineStart: '-1em',
    },
    titleContainer: {
      marginBlock: -4,
      overflow: 'hidden',
    },
  }),
  stylex.create({
    root: {
      marginInlineStart: -8,
      marginTop: -8,
      maxWidth: 'calc(100% + 16px)',
      width: 'calc(100% + 16px)',
    },
  }),
  stylex.create({
    columnStyle: {
      display: 'flex',
      flexGrow: 1,
      flexShrink: 1,
    },
    footerMargin: {
      marginTop: 8,
    },
    mediaImageSize: {
      maxHeight: '100%',
      maxWidth: '100%',
    },
    overlay: {
      position: 'absolute',
    },
    overlayFullSize: {
      bottom: 0,
      boxSizing: 'border-box',
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
    },
    pressed: {
      transform: 'scale(0.96)',
    },
    rounded: {
      borderRadius: 8,
    },
  }),
  stylex.create({
    bottom: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
    },
    bottomLeft: {
      bottom: 8,
      position: 'absolute',
      start: 8,
    },
    bottomRight: {
      bottom: 8,
      end: 8,
      position: 'absolute',
    },
    center: {
      position: 'absolute',
      start: '50%',
      top: '50%',
    },
    default: {
      bottom: 8,
      end: 8,
      position: 'absolute',
    },
    topLeft: {
      position: 'absolute',
      start: 8,
      top: 8,
    },
    topRight: {
      end: 8,
      position: 'absolute',
      top: 8,
    },
  }),
  stylex.create({
    60: {
      height: 60,
      width: 60,
    },
    144: {
      maxHeight: 144,
      maxWidth: 144,
    },
    160: {
      maxHeight: 160,
      maxWidth: 160,
    },
  }),
  stylex.create({
    container: {
      marginInline: -4,
      marginBlock: -6,
    },
    item: {
      paddingInline: 4,
      paddingBlock: 6,
    },
  }),
  stylex.create({
    container: {
      marginBlock: -6,
    },
    item: {
      paddingInline: 0,
      paddingBlock: 6,
    },
  }),
  stylex.create({
    fbonlyContainer: {
      borderColor: 'var(--negative)',
      borderStyle: 'solid',
      borderWidth: 1,
      marginBottom: 5,
      padding: 3,
    },
    fbonlyText: {
      color: 'var(--negative)',
      fontSize: 15,
      fontWeight: 500,
    },
  }),
  stylex.create({
    content: {
      paddingInline: '16px',
      position: 'relative',
    },
  }),
  stylex.create({
    anchor: {
      paddingInline: 8,
      paddingBlock: 'var(--dialog-anchor-vertical-padding)',
      '@media (max-width: 564px)': {
        paddingInline: 0,
      },
    },
    anchorInMobileEnvironment: {
      paddingBlock: 0,
    },
    backButton: {
      position: 'absolute',
      start: 16,
      top: 12,
      zIndex: 1,
    },
    card: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow:
        '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1), inset 0 0 0 1px var(--shadow-inset)',
      '@media (max-width: 564px)': {
        borderRadius: 0,
      },
    },
    closeButton: {
      end: 16,
      position: 'absolute',
      top: 12,
      zIndex: 1,
    },
    header: {
      boxSizing: 'border-box',
      height: 60,
    },
    headerBottomBorder: {
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
    },
    headerWithBackButton: {
      paddingInlineStart: 60,
    },
    headerWithCloseButton: {
      paddingInlineEnd: 60,
    },
    headerWithPadding: {
      paddingInlineEnd: 60,
      paddingInlineStart: 60,
    },
    rootInMobileEnvironment: {
      justifyContent: 'flex-start',
    },
    titleWrapper: {
      alignItems: 'center',
      boxSizing: 'border-box',
      display: 'flex',
      height: '100%',
      paddingInlineEnd: 16,
      paddingInlineStart: 16,
    },
  }),
  stylex.create({
    content: {
      maxWidth: '100%',
    },
    'content-mobile-safe': {
      width: '100%',
    },
    medium: {
      maxWidth: 700,
      width: '100%',
    },
    small: {
      maxWidth: 548,
      width: '100%',
    },
  }),
  stylex.create({
    center: {
      justifyContent: 'center',
    },
    start: {
      justifyContent: 'flex-start',
    },
  }),
  stylex.create({
    bodyGlimmer: {
      borderRadius: 7,
      height: 14,
      marginBottom: 14,
    },
    bodyGlimmerContainer: {
      padding: '20px 20px 150px 20px',
    },
    bodyGlimmerFirst: {
      width: '80%',
    },
    bodyGlimmerSecond: {
      width: '40%',
    },
    header: {
      alignItems: 'center',
      borderBottomWidth: 1,
      borderBottomStyle: 'solid',
      borderBottomColor: 'var(--media-inner-border)',
      display: 'flex',
      height: 60,
      justifyContent: 'center',
      textAlign: 'center',
    },
    headerGlimmer: {
      borderRadius: 7,
      height: 14,
      width: 100,
    },
  }),
  stylex.keyframes({
    '0%': {
      transform: 'scale(0.98)',
    },
    '100%': {
      transform: 'scale(1)',
    },
  }),
  stylex.create({
    root: {
      animationDuration: 'var(--fds-fast)',
      animationName: 'xitoqud-B',
      animationTimingFunction: 'var(--fds-soft)',
    },
  }),
  stylex.keyframes({
    '0%': {
      transform: 'scale(0.98)',
    },
    '100%': {
      transform: 'scale(1)',
    },
  }),
  stylex.create({
    root: {
      animationDuration: 'var(--fds-fast)',
      animationName: 'xitoqud-B',
      animationTimingFunction: 'var(--fds-soft)',
    },
  }),
  stylex.create({
    container: {
      position: 'absolute',
      width: '100%',
    },
    headerItem: {
      marginInline: 16,
    },
    headerPlaceholder: {
      height: 36,
      width: 36,
    },
    image: {
      height: '100%',
      objectFit: 'cover',
      verticalAlign: 'middle',
      width: '100%',
    },
  }),
  stylex.create({
    item: {
      flexBasis: 0,
      minWidth: 'fit-content',
      '@media (max-width: 679px)': {
        minWidth: '50%',
      },
    },
    secondary: {
      paddingInlineEnd: 8,
    },
  }),
  stylex.create({
    anchor: {
      alignItems: 'stretch',
      maxHeight: '100vh',
      paddingInline: 4,
      paddingBlock: 'var(--dialog-anchor-vertical-padding)',
      '@supports (padding: env(safe-area-inset-bottom, 0))': {
        paddingBottom:
          'calc(var(--dialog-anchor-vertical-padding) + env(safe-area-inset-bottom, 0))',
        paddingTop: 'calc(var(--dialog-anchor-vertical-padding) + env(safe-area-inset-top, 0))',
      },
    },
    card: {
      backgroundColor: 'var(--card-background)',
      borderRadius: 'var(--card-corner-radius)',
      boxShadow:
        '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1), inset 0 0 0 1px var(--shadow-inset)',
      clipPath: 'none',
      flexGrow: 1,
      overflow: 'hidden',
      '@media (max-width: 679px)': {
        boxShadow: 'none',
        clipPath: 'inset(0px 0px 0px 0px round var(--card-corner-radius))',
        overflow: 'visible',
      },
    },
    dialog: {
      alignItems: 'stretch',
      borderRadius: 'var(--card-corner-radius)',
      display: 'flex',
      overflow: 'visible',
      '@media (max-width: 679px)': {
        boxShadow:
          '0 12px 28px 0 var(--shadow-2), 0 2px 4px 0 var(--shadow-1), inset 0 0 0 1px var(--shadow-inset)',
      },
    },
    root: {
      '@media (max-width: 679px)': {
        justifyContent: 'center',
      },
    },
  }),
  stylex.create({
    medium: {
      maxWidth: 700,
      width: '100%',
    },
    small: {
      maxWidth: 548,
      width: '100%',
    },
  }),
  stylex.create({
    headerItem: {
      marginInline: 16,
    },
    headerPlaceholder: {
      height: 36,
      width: 36,
    },
  }),
  stylex.create({
    container: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      minHeight: 50,
    },
    inert: {
      pointerEvents: 'none',
      userSelect: 'none',
    },
    placeholder: {
      opacity: 0,
      pointerEvents: 'none',
      position: 'relative',
    },
    root: {
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      maxHeight: 'calc(100vh - (2 * var(--dialog-anchor-vertical-padding)))',
      position: 'relative',
      '@media (max-width: 679px)': {
        maxHeight: 'none',
      },
    },
    rootFullHeight: {
      minHeight: 'calc(100vh - (2 * var(--dialog-anchor-vertical-padding)))',
    },
    rootMinHeight: {
      '@media (max-width: 679px)': {
        minHeight: '100vh',
      },
    },
    scrollableArea: {
      flexGrow: 1,
      overscrollBehaviorY: 'auto',
    },
    scrollSectionObserver: {
      height: 1,
    },
  }),
  stylex.create({
    addOnEnd: {
      alignSelf: 'flex-start',
      marginInline: 16,
      marginBlock: 12,
    },
    headerPlaceholder: {
      height: 36,
      marginBlock: 16,
      width: 36,
    },
    headerText: {
      marginInlineStart: 16,
      marginBlock: 16,
    },
  }),
  stylex.create({
    addOnEnd: {
      marginInline: 16,
    },
    headerPlaceholder: {
      height: 36,
      marginInline: 16,
      width: 36,
    },
    tabs: {
      paddingInline: 16,
    },
  }),
  stylex.create({
    footer: {
      flexShrink: 0,
      padding: '9px 0px',
    },
    scrollArea: {
      maxHeight: '50vh',
    },
    scrollAreaInner: {
      paddingBlock: 3,
    },
  }),
  stylex.create({
    content: {
      padding: '12px 16px',
      position: 'relative',
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--card-background)',
      width: '100%',
      '@media (max-width: 679px)': {
        backgroundColor: 'var(--card-background)',
        borderBottomEndRadius: 'var(--card-corner-radius)',
        borderBottomStartRadius: 'var(--card-corner-radius)',
        bottom: 0,
        boxShadow: 'var(--scroll-shadow)',
        position: 'sticky',
      },
    },
    containerFloated: {
      bottom: 0,
      end: 0,
      position: 'absolute',
      start: 0,
      zIndex: 1,
    },
    containerStatic: {
      position: 'relative',
      '@media (max-width: 679px)': {
        position: 'sticky',
      },
    },
  }),
  stylex.create({
    item: {
      flexBasis: 0,
      minWidth: 'fit-content',
      '@media (max-width: 679px)': {
        minWidth: '100%',
      },
    },
  }),
  stylex.create({
    container: {
      backgroundColor: 'var(--card-background)',
      width: '100%',
    },
    containerFloated: {
      end: 0,
      position: 'absolute',
      start: 0,
      top: 0,
      width: '100%',
      zIndex: 1,
    },
    containerStatic: {
      position: 'relative',
    },
    content: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'space-between',
      minHeight: 60,
      position: 'relative',
    },
  }),
  stylex.create({
    headerContainer: {
      position: 'relative',
      width: '100%',
    },
    headerFloated: {
      position: 'absolute',
    },
    headerRow: {
      alignItems: 'center',
      display: 'flex',
      flexShrink: 0,
      justifyContent: 'flex-end',
      marginInline: 16,
      minHeight: 60,
    },
  }),
  stylex.create({
    firstLine: {
      height: 12,
      marginBottom: 10,
      maxWidth: 440,
    },
    glimmer: {
      alignSelf: 'flex-start',
      borderRadius: 8,
      boxSizing: 'border-box',
      marginInline: 16,
      width: 'calc(100% - 40px)',
    },
    heading: {
      height: 20,
      marginBlock: 20,
      maxWidth: 241,
    },
    secondLine: {
      height: 12,
      marginBottom: 20,
      maxWidth: 296,
    },
  }),
  stylex.create({
    bottomShadow: {
      top: -1,
      transform: 'rotate(180deg)',
    },
    hidden: {
      opacity: 0,
    },
    shadow: {
      bottom: 0,
      boxShadow: 'var(--scroll-shadow)',
      height: 1,
      position: 'absolute',
      transitionDuration: 'var(--fds-fast)',
      transitionProperty: 'opacity',
      width: '100%',
    },
  }),
  stylex.create({
    row: {
      margin: '20px 20px 20px',
    },
  }),
];
