import * as _stylex from "custom-stylex-path";
import type { StyleXStyles } from 'custom-stylex-path';
function Foo(props) {
    const x = props.x;
    return <svg {..._stylex.props(x)}/>;
}
