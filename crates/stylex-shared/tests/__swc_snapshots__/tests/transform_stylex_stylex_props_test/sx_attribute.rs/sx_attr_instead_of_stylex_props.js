import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
function Foo() {
    return <>
      <div id="test" className="color-x1e2nbdu" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:3">Hello World</div>
      <div className="test" className="color-x1e2nbdu" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:3" id="test">Hello World</div>
      <div id="test" className="color-x1e2nbdu" data-style-src="npm-package:node_modules/npm-package/dist/components/Foo.react.js:3" className="test">Hello World</div>
    </>;
}
