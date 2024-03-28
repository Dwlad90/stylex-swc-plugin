import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
function MyComponent() {
    return <>

                    <div {...{
        class: "x1e2nbdu"
    }}/>

                    <div {...{
        class: "x1t391ir"
    }}/>

                    <CustomComponent xstyle={styles.foo}/>

                    <div {...{
        class: "x1e2nbdu x1t391ir"
    }}/>

                </>;
}
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x1t391ir{background-color:blue}", 3000);
const styles = {
    foo: {
        color: "x1e2nbdu",
        $$css: true
    }
};
