import _inject from "@stylexjs/stylex/lib/stylex-inject";
var _inject2 = _inject;
import stylex from 'stylex';
_inject2({
    ltr: ".color-x1e2nbdu{color:red}",
    priority: 3000
});
_inject2({
    ltr: ".borderRadius-x12oqio5{border-radius:4px}",
    priority: 2000
});
_inject2({
    ltr: ".backgroundColor-x1t391ir{background-color:blue}",
    priority: 3000
});
_inject2({
    ltr: ".display-x78zum5{display:flex}",
    priority: 3000
});
const styles = {
    a: {
        "Foo__styles.a": "Foo__styles.a",
        "color-kMwMTN": "color-x1e2nbdu",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
    },
    b: {
        "Foo__styles.b": "Foo__styles.b",
        "borderRadius-kaIpWk": "borderRadius-x12oqio5",
        $$css: "npm-package:node_modules/npm-package/dist/components/Foo.react.js:6"
    }
};
function App({ sx, rest }) {
    return _jsx("section", {
        ...stylex.props(sx),
        id: "outer",
        children: _jsxs("div", {
            ...{
                className: "Foo__styles.a color-x1e2nbdu",
                "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:3"
            },
            className: "middle",
            children: [
                _jsx("span", {
                    ...{
                        className: "Foo__styles.b borderRadius-x12oqio5 Foo__styles.c backgroundColor-x1t391ir",
                        "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:6; npm-package:node_modules/npm-package/dist/components/Foo.react.js:9"
                    },
                    children: _jsx("em", {
                        ...rest,
                        ...{
                            className: "Foo__styles.d display-x78zum5",
                            "data-style-src": "npm-package:node_modules/npm-package/dist/components/Foo.react.js:12"
                        },
                        children: _jsx(MyComponent, {
                            sx: styles.a
                        })
                    })
                }),
                _jsx("p", {
                    get sx () {
                        return styles.b;
                    }
                })
            ]
        })
    });
}
