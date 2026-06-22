import * as stylex from "custom-stylex-path";
function Foo(props) {
    const x = props.x;
    return <svg {...stylex.props(x)}/>;
}
