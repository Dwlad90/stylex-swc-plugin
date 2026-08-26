import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex';
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex';
import * as stylex from '@stylexjs/stylex';

// Reached through a dynamic import on purpose: these rules are compiled after
// the stylesheet holding the marker has already been loaded.
const styles = stylex.create({
  card: {
    padding: sizes.spacing5,
    borderRadius: sizes.spacing2,
    backgroundColor: colors.green3,
    color: colors.gray9,
    fontFamily: fonts.mono,
  },
});

export default function LateCard() {
  return (
    <div data-testid="late-card" {...stylex.props(styles.card)}>
      Green rectangle from a lazily loaded module
    </div>
  );
}
