import * as stylex from '@stylexjs/stylex';
function Bar() {
    const stylex = 1;
    return stylex;
}
function Foo(props) {
    const x = props.x;
    return <svg {...stylex.props(x)}/>;
}
