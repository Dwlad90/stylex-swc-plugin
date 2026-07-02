import 'server-only';

import * as stylex from '@stylexjs/stylex';
import { colors, globalTokens as $, spacing, text } from '@/app/globalTokens.stylex';
import Counter from './Counter';

/**
 * Server component page (no 'use client'): rendered on the server, ships no
 * JavaScript of its own. Its StyleX rules — and those of the <Counter />
 * client island it composes — are extracted into one static stylesheet by
 * @stylexswc/rspack-plugin.
 */
export default function RscPage() {
  return (
    <div {...stylex.props(styles.page)}>
      <header {...stylex.props(styles.hero)}>
        <div {...stylex.props(styles.badge, styles.heroBadge)}>React Server Components</div>
        <h1 {...stylex.props(styles.title)}>
          Server + Client,
          <span {...stylex.props(styles.titleAccent)}> one stylesheet</span>
        </h1>
        <p {...stylex.props(styles.lede)}>
          The card on the left renders entirely on the server. The one on the right hydrates as an
          interactive client island. Both are styled with StyleX, compiled by the Rust compiler, and
          extracted into a single static CSS file — no runtime style injection anywhere.
        </p>
      </header>

      <div {...stylex.props(styles.grid)}>
        <section {...stylex.props(styles.card, styles.serverCard)}>
          <div {...stylex.props(styles.badge, styles.serverBadge)}>Server Component</div>
          <h2 {...stylex.props(styles.cardTitle)}>Zero-JS rendering</h2>
          <p {...stylex.props(styles.cardBody)}>
            No <code {...stylex.props(styles.inlineCode)}>&apos;use client&apos;</code> directive,
            no hydration, no bundle cost. The markup arrives fully styled because the CSS below was
            generated at build time.
          </p>
          <ul {...stylex.props(styles.list)}>
            <li {...stylex.props(styles.listItem)}>
              <span {...stylex.props(styles.check)}>✓</span>
              Styles compiled by the NAPI-RS StyleX compiler
            </li>
            <li {...stylex.props(styles.listItem)}>
              <span {...stylex.props(styles.check)}>✓</span>
              Extracted through virtual CSS imports in the server graph
            </li>
            <li {...stylex.props(styles.listItem)}>
              <span {...stylex.props(styles.check)}>✓</span>
              Mirrored into the client build — deduplicated, sorted, atomic
            </li>
          </ul>
          <pre {...stylex.props(styles.codeBlock)}>
            <code>{'stylex.create({ card: { … } })\n→ .x1a2b3c { … }  /* static CSS */'}</code>
          </pre>
        </section>

        <Counter />
      </div>

      <footer {...stylex.props(styles.footer)}>
        Rendered with <span {...stylex.props(styles.footerAccent)}>Next.js + PostCSS</span> · StyleX
        extraction for Server and Client Components via{' '}
        <code {...stylex.props(styles.inlineCode)}>@stylexswc/postcss-plugin</code>
      </footer>
    </div>
  );
}

const DARK = '@media (prefers-color-scheme: dark)' as const;
const MOBILE = '@media (max-width: 768px)' as const;

const styles = stylex.create({
  page: {
    display: 'flex',
    flexDirection: 'column',
    gap: spacing.xl,
    width: '100%',
    maxWidth: $.maxWidth,
  },
  hero: {
    display: 'flex',
    flexDirection: 'column',
    gap: spacing.sm,
    alignItems: 'center',
    marginTop: spacing.xxl,
    textAlign: 'center',
  },
  heroBadge: {
    color: colors.purple6,
    backgroundColor: {
      default: 'rgba(124, 106, 232, 0.1)',
      [DARK]: 'rgba(124, 106, 232, 0.2)',
    },
  },
  title: {
    margin: 0,
    fontSize: text.h1,
    fontWeight: 700,
    letterSpacing: '-0.02em',
    lineHeight: 1.1,
  },
  titleAccent: {
    backgroundImage: `linear-gradient(100deg, ${'#1c7ed6'}, ${'#7C6AE8'})`,
    backgroundClip: 'text',
    color: 'transparent',
  },
  lede: {
    maxWidth: 640,
    margin: 0,
    fontSize: text.p,
    lineHeight: 1.7,
    color: {
      default: colors.gray6,
      [DARK]: colors.gray5,
    },
  },
  grid: {
    display: 'grid',
    gridTemplateColumns: {
      default: '1fr 1fr',
      [MOBILE]: '1fr',
    },
    gap: spacing.lg,
    alignItems: 'start',
  },
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
  serverCard: {
    borderTopColor: colors.emerald,
    borderTopWidth: 3,
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
  serverBadge: {
    color: colors.emerald,
    backgroundColor: {
      default: 'rgba(12, 166, 120, 0.08)',
      [DARK]: 'rgba(12, 166, 120, 0.18)',
    },
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
  list: {
    display: 'flex',
    flexDirection: 'column',
    gap: spacing.xxs,
    margin: 0,
    padding: 0,
    listStyle: 'none',
  },
  listItem: {
    display: 'flex',
    gap: spacing.xxs,
    alignItems: 'baseline',
    fontSize: text.sm,
    lineHeight: 1.6,
  },
  check: {
    fontWeight: 700,
    color: colors.emerald,
  },
  inlineCode: {
    paddingInline: spacing.xxxs,
    fontFamily: $.fontMono,
    fontSize: '0.9em',
    backgroundColor: $.calloutRGB50,
    borderRadius: 4,
  },
  codeBlock: {
    margin: 0,
    marginTop: spacing.xxs,
    padding: spacing.sm,
    overflowX: 'auto',
    fontFamily: $.fontMono,
    fontSize: text.xs,
    lineHeight: 1.6,
    backgroundColor: $.calloutRGB,
    borderRadius: spacing.xs,
  },
  footer: {
    marginBottom: spacing.xxl,
    fontSize: text.xs,
    textAlign: 'center',
    color: {
      default: colors.gray5,
      [DARK]: colors.gray6,
    },
  },
  footerAccent: {
    fontWeight: 600,
    color: colors.accent,
  },
});
