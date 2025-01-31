<script setup lang="ts">
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex';
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex';
import * as stylex from '@stylexjs/stylex';
import { tokens } from '@stylexswc/design-system/tokens.stylex';
</script>

<script lang="ts">
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
});
</script>

<template>
  <div :class="stylex.props(styles.main).className">
    <div :class="stylex.props(styles.card, styles.blueBg,styles.orangeFg).className">
      <span>Blue rounded rectangle with orange text</span>
    </div>
    <div :class="stylex.props(styles.card, styles.pinkBg, styles.blueFg).className">
      <span>Pink rounded rectangle with blue text</span>
    </div>
  </div>
</template>
