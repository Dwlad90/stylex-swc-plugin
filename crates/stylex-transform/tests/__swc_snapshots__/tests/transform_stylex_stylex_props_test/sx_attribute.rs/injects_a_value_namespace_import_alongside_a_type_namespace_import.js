import * as _stylex from "@stylexjs/stylex";
import type * as stylex from '@stylexjs/stylex';
function Foo(props) {
    const x = props.x;
    return <svg {..._stylex.props(x)}/>;
}
