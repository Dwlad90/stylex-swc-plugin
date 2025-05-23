import * as stylex from '@stylexjs/stylex';
export const styles = {
    root: (bgColor, otherColor)=>[
            {
                "--background-color": bgColor == null ? null : "x15mgraa",
                "--otherColor": otherColor == null ? null : "x1qph05k",
                $$css: true
            },
            {
                "----background-color": bgColor != null ? bgColor : undefined,
                "----otherColor": otherColor != null ? otherColor : undefined
            }
        ]
};
