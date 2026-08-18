import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import * as stylex from "@stylexjs/stylex";
const VIEWS = [
    'grid',
    'list'
];
const SIZES = {
    small: 1
};
_inject2({
    ltr: ".x1mqxbix{color:black}",
    priority: 3000
});
_inject2({
    ltr: ".x1e2nbdu{color:red}",
    priority: 3000
});
// A method on a runtime receiver.
export const Lower = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!q.toLowerCase() << 0]}/>;
// An array method whose receiver folds and whose argument does not.
export const Index = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!(VIEWS.indexOf(q) >= 0) << 0]}/>;
// A call on the result of a call that does fold.
export const Keys = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!Object.keys(SIZES).includes(q) << 0]}/>;
// The inner call folds, the outer one cannot.
export const Chain = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!"documentation".slice(0, 3).startsWith(q) << 0]}/>;
// The call is optional, so the node the evaluator meets is not a plain one.
export const Optional = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!q?.startsWith("a") << 0]}/>;
// A method no fold exists for under any receiver.
export const Unknown = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!q.somethingUnknown() << 0]}/>;
// A receiver the evaluator has no fold for at all, rather than a method it
// does not know on a receiver it does.
export const Constructed = ({ q })=><i {...{
        0: {
            className: "x1mqxbix"
        },
        1: {
            className: "x1e2nbdu"
        }
    }[!!new Set(VIEWS).has(q) << 0]}/>;
