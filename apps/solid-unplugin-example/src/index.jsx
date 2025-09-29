import { colors } from '@stylexjs/open-props/lib/colors.stylex'
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex'
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex'
import * as stylex from '@stylexjs/stylex'
import { render } from 'solid-js/web'
import { tokens } from '@toss/stylexswc-design-system/tokens.stylex';
import { breakpoints } from '@toss/stylexswc-design-system/consts.stylex';

const styles = stylex.create({
  main: {
    width: '100vw',
    height: '100vh',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-around',
    backgroundColor: colors.violet1,
    flexDirection: 'column',
  },
  card: {
    padding: sizes.spacing5,
    borderRadius: sizes.spacing2,
    justifyContent: 'center',
    display: 'flex',
    alignItems: 'center',
    fontFamily: fonts.mono,
  },
  blueBg: {
    backgroundColor: colors.blue3,
  },
  pinkBg: {
    backgroundColor: tokens.pink7,
  },
  orangeFg: {
    color: colors.orange7,
  },
  blueFg: {
    color: tokens.blue9,
  },
  redBg: {
    backgroundColor: tokens.red7,
  },
  greenFg: {
    color: tokens.green7,
  },
  onlyMobile: {
    display: {
      default: 'none',
      [breakpoints.mobile]: 'block',
    }
  },
  onlyDesktop: {
    display: {
      default: 'none',
      [breakpoints.desktop]: 'block',
    }
  },
})

function App() {
  return (
    <div {...stylex.props(styles.main)}>
      <div {...stylex.props(styles.card, styles.blueBg)}>
        <span {...stylex.props(styles.orangeFg)}>Blue rounded rectangle with orange text</span>
      </div>
      <div {...stylex.props(styles.card, styles.pinkBg, styles.onlyMobile)}>
        <span {...stylex.props(styles.blueFg)}>Pink rounded rectangle with blue text</span>
      </div>
      <div {...stylex.props(styles.card, styles.redBg, styles.onlyDesktop)}>
        <span {...stylex.props(styles.greenFg)}>Red rounded rectangle with green text</span>
      </div>
    </div>
  )
}

const root = document.getElementById('root')

if (root) {
  render(() => <App />, root)
}