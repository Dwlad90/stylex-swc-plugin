import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import style from "@stylexjs/stylex";
_inject2(".x1e2nbdu{color:red}", 3000);
_inject2(".x1118g2m{border-color:blue}", 2000);
_inject2(".x15hxx75{border-color:pink}", 2000);
_inject2(".x7z7khe{padding:10px}", 1000);
_inject2(".x16ydxro{margin-left:10px}", 4000);
export default function Card() {
    const { className, style } = {
        className: "Page__c.base x1e2nbdu Page__c.test x15hxx75 x7z7khe"
    };
    return <article className={className} style={style}>Card</article>;
}
