import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".backgroundColor-x1t391ir{background-color:blue}",
    priority: 3000
});
function Foo() {
    return <>
      <div id="test" className="color-x1e2nbdu" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:3">Hello World</div>
      <div id="test" className="color-x1e2nbdu" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:3">Hello World</div>
      <div className="test" {...{
        0: {
            className: "color-x1e2nbdu",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
        },
        1: {
            className: "color-x1e2nbdu backgroundColor-x1t391ir",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3; npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
        }
    }[!!color << 0]} id="test">Hello World</div>
      <div className="test" {...{
        0: {
            className: "color-x1e2nbdu",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
        },
        1: {
            className: "color-x1e2nbdu backgroundColor-x1t391ir",
            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3; npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
        }
    }[!!color << 0]} id="test">Hello World</div>
      <div id="test" className="backgroundColor-x1t391ir" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:6" className="test">Hello World</div>
      <div id="test" className="backgroundColor-x1t391ir" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:6" className="test">Hello World</div>
    </>;
}
