import * as stylex from "custom-stylex-a";
function Foo(props) {
    const x = props.x;
    return <svg {...stylex.props(x)}/>;
}
