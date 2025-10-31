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
({
    0: {
        className: "x1e2nbdu"
    },
    1: {}
})[!!(Math.random() > 1) << 0];
({
    0: {
        className: "x1e2nbdu"
    },
    1: {}
})[!!true << 0];
({
    0: {
        className: "x1e2nbdu"
    },
    1: {}
})[!!false << 0];
({
    0: {
        className: "x1e2nbdu"
    },
    1: {}
})[!!false << 0];
({
    0: {
        className: "x1e2nbdu"
    },
    1: {}
})[!!true << 0];
export function TestComponent({ removeStyle, isAnimation }) {
    stylex.props(Math.random() > 1 ? styles.red : undefined);
    stylex.props(true ? styles.red : undefined);
    stylex.props(false ? styles.red : undefined);
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!(Math.random() > 1) << 0];
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!true << 0];
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!false << 0];
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!false << 0];
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!true << 0];
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!removeStyle << 0];
    ({
        0: {
            className: "x1e2nbdu"
        },
        1: {}
    })[!!removeStyle << 0];
    const { className: classNameDiv2, style: styleDiv2 } = sx.props(removeStyle ? null : c.red, isAnimation && c.red);
    return <div className={classNameDiv2} style={styleDiv2}/>;
}
