import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  root: {
    backgroundColor: 'purple',
    borderColor: 'orange',
    borderStyle: 'solid',
    borderWidth: 10,
    boxSizing: 'border-box',
    display: 'flex',
    marginBlockEnd: 16,
    marginBlockStart: 16,
    marginInline: 16,
    paddingBlock: 32,
    paddingInlineEnd: 32,
    paddingInlineStart: 32,
    verticalAlign: 'top',
    textDecorationLine: 'underline',
  },
});
