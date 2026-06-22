import * as _stylex2 from "@stylexjs/stylex";
function Foo(props) {
    const stylex = props.stylex;
    const _stylex = props.theme;
    const x = props.x;
    return <svg {..._stylex2.props(x)}/>;
}
