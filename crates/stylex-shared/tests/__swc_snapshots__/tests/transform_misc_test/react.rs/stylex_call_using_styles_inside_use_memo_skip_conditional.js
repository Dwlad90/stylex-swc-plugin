import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
import { useMemo } from 'react';
_inject2(".x1e2nbdu{color:red}", 3000);
const styles = {
    selected: {
        kMwMTN: "x1e2nbdu",
        $$css: true
    }
};
export default function MyComponent() {
    const isSelected = true;
    const innerComponent = useMemo(()=>{
        return <Component {...stylex.props(isSelected && styles.selected)}/>;
    }, [
        isSelected
    ]);
    return innerComponent;
}
