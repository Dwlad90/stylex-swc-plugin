import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import s from "@stylexjs/stylex";
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".x1118g2m{border-color:blue}",
    priority: 2000
});
_inject2({
    ltr: ".x15hxx75{border-color:pink}",
    priority: 2000
});
_inject2({
    ltr: ".x7z7khe{padding:10px}",
    priority: 1000
});
_inject2({
    ltr: ".x16ydxro{margin-left:10px}",
    priority: 4000
});
export default function Home() {
    const { className, style } = {
        className: "x1e2nbdu x15hxx75 x7z7khe"
    };
    return <main className={className} style={style}>
          Main
        </main>;
}
