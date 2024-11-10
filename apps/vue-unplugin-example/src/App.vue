<script setup lang="ts">
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex';
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex';
import * as stylex from '@stylexjs/stylex';
</script>

<script lang="ts">
const styles = stylex.create({
  container: {
    alignItems: 'center',
    backgroundColor: colors.choco3,
    display: 'grid',
    height: '100dvh',
    justifyContent: 'center',
    width: '100dvw',
  },
  button: {
    alignItems: 'center',
    backgroundColor: colors.jungle6,
    borderRadius: sizes.spacing15,
    color: colors.gray2,
    display: 'flex',
    fontFamily: fonts.mono,
    justifyContent: 'center',
    paddingBlock: sizes.spacing5,
    paddingInline: sizes.spacing10,
    borderWidth: 0,
    fontWeight: 'bold',
    ":hover": {
      backgroundColor: colors.jungle8,
    },
    ":active": {
      backgroundColor: colors.jungle10,
    }
  },
});
</script>

<template>
  <main :class="stylex.props(styles.container).className">
    <button :class="stylex.props(styles.button).className">
      Click Me
    </button>
  </main>
</template>
