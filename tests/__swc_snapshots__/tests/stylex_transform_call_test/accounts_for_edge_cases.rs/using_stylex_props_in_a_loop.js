import stylex from '@stylexjs/stylex';
function test(colors, obj) {
    for (const color of colors) {
        obj[color.key] = stylex.props(color.style);
    }
}