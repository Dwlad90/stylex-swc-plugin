//__stylex_metadata_start__[{"class_name":"color-x1e2nbdu","style":{"rtl":null,"ltr":".color-x1e2nbdu{color:red}"},"priority":3000},{"class_name":"borderColor-x1118g2m","style":{"rtl":null,"ltr":".borderColor-x1118g2m{border-color:blue}"},"priority":2000},{"class_name":"borderColor-x15hxx75","style":{"rtl":null,"ltr":".borderColor-x15hxx75{border-color:pink}"},"priority":2000},{"class_name":"padding-x7z7khe","style":{"rtl":null,"ltr":".padding-x7z7khe{padding:10px}"},"priority":1000},{"class_name":"marginLeft-x16ydxro","style":{"rtl":null,"ltr":".marginLeft-x16ydxro{margin-left:10px}"},"priority":4000}]__stylex_metadata_end__
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
        className: "Page__c.base color-x1e2nbdu Page__c.test borderColor-x15hxx75 padding-x7z7khe"
    };
    return <article className={className} style={style}>Card</article>;
}
