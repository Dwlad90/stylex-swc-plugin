import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import style from "@stylexjs/stylex";
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".borderColor-x1118g2m{border-color:blue}",
    priority: 2000
});
_inject2({
    ltr: ".borderColor-x15hxx75{border-color:pink}",
    priority: 2000
});
_inject2({
    ltr: ".padding-x7z7khe{padding:10px}",
    priority: 1000
});
_inject2({
    ltr: ".marginLeft-x16ydxro{margin-left:10px}",
    priority: 4000
});
export default function Card() {
    const { className, style } = {
        className: "input__c.base color-x1e2nbdu input__c.test borderColor-x15hxx75 padding-x7z7khe",
        "data-style-src": "tests/fixture/card/input.stylex.js:4; tests/fixture/card/input.stylex.js:8"
    };
    return <article className={className} style={style}>Card</article>;
}
