import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
export function MyComponent() {
    return <>
            <div className={"x1e2nbdu"}/>
            <div className={"x1t391ir"}/>
            <CustomComponent xstyle={styles.foo}/>
            <div className={"x1e2nbdu x1t391ir"}/>
          </>;
}
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x1t391ir{background-color:blue}",
    priority: 3000
});
const styles = {
    foo: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
