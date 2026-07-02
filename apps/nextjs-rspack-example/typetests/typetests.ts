/* eslint-disable no-unused-vars, @typescript-eslint/no-empty-object-type, @typescript-eslint/no-unused-expressions */
import * as stylex from '@stylexjs/stylex';
import type {
  StaticStyles,
  StyleXStyles,
  StaticStylesWithout,
  StyleXStylesWithout,
} from '@stylexjs/stylex';
import type { InlineStyles } from '@stylexjs/stylex';

type NotUndefined = {} | null;

/**
 * EMPTY STYLES
 */
const styles1: Readonly<{ foo: Readonly<{}> }> = stylex.create({
  foo: {},
});
styles1.foo as StaticStyles;
styles1.foo as StaticStyles<{}>;
styles1.foo as StaticStyles<{ width?: number | string }>;
styles1.foo as StaticStyles<{ width?: NotUndefined }>;
styles1.foo as StaticStylesWithout<{ width: NotUndefined }>;
styles1.foo as StyleXStyles;
styles1.foo as StyleXStyles<{}>;
styles1.foo as StyleXStyles<{ width?: number | string }>;
styles1.foo as StyleXStyles<{ width?: NotUndefined }>;
styles1.foo as StyleXStylesWithout<{ width: NotUndefined }>;

stylex.props(styles1.foo);

/**
 * SIMPLE STYLES
 */
// @ts-expect-error - will be fixed in the next version
const styles2: Readonly<{
  foo: Readonly<{
    width: '100%';
  }>;
}> = stylex.create({
  foo: {
    width: '100%',
  },
});
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StaticStyles;
// @ts-expect-error - We want to disallow extra keys
styles2.foo satisfies StaticStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StaticStyles<{ width: '100%' }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StaticStyles<{ width: unknown }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StaticStylesWithout<{ height: unknown }>;
// @ts-expect-error - The style does have `width`
styles2.foo satisfies StaticStylesWithout<{ width: unknown }>;
// @ts-expect-error - 'number' is not assignable to '100%'.
styles2.foo satisfies StaticStyles<{ width: 100 }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StaticStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StaticStyles<{ width?: unknown; height?: string }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles2.foo satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StyleXStyles<{ width: '100%' }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StyleXStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StyleXStyles<{ width?: unknown }>;
// @ts-expect-error - will be fixed in the next version
styles2.foo satisfies StyleXStylesWithout<{ height: unknown }>;
// @ts-expect-error - The style does have `width`
styles2.foo satisfies StyleXStylesWithout<{ width: unknown }>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles2.foo);

/**
 * FALLBACK STYLES
 */
// @ts-expect-error - will be fixed in the next version
const styles3: Readonly<{
  foo: Readonly<{
    width: '100%' | '200%';
  }>;
}> = stylex.create({
  foo: {
    width: stylex.firstThatWorks('100%', '200%'),
  },
});
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StaticStyles;
// @ts-expect-error - We want to disallow extra keys
styles3.foo satisfies StaticStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StaticStyles<{ width: '100%' | '200%' }>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StaticStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StaticStylesWithout<{ height: unknown }>;
// @ts-expect-error - The style does have `width`
styles3.foo satisfies StaticStylesWithout<{ width: unknown }>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles3.foo satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StyleXStyles<{ width: '100%' | '200%' }>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StyleXStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StyleXStyles<{ width?: unknown }>;
// @ts-expect-error - will be fixed in the next version
styles3.foo satisfies StyleXStylesWithout<{ height: unknown }>;
// @ts-expect-error - The style does have `width`
styles3.foo satisfies StyleXStylesWithout<{ width: unknown }>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles3.foo);

/**
 * CONTEXTUAL STYLES
 */
// @ts-expect-error - will be fixed in the next version
const styles4: Readonly<{
  foo: Readonly<{
    width: '100%' | '100dvw';
  }>;
}> = stylex.create({
  foo: {
    width: {
      default: '100%',
      '@supports (width: 100dvw)': '100dvw',
    },
  },
});
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StaticStyles;
// @ts-expect-error - We want to disallow extra keys
styles4.foo satisfies StaticStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StaticStyles<{ width: '100%' | '100dvw' }>;
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StaticStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles4.foo satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StyleXStyles<{ width: '100%' | '100dvw' }>;
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StyleXStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles4.foo satisfies StyleXStyles<{ width?: unknown }>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles4.foo);

/**
 * NESTED CONTEXTUAL STYLES
 */
// @ts-expect-error - will be fixed in the next version
const styles5: Readonly<{
  foo: Readonly<{
    width: '100%' | '100dvw' | '200%';
  }>;
}> = stylex.create({
  foo: {
    width: {
      default: '100%',
      '@supports (width: 100dvw)': {
        default: '100dvw',
        '@media (max-width: 1000px)': '200%',
      },
    },
  },
});
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StaticStyles;
// @ts-expect-error - We want to disallow extra keys
styles5.foo satisfies StaticStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StaticStyles<{ width: '100%' | '100dvw' | '200%' }>;
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StaticStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles5.foo satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StyleXStyles<{ width: '100%' | '100dvw' | '200%' }>;
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StyleXStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles5.foo satisfies StyleXStyles<{ width?: unknown }>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles5.foo);

