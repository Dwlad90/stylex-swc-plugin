import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from 'stylex';
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    red: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
stylex.props(Math.random() > 1 ? styles.red : undefined);
stylex.props(true ? styles.red : undefined);
stylex.props(false ? styles.red : undefined);
stylex.props(Math.random() > 1 ? undefined : styles.red);
stylex.props(true ? undefined : styles.red);
stylex.props(false ? undefined : styles.red);
stylex.props(false ? null : styles.red);
stylex.props(true ? null : styles.red);
export function TestComponent({ removeStyle, isAnimation }) {
    stylex.props(Math.random() > 1 ? styles.red : undefined);
    stylex.props(true ? styles.red : undefined);
    stylex.props(false ? styles.red : undefined);
    stylex.props(Math.random() > 1 ? undefined : styles.red);
    stylex.props(true ? undefined : styles.red);
    stylex.props(false ? undefined : styles.red);
    stylex.props(false ? null : styles.red);
    stylex.props(true ? null : styles.red);
    stylex.props(removeStyle ? undefined : styles.red);
    stylex.props(removeStyle ? null : styles.red);
    const { className: classNameDiv2, style: styleDiv2 } = sx.props(removeStyle ? null : c.red, isAnimation && c.red);
    return <div className={classNameDiv2} style={styleDiv2}/>;
}
