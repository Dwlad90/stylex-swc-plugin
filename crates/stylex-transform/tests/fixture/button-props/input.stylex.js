// Button.tsx

import { BUTTON_PRIMARY, BUTTON_SECONDARY } from './button.stylex.ts'

const buttonTheme = {
  primary: BUTTON_PRIMARY,
  secondary: BUTTON_SECONDARY,
}

const styles = stylex.create({
  bg: { backgroundColor: 'red' },
});

export const Button = (props) => {
  // this doesn't work
  return (
    <button
      {...stylex.props(
        buttonTheme[props.theme],
      )}
    >
      {children}
    </button>
  );
}

export const Button2 = (props) => {
  // this works
  return (
    <button
      {...stylex.props(
        styles.bg,
        buttonTheme[props.theme],
      )}
    >
      {children}
    </button>
  );
}

export const Button3 = (props) => {
  // this works
  return (
    <button
      {...stylex.props(
        undefined,
        buttonTheme[props.theme],
      )}
    >
      {children}
    </button>
  );
}

const B1 = stylex.create({
  Regular: {
    fontWeight: 400,
    fontSize: 20,
    lineHeight: '24px',
    letterSpacing: '-0.01em',
  },
  Bold: {
    fontWeight: 700,
    fontSize: 20,
    lineHeight: '24px',
    letterSpacing: '-0.01em',
  },
});

const B2 = stylex.create({
  Regular: {
    fontWeight: 400,
    fontSize: 18,
    lineHeight: '22px',
    letterSpacing: '-0.01em',
  },
  Bold: {
    fontWeight: 700,
    fontSize: 18,
    lineHeight: '22px',
    letterSpacing: '-0.01em',
  },
});

const typography = {
  xlarge: B1,
  large: B2,
};

const Button = (props) => {
  return (
    <button
      {...stylex.props(
        typography[props.size],
      )}
    >
      {children}
    </button>
  );

}
