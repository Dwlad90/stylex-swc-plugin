"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Rgba = exports.Rgb = exports.Oklch = exports.Oklab = exports.NamedColor = exports.Lch = exports.Hsla = exports.Hsl = exports.HashColor = exports.Color = void 0;
var _core = require("../core2");
var _alphaValue = require("./alpha-value");
var _angle = require("./angle");
var _commonTypes = require("./common-types");
class Color {
  static get parser() {
    return _core.TokenParser.oneOf(NamedColor.parser, HashColor.parser, Rgb.parser, Rgba.parser, Hsl.parser, Hsla.parser, Lch.parser, Oklch.parser, Oklab.parser);
  }
}
exports.Color = Color;
class NamedColor extends Color {
  constructor(value) {
    super();
    this.value = value;
  }
  toString() {
    return this.value;
  }
  static parser = (() => _core.TokenParser.tokens.Ident.map(token => token[4].value).where(str => ['aliceblue', 'antiquewhite', 'aqua', 'aquamarine', 'azure', 'beige', 'bisque', 'black', 'blanchedalmond', 'blue', 'blueviolet', 'brown', 'burlywood', 'cadetblue', 'chartreuse', 'chocolate', 'coral', 'cornflowerblue', 'cornsilk', 'crimson', 'cyan', 'darkblue', 'darkcyan', 'darkgoldenrod', 'darkgray', 'darkgreen', 'darkgrey', 'darkkhaki', 'darkmagenta', 'darkolivegreen', 'darkorange', 'darkorchid', 'darkred', 'darksalmon', 'darkseagreen', 'darkslateblue', 'darkslategray', 'darkslategrey', 'darkturquoise', 'darkviolet', 'deeppink', 'deepskyblue', 'dimgray', 'dimgrey', 'dodgerblue', 'firebrick', 'floralwhite', 'forestgreen', 'fuchsia', 'gainsboro', 'ghostwhite', 'gold', 'goldenrod', 'gray', 'green', 'greenyellow', 'grey', 'honeydew', 'hotpink', 'indianred', 'indigo', 'ivory', 'khaki', 'lavender', 'lavenderblush', 'lawngreen', 'lemonchiffon', 'lightblue', 'lightcoral', 'lightcyan', 'lightgoldenrodyellow', 'lightgray', 'lightgreen', 'lightgrey', 'lightpink', 'lightsalmon', 'lightseagreen', 'lightskyblue', 'lightslategray', 'lightslategrey', 'lightsteelblue', 'lightyellow', 'lime', 'limegreen', 'linen', 'magenta', 'maroon', 'mediumaquamarine', 'mediumblue', 'mediumorchid', 'mediumpurple', 'mediumseagreen', 'mediumslateblue', 'mediumspringgreen', 'mediumturquoise', 'mediumvioletred', 'midnightblue', 'mintcream', 'mistyrose', 'moccasin', 'navajowhite', 'navy', 'oldlace', 'olive', 'olivedrab', 'orange', 'orangered', 'orchid', 'palegoldenrod', 'palegreen', 'paleturquoise', 'palevioletred', 'papayawhip', 'peachpuff', 'peru', 'pink', 'plum', 'powderblue', 'purple', 'rebeccapurple', 'red', 'rosybrown', 'royalblue', 'saddlebrown', 'salmon', 'sandybrown', 'seagreen', 'seashell', 'sienna', 'silver', 'skyblue', 'slateblue', 'slategray', 'slategrey', 'snow', 'springgreen', 'steelblue', 'tan', 'teal', 'thistle', 'tomato', 'transparent', 'turquoise', 'violet', 'wheat', 'white', 'whitesmoke', 'yellow', 'yellowgreen'].includes(str)).map(value => new NamedColor(value)))();
}
exports.NamedColor = NamedColor;
class HashColor extends Color {
  constructor(value) {
    super();
    this.value = value;
  }
  toString() {
    return `#${this.value}`;
  }
  get r() {
    return parseInt(this.value.slice(0, 2), 16);
  }
  get g() {
    return parseInt(this.value.slice(2, 4), 16);
  }
  get b() {
    return parseInt(this.value.slice(4, 6), 16);
  }
  get a() {
    return this.value.length === 8 ? parseInt(this.value.slice(6, 8), 16) / 255 : 1;
  }
  static get parser() {
    return _core.TokenParser.tokens.Hash.map(token => token[4].value).where(value => [3, 6, 8].includes(value.length) && /^[0-9a-fA-F]+$/.test(value)).map(value => new HashColor(value));
  }
}
exports.HashColor = HashColor;
const rgbNumberParser = _core.TokenParser.tokens.Number.map(token => token[4].value).where(value => value >= 0 && value <= 255);
const alphaAsNumber = _alphaValue.AlphaValue.parser.map(alpha => alpha.value);
const slashParser = _core.TokenParser.tokens.Delim.map(token => token[4].value).where(value => value === '/').surroundedBy(_core.TokenParser.tokens.Whitespace);
class Rgb extends Color {
  constructor(r, g, b) {
    super();
    this.r = r;
    this.g = g;
    this.b = b;
  }
  toString() {
    return `rgb(${this.r},${this.g},${this.b})`;
  }
  static get parser() {
    const rgbCommaSeparated = _core.TokenParser.sequence(rgbNumberParser, rgbNumberParser, rgbNumberParser).separatedBy(_core.TokenParser.tokens.Comma).separatedBy(_core.TokenParser.tokens.Whitespace.optional);
    const commaParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgb'), rgbCommaSeparated, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref => {
      let [_fn, [r, g, b], _closeParen] = _ref;
      return new Rgb(r, g, b);
    });
    const spaceSeparatedRGB = _core.TokenParser.sequence(rgbNumberParser, rgbNumberParser, rgbNumberParser).separatedBy(_core.TokenParser.tokens.Whitespace).surroundedBy(_core.TokenParser.tokens.Whitespace.optional);
    const spaceParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgb'), spaceSeparatedRGB, _core.TokenParser.tokens.CloseParen).map(_ref2 => {
      let [_fn, [r, g, b], _closeParen] = _ref2;
      return new Rgb(r, g, b);
    });
    return _core.TokenParser.oneOf(commaParser, spaceParser);
  }
}
exports.Rgb = Rgb;
class Rgba extends Color {
  constructor(r, g, b, a) {
    super();
    this.r = r;
    this.g = g;
    this.b = b;
    this.a = a;
  }
  toString() {
    return `rgba(${this.r},${this.g},${this.b},${this.a})`;
  }
  static get parser() {
    const commaParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgba'), rgbNumberParser, _core.TokenParser.tokens.Comma, rgbNumberParser, _core.TokenParser.tokens.Comma, rgbNumberParser, _core.TokenParser.tokens.Comma, alphaAsNumber, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref3 => {
      let [_fn, r, _comma, g, _comma2, b, _comma3, a, _closeParen] = _ref3;
      return new Rgba(r, g, b, a);
    });
    const spaceParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgb'), _core.TokenParser.tokens.Whitespace.optional, rgbNumberParser, _core.TokenParser.tokens.Whitespace, rgbNumberParser, _core.TokenParser.tokens.Whitespace, rgbNumberParser, slashParser, alphaAsNumber, _core.TokenParser.tokens.Whitespace.optional, _core.TokenParser.tokens.CloseParen).map(_ref4 => {
      let [_fn, _preSpace, r, _space, g, _space2, b, _slash, a, _postSpace, _closeParen] = _ref4;
      return new Rgba(r, g, b, a);
    });
    return _core.TokenParser.oneOf(commaParser, spaceParser);
  }
}
exports.Rgba = Rgba;
class Hsl extends Color {
  constructor(h, s, l) {
    super();
    this.h = h;
    this.s = s;
    this.l = l;
  }
  toString() {
    return `hsl(${this.h.toString()},${this.s.toString()},${this.l.toString()})`;
  }
  static get parser() {
    const commaParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsl'), _angle.Angle.parser, _core.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _core.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(tokens => new Hsl(tokens[1], tokens[3], tokens[5]));
    const spaceParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsl'), _angle.Angle.parser, _core.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, _core.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, _core.TokenParser.tokens.Whitespace, _core.TokenParser.tokens.CloseParen).map(tokens => new Hsl(tokens[1], tokens[3], tokens[5]));
    return _core.TokenParser.oneOf(commaParser, spaceParser);
  }
}
exports.Hsl = Hsl;
class Hsla extends Color {
  constructor(h, s, l, a) {
    super();
    this.h = h;
    this.s = s;
    this.l = l;
    this.a = a;
  }
  toString() {
    return `hsla(${this.h.toString()},${this.s.toString()},${this.l.toString()},${this.a})`;
  }
  static get parser() {
    const commaParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsla'), _angle.Angle.parser, _core.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _core.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _core.TokenParser.tokens.Comma, alphaAsNumber, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref5 => {
      let [_fn, h, _comma, s, _comma2, l, _comma3, a, _closeParen] = _ref5;
      return new Hsla(h, s, l, a);
    });
    const spaceParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsl'), _angle.Angle.parser, _core.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, _core.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, slashParser, alphaAsNumber, _core.TokenParser.tokens.Whitespace.optional, _core.TokenParser.tokens.CloseParen).map(_ref6 => {
      let [_fn, h, _space, s, _space2, l, _slash, a, _postSpace, _closeParen] = _ref6;
      return new Hsla(h, s, l, a);
    });
    return _core.TokenParser.oneOf(commaParser, spaceParser);
  }
}
exports.Hsla = Hsla;
class Lch extends Color {
  constructor(l, c, h, alpha) {
    super();
    this.l = l;
    this.c = c;
    this.h = h;
    this.alpha = alpha;
  }
  toString() {
    return `lch(${this.l} ${this.c} ${this.h.toString()}${this.alpha ? ` / ${this.alpha}` : ''})`;
  }
  static get parser() {
    const l = _core.TokenParser.oneOf(_commonTypes.Percentage.parser.map(p => p.value), _core.TokenParser.tokens.Number.map(token => token[4].value).where(value => value >= 0), _core.TokenParser.tokens.Ident.map(token => token[4].value).where(value => value === 'none').map(() => 0));
    const c = _core.TokenParser.oneOf(_commonTypes.Percentage.parser.map(p => 150 * p.value / 100), _core.TokenParser.tokens.Number.map(token => token[4].value).where(value => value >= 0));
    const h = _core.TokenParser.oneOf(_angle.Angle.parser, _core.TokenParser.tokens.Number.map(token => token[4].value));
    const a = _core.TokenParser.sequence(slashParser, alphaAsNumber).separatedBy(_core.TokenParser.tokens.Whitespace).map(_ref7 => {
      let [_, a] = _ref7;
      return a;
    });
    return _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'lch'), _core.TokenParser.sequence(l, c, h).separatedBy(_core.TokenParser.tokens.Whitespace), a.suffix(_core.TokenParser.tokens.Whitespace.optional).optional, _core.TokenParser.tokens.CloseParen).map(_ref8 => {
      let [_fn, [l, c, h], a, _closeParen] = _ref8;
      return new Lch(l, c, h, a);
    });
  }
}
exports.Lch = Lch;
class Oklch extends Color {
  constructor(l, c, h, alpha) {
    super();
    this.l = l;
    this.c = c;
    this.h = h;
    this.alpha = alpha;
  }
  toString() {
    return `oklch(${this.l} ${this.c} ${this.h.toString()}${this.alpha ? ` / ${this.alpha}` : ''})`;
  }
  static get parser() {
    const lc = _core.TokenParser.oneOf(alphaAsNumber, _core.TokenParser.tokens.Ident.map(token => token[4].value).where(value => value === 'none').map(() => 0)).prefix(_core.TokenParser.tokens.Whitespace.optional);
    const h = _core.TokenParser.oneOf(_angle.Angle.parser, lc.map(num => new _angle.Angle(num * 360, 'deg')));
    const a = _core.TokenParser.sequence(slashParser, alphaAsNumber).map(_ref9 => {
      let [_, a] = _ref9;
      return a;
    });
    return _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'oklch'), lc, _core.TokenParser.tokens.Whitespace, lc, _core.TokenParser.tokens.Whitespace, h, a.suffix(_core.TokenParser.tokens.Whitespace.optional).optional, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref10 => {
      let [_fn, l, _comma, c, _comma2, h, a, _closeParen] = _ref10;
      return new Lch(l, c, h, a);
    });
  }
}
exports.Oklch = Oklch;
class Oklab extends Color {
  constructor(l, a, b, alpha) {
    super();
    this.l = l;
    this.a = a;
    this.b = b;
    this.alpha = alpha;
  }
  toString() {
    return `oklab(${this.l} ${this.a} ${this.b}${this.alpha ? ` / ${this.alpha}` : ''})`;
  }
  static get parser() {
    const lab = _core.TokenParser.oneOf(alphaAsNumber, _core.TokenParser.tokens.Ident.map(token => token[4].value).where(value => value === 'none').map(() => 0)).prefix(_core.TokenParser.tokens.Whitespace.optional);
    const a = _core.TokenParser.sequence(slashParser, alphaAsNumber).map(_ref11 => {
      let [_, a] = _ref11;
      return a;
    });
    return _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'oklab'), lab, _core.TokenParser.tokens.Whitespace, lab, _core.TokenParser.tokens.Whitespace, lab, a.suffix(_core.TokenParser.tokens.Whitespace.optional).optional, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref12 => {
      let [_fn, l, _comma, a, _comma2, b, alpha, _closeParen] = _ref12;
      return new Oklab(l, a, b, alpha);
    });
  }
}
exports.Oklab = Oklab;