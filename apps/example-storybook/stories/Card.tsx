import { FC } from 'react';
import * as stylex from '@stylexjs/stylex';

export const cardStyles = stylex.create({
  base: {
    // @ts-expect-error - env is not correctly typed
    backgroundColor: stylex.env.wrapper(stylex.env.tokens.colors.background),
    borderRadius: 8,
    // @ts-expect-error - env is not correctly typed
    padding: stylex.env.wrapper(16),
    boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)',
    borderWidth: 1,
    borderStyle: 'solid',
    borderColor: '#e0e0e0',
    maxWidth: 300,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    // @ts-expect-error - env is not correctly typed
    marginBottom: stylex.env.wrapper(8),
    // @ts-expect-error - env is not correctly typed
    color: stylex.env.wrapper(stylex.env.tokens.colors.text),
  },
  content: {
    fontSize: 14,
    lineHeight: 1.5,
    color: '#666',
  },
  elevated: {
    boxShadow: '0 4px 8px rgba(0, 0, 0, 0.15)',
  },
});

type CardProps = {
  title: string;
  content: string;
  elevated?: boolean;
};

export const Card: FC<CardProps> = ({ title, content, elevated = false }) => {
  return (
    // @ts-expect-error - sx is not correctly typed
    <div sx={[cardStyles.base, elevated && cardStyles.elevated]}>
      <h3 {...stylex.props(cardStyles.title)}>{title}</h3>
      <p {...stylex.props(cardStyles.content)}>{content}</p>
    </div>
  );
};
