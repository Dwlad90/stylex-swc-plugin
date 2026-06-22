import * as stylex from '@stylexjs/stylex';
function Foo(props) {
    try {
        props.run();
    } catch (stylex) {
        props.onError(stylex);
    }
    return <svg {...stylex.props(props.x)}/>;
}
