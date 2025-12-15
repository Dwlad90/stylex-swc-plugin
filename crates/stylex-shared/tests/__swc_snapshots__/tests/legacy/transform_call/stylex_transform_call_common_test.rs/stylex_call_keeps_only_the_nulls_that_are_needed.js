import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".x14odnwx{padding:5px}",
    priority: 1000
});
_inject2({
    ltr: ".xp59q4u{padding-block:10px}",
    priority: 2000
});
_inject2({
    ltr: ".xm7lytj{padding-top:7px}",
    priority: 4000
});
const styles = {
    foo: {
        kmVPX3: "x14odnwx",
        kLKAdn: null,
        $$css: true
    },
    baz: {
        kLKAdn: "xm7lytj",
        $$css: true
    }
};
"x14odnwx";
"x14odnwx xp59q4u";
"x14odnwx";
"x14odnwx xp59q4u xm7lytj";
stylex(styles.baz, styles.foo, somethingElse);
