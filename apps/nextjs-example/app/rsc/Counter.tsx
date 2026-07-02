'use client';

import { useState } from 'react';
import * as stylex from '@stylexjs/stylex';
import { colors, globalTokens as $, spacing, text } from '@/app/globalTokens.stylex';

/**
 * Client island: interactive state, event handlers, and conditional styles.
 * Its StyleX rules are compiled into the same extracted stylesheet as the
 * server components around it.
 */
export default function Counter() {
  const [count, setCount] = useState(0);

  return (
    <section {...stylex.props(styles.card, styles.clientCard)}>
      <div {...stylex.props(styles.badge, styles.clientBadge)}>Client Component</div>
      <h2 {...stylex.props(styles.cardTitle)}>Interactive island</h2>
      <p {...stylex.props(styles.cardBody)}>
        This card ships JavaScript: <code {...stylex.props(styles.inlineCode)}>useState</code>,
        event handlers, and conditional styles — all resolved from the same compile-time CSS.
      </p>
      <div {...stylex.props(styles.counterRow)}>
        <button
          {...stylex.props(styles.counterButton)}
          aria-label="Decrement"
          onClick={() => setCount(c => c - 1)}
        >
          −
        </button>
        <output {...stylex.props(styles.count, count !== 0 && styles.countActive)}>{count}</output>
        <button
          {...stylex.props(styles.counterButton)}
          aria-label="Increment"
          onClick={() => setCount(c => c + 1)}
        >
          +
        </button>
      </div>
      <button
        {...stylex.props(styles.resetButton, count === 0 && styles.resetHidden)}
        onClick={() => setCount(0)}
      >
        Reset
      </button>
    </section>
  );
}

const DARK = '@media (prefers-color-scheme: dark)' as const;

const styles = stylex.create({
  card: {
    display: 'flex',
    flexDirection: 'column',
    gap: spacing.sm,
    padding: spacing.lg,
    backgroundColor: $.surfaceCard,
    borderColor: {
      default: colors.gray3,
      [DARK]: colors.gray8,
    },
    borderStyle: 'solid',
    borderWidth: 1,
    borderRadius: spacing.md,
    boxShadow: $.surfaceCardShadow,
  },
  clientCard: {
    borderTopColor: colors.accent,
    borderTopWidth: 3,
    transform: {
      default: null,
      ':hover': 'translateY(-2px)',
    },
    transitionDuration: '200ms',
    transitionProperty: 'transform, box-shadow',
  },
  badge: {
    alignSelf: 'flex-start',
    paddingBlock: spacing.xxxs,
    paddingInline: spacing.xs,
    fontSize: text.xxs,
    fontWeight: 700,
    letterSpacing: '0.08em',
    textTransform: 'uppercase',
    borderRadius: spacing.sm,
  },
  clientBadge: {
    color: colors.accent,
    backgroundColor: colors.accentLight,
  },
  cardTitle: {
    margin: 0,
    fontSize: text.h5,
    fontWeight: 600,
  },
  cardBody: {
    margin: 0,
    fontSize: text.sm,
    lineHeight: 1.6,
    color: {
      default: colors.gray6,
      [DARK]: colors.gray5,
    },
  },
  inlineCode: {
    paddingInline: spacing.xxxs,
    fontFamily: $.fontMono,
    fontSize: '0.9em',
    backgroundColor: $.calloutRGB50,
    borderRadius: 4,
  },
  counterRow: {
    display: 'flex',
    gap: spacing.md,
    alignItems: 'center',
    justifyContent: 'center',
    marginTop: spacing.xs,
  },
  counterButton: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: 48,
    height: 48,
    fontFamily: $.fontSans,
    fontSize: text.h4,
    fontWeight: 300,
    color: colors.accent,
    cursor: 'pointer',
    backgroundColor: {
      default: 'transparent',
      ':hover': {
        default: colors.gray2,
        [DARK]: colors.gray9,
      },
    },
    borderColor: colors.accent,
    borderStyle: 'solid',
    borderWidth: 1,
    borderRadius: spacing.xs,
    transform: {
      default: null,
      ':active': 'scale(0.92)',
    },
    transitionDuration: '150ms',
    transitionProperty: 'background-color, transform',
  },
  count: {
    minWidth: 96,
    fontFamily: $.fontMono,
    fontSize: text.h2,
    fontWeight: 200,
    textAlign: 'center',
    transitionDuration: '200ms',
    transitionProperty: 'color, font-weight',
  },
  countActive: {
    fontWeight: 500,
    color: colors.accent,
  },
  resetButton: {
    alignSelf: 'center',
    paddingBlock: spacing.xxxs,
    paddingInline: spacing.sm,
    fontFamily: $.fontSans,
    fontSize: text.xs,
    color: {
      default: colors.gray6,
      ':hover': colors.accent,
    },
    cursor: 'pointer',
    backgroundColor: 'transparent',
    borderColor: {
      default: colors.gray4,
      ':hover': colors.accent,
    },
    borderStyle: 'solid',
    borderWidth: 1,
    borderRadius: spacing.xs,
    opacity: 1,
    transitionDuration: '200ms',
    transitionProperty: 'color, border-color, opacity',
  },
  resetHidden: {
    opacity: 0,
    pointerEvents: 'none',
  },
});
