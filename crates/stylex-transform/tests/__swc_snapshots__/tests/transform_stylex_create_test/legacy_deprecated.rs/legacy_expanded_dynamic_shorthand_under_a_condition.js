import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
const _temp = {
    koQZXg: "",
    km5ZXQ: "",
    $$css: true
};
_inject2({
    ltr: ".x1vht7gn{margin-inline-start:var(--x-txj11k)}",
    priority: 3000
});
_inject2({
    ltr: ".x1y42md2:hover{margin-inline-start:var(--x-e0sjnx)}",
    priority: 3130
});
_inject2({
    ltr: ".x69mnlg{margin-inline-end:var(--x-txj11k)}",
    priority: 3000
});
_inject2({
    ltr: ".xlcbw0d:hover{margin-inline-end:var(--x-e0sjnx)}",
    priority: 3130
});
_inject2({
    ltr: '@property --x-txj11k { syntax: "*"; inherits: false;}',
    priority: 0
});
_inject2({
    ltr: '@property --x-e0sjnx { syntax: "*"; inherits: false;}',
    priority: 0
});
export const styles = {
    inline: (space)=>[
            _temp,
            {
                keTefX: (space != null ? "x1vht7gn " : space) + (space != null ? "x1y42md2" : space),
                k71WvV: (space != null ? "x69mnlg " : space) + (space != null ? "xlcbw0d" : space),
                $$css: true
            },
            {
                "--x-txj11k": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(space),
                "--x-e0sjnx": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(space)
            }
        ]
};
