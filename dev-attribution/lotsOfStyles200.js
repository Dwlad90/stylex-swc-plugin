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
];
