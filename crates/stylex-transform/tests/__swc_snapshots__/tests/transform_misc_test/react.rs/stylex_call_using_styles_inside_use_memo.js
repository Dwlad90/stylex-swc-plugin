import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
import { useMemo } from 'react';
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
export default function MyComponent() {
    const isSelected = true;
    const innerComponent = useMemo(()=>{
        return <Component {...{
            0: {},
            1: {
                className: "x1e2nbdu"
            }
        }[!!isSelected << 0]}/>;
    }, [
        isSelected
    ]);
    return innerComponent;
}
