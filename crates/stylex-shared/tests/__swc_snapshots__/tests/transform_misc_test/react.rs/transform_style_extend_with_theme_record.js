import { BUTTON_PRIMARY, BUTTON_SECONDARY } from 'styles/themes/button.stylex';
import * as stylex from '@stylexjs/stylex';
const buttonTheme = {
    primary: BUTTON_PRIMARY,
    secondary: BUTTON_SECONDARY
};
export function Button_Record_From_Import() {
    return <button {...stylex.props(buttonTheme[state.theme])}>
        Click Me!
      </button>;
}
