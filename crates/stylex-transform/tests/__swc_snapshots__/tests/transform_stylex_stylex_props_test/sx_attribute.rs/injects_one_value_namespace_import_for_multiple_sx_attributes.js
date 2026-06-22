import * as stylex from "@stylexjs/stylex";
function Foo(props) {
    const x = props.x;
    return <>
          <svg {...stylex.props(x)}/>
          <svg {...stylex.props(props.y)}/>
        </>;
}
