import { colors } from '@stylexjs/open-props/lib/colors.stylex'
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex'
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex'
import * as stylex from '@stylexjs/stylex'
import { render } from 'solid-js/web'
import { tokens } from '@stylexswc/design-system/tokens.stylex';

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
})

function App() {
  return (
    <div {...stylex.props(styles.main)}>
      <div {...stylex.props(styles.card, styles.blueBg)}>
        <span {...stylex.props(styles.orangeFg)}>Blue rounded rectangle with orange text</span>
      </div>
      <div {...stylex.props(styles.card, styles.pinkBg)}>
      <span {...stylex.props(styles.blueFg)}>Pink rounded rectangle with blue text</span>
      </div>
    </div>
  )
}

const root = document.getElementById('root')

if (root) {
  render(() => <App />, root)
}