import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2(".x14odnwx{padding:5px}", 1000);
_inject2(".xp59q4u{padding-block:10px}", 2000);
_inject2(".xm7lytj{padding-top:7px}", 4000);
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
