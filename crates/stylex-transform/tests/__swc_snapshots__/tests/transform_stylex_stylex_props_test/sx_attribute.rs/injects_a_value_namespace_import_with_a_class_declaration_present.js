import * as stylex from "@stylexjs/stylex";
class Bar {
}
function Foo(props) {
    const x = props.x;
    return <svg {...stylex.props(x)}/>;
}