/**
 * DYNAMIC NESTED CONTEXTUAL STYLES
 */
// @ts-expect-error - will be fixed in the next version
const styles6: Readonly<{
  foo: (mobile: number) => Readonly<
    [
      Readonly<{
        width: '100%' | '100dvw' | number;
      }>,
      InlineStyles,
    ]
  >;
}> = stylex.create({
  foo: (mobile: number) =>
    ({
      width: {
        default: '100%',
        '@supports (width: 100dvw)': {
          default: '100dvw',
          '@media (max-width: 1000px)': mobile,
        },
      },
    }) as const, // TypeScript limitation
});
// @ts-expect-error - Functions don't return static styles.
styles6.foo(100) satisfies StaticStyles;
// @ts-expect-error - Functions don't return static styles.
styles6.foo(100) satisfies StaticStyles<{}>;
// @ts-expect-error - Functions don't return static styles.
styles6.foo(100) satisfies StaticStyles<{ width: '100%' | '100dvw' | number }>;
// @ts-expect-error - Functions don't return static styles.
styles6.foo(100) satisfies StaticStyles<{ width: number | string }>;
// Functions return StyleXStyles!
// @ts-expect-error - will be fixed in the next version
styles6.foo(100) satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles6.foo(100) satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles6.foo(100) satisfies StyleXStyles<{ width: '100%' | '100dvw' | number }>;
// @ts-expect-error - will be fixed in the next version
styles6.foo(100) satisfies StyleXStyles<{ width: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles6.foo(100) satisfies StyleXStyles<{ width?: unknown }>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles6.foo(100));

/**
 * PSEUDO-ELEMENT STYLES
 */
// @ts-expect-error - will be fixed in the next version
const styles7: Readonly<{
  foo: Readonly<{
    '::before': Readonly<{
      width: '100%';
    }>;
  }>;
}> = stylex.create({
  foo: {
    '::before': { width: '100%' },
  },
});
// @ts-expect-error - will be fixed in the next version
styles7.foo satisfies StaticStyles;
// @ts-expect-error - We want to disallow extra keys
styles7.foo satisfies StaticStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles7.foo satisfies StaticStyles<{ '::before': { width: '100%' } }>;
// @ts-expect-error - will be fixed in the next version
styles7.foo satisfies StaticStyles<{
  '::before': { width: number | string; height?: unknown };
}>;
// @ts-expect-error - will be fixed in the next version
styles7.foo satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles7.foo satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles7.foo satisfies StyleXStyles<{ '::before': { width: '100%' } }>;
// @ts-expect-error - will be fixed in the next version
styles7.foo satisfies StyleXStyles<{
  '::before': { width: number | string; height?: unknown };
}>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles7.foo);

// CSS variables

const vars = stylex.defineVars({
  accent: 'red' as const,
});

// @ts-expect-error - will be fixed in the next version
const styles8: Readonly<{
  foo: Readonly<{
    color: 'red';
  }>;
}> = stylex.create({
  foo: {
    // In a real example `vars` would be imported from another file.
    // eslint-disable-next-line @stylexjs/valid-styles
    color: vars.accent,
  },
});

// @ts-expect-error - will be fixed in the next version
vars.accent satisfies 'red';

// @ts-expect-error - We want to disallow extra keys
vars.accent satisfies 'blue';

// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StaticStyles;
// @ts-expect-error - We want to disallow extra keys
styles8.foo satisfies StaticStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StaticStyles<{ color: 'red' }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StaticStyles<{ color: unknown }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StaticStylesWithout<{ height: unknown }>;
// @ts-expect-error - The style does have `width`
styles8.foo satisfies StaticStylesWithout<{ color: unknown }>;
// @ts-expect-error - 'number' is not assignable to 'red'.
styles8.foo satisfies StaticStyles<{ color: 100 }>;
// @ts-expect-error - 'blue' is not assignable to 'red'.
styles8.foo satisfies StaticStyles<{ color: 'blue' }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StaticStyles<{ color: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StaticStyles<{ color?: unknown; height?: string }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StyleXStyles;
// @ts-expect-error - We want to disallow extra keys
styles8.foo satisfies StyleXStyles<{}>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StyleXStyles<{ color: 'red' }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StyleXStyles<{ color: number | string }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StyleXStyles<{ color?: unknown }>;
// @ts-expect-error - will be fixed in the next version
styles8.foo satisfies StyleXStylesWithout<{ height: unknown }>;
// @ts-expect-error - The style does have `color`
styles8.foo satisfies StyleXStylesWithout<{ color: unknown }>;

// @ts-expect-error - will be fixed in the next version
stylex.props(styles8.foo);
/* eslint-enable no-unused-vars, @typescript-eslint/no-empty-object-type, @typescript-eslint/no-unused-expressions */
