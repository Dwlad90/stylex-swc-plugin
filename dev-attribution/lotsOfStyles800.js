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
];
