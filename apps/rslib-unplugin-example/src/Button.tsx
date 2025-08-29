import * as stylex from '@stylexjs/stylex';

interface ButtonProps {
  primary?: boolean;
  backgroundColor?: string;
  size?: 'small' | 'medium' | 'large';
  label: string;
  onClick?: () => void;
}

const styles = stylex.create({
  base: {
    fontWeight: 700,
    borderWidth: 0,
    borderStyle: 'none',
    borderColor: 'transparent',
    borderRadius: '3em',
    cursor: 'pointer',
    display: 'inline-block',
    lineHeight: 1,
  },
  primary: {
    color: 'white',
    backgroundColor: '#1ea7fd',
  },
  secondary: {
    color: '#333',
    backgroundColor: 'transparent',
    boxShadow: 'rgba(0, 0, 0, 0.15) 0px 0px 0px 1px inset',
  },
  small: {
    fontSize: '12px',
    padding: '10px 16px',
  },
  medium: {
    fontSize: '14px',
    padding: '11px 20px',
  },
  large: {
    fontSize: '16px',
    padding: '12px 24px',
  },
});

export const Button = ({
  primary = false,
  size = 'medium',
  backgroundColor,
  label,
  ...props
}: ButtonProps) => {
  return (
    <button
      type="button"
      {...stylex.props(styles.base, styles[size], primary ? styles.primary : styles.secondary)}
      style={backgroundColor ? { backgroundColor } : undefined}
      {...props}
    >
      {label}
    </button>
  );
};
