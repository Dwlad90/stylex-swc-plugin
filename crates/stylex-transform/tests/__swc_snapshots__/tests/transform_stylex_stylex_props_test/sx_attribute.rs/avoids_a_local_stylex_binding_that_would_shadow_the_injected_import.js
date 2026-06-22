import * as _stylex from "@stylexjs/stylex";
function Foo(props) {
    const stylex = props.stylex;
    const x = props.x;
    return <svg {..._stylex.props(x)}/>;
}
