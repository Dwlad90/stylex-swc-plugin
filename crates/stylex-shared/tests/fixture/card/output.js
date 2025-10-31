import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import style from "@stylexjs/stylex";
_inject2(".color-x1e2nbdu{color:red}", 3000);
_inject2(".borderColor-x1118g2m{border-color:blue}", 2000);
_inject2(".borderColor-x15hxx75{border-color:pink}", 2000);
_inject2(".padding-x7z7khe{padding:10px}", 1000);
_inject2(".marginLeft-x16ydxro{margin-left:10px}", 4000);
export default function Card() {
    const { className, style } = {
        className: "color-x1e2nbdu borderColor-x15hxx75 padding-x7z7khe",
        "data-style-src": "input.stylex.js:3; input.stylex.js:7"
    };
    return <article className={className} style={style}>Card</article>;
}
