"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Rgba = exports.Rgb = exports.Oklch = exports.Oklab = exports.NamedColor = exports.Lch = exports.Hsla = exports.Hsl = exports.HashColor = exports.Color = void 0;
var _tokenParser = require("../token-parser");
var _alphaValue = require("./alpha-value");
var _angle = require("./angle");
var _commonTypes = require("./common-types");
class Color {
  static get parser() {
    return _tokenParser.TokenParser.oneOf(NamedColor.parser, HashColor.parser, Rgb.parser, Rgba.parser, Hsl.parser, Hsla.parser, Lch.parser, Oklch.parser, Oklab.parser);
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
  static parser = (() => _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value).where(str => ['aliceblue', 'antiquewhite', 'aqua', 'aquamarine', 'azure', 'beige', 'bisque', 'black', 'blanchedalmond', 'blue', 'blueviolet', 'brown', 'burlywood', 'cadetblue', 'chartreuse', 'chocolate', 'coral', 'cornflowerblue', 'cornsilk', 'crimson', 'cyan', 'darkblue', 'darkcyan', 'darkgoldenrod', 'darkgray', 'darkgreen', 'darkgrey', 'darkkhaki', 'darkmagenta', 'darkolivegreen', 'darkorange', 'darkorchid', 'darkred', 'darksalmon', 'darkseagreen', 'darkslateblue', 'darkslategray', 'darkslategrey', 'darkturquoise', 'darkviolet', 'deeppink', 'deepskyblue', 'dimgray', 'dimgrey', 'dodgerblue', 'firebrick', 'floralwhite', 'forestgreen', 'fuchsia', 'gainsboro', 'ghostwhite', 'gold', 'goldenrod', 'gray', 'green', 'greenyellow', 'grey', 'honeydew', 'hotpink', 'indianred', 'indigo', 'ivory', 'khaki', 'lavender', 'lavenderblush', 'lawngreen', 'lemonchiffon', 'lightblue', 'lightcoral', 'lightcyan', 'lightgoldenrodyellow', 'lightgray', 'lightgreen', 'lightgrey', 'lightpink', 'lightsalmon', 'lightseagreen', 'lightskyblue', 'lightslategray', 'lightslategrey', 'lightsteelblue', 'lightyellow', 'lime', 'limegreen', 'linen', 'magenta', 'maroon', 'mediumaquamarine', 'mediumblue', 'mediumorchid', 'mediumpurple', 'mediumseagreen', 'mediumslateblue', 'mediumspringgreen', 'mediumturquoise', 'mediumvioletred', 'midnightblue', 'mintcream', 'mistyrose', 'moccasin', 'navajowhite', 'navy', 'oldlace', 'olive', 'olivedrab', 'orange', 'orangered', 'orchid', 'palegoldenrod', 'palegreen', 'paleturquoise', 'palevioletred', 'papayawhip', 'peachpuff', 'peru', 'pink', 'plum', 'powderblue', 'purple', 'rebeccapurple', 'red', 'rosybrown', 'royalblue', 'saddlebrown', 'salmon', 'sandybrown', 'seagreen', 'seashell', 'sienna', 'silver', 'skyblue', 'slateblue', 'slategray', 'slategrey', 'snow', 'springgreen', 'steelblue', 'tan', 'teal', 'thistle', 'tomato', 'transparent', 'turquoise', 'violet', 'wheat', 'white', 'whitesmoke', 'yellow', 'yellowgreen'].includes(str)).map(value => new NamedColor(value)))();
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
    return _tokenParser.TokenParser.tokens.Hash.map(token => token[4].value).where(value => [3, 6, 8].includes(value.length) && /^[0-9a-fA-F]+$/.test(value)).map(value => new HashColor(value));
  }
}
exports.HashColor = HashColor;
const rgbNumberParser = _tokenParser.TokenParser.tokens.Number.map(token => token[4].value).where(value => value >= 0 && value <= 255);
const alphaAsNumber = _alphaValue.AlphaValue.parser.map(alpha => alpha.value);
const slashParser = _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(value => value === '/').surroundedBy(_tokenParser.TokenParser.tokens.Whitespace);
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
    const rgbCommaSeparated = _tokenParser.TokenParser.sequence(rgbNumberParser, rgbNumberParser, rgbNumberParser).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    const commaParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgb'), rgbCommaSeparated, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref => {
      let [_fn, [r, g, b], _closeParen] = _ref;
      return new Rgb(r, g, b);
    });
    const spaceSeparatedRGB = _tokenParser.TokenParser.sequence(rgbNumberParser, rgbNumberParser, rgbNumberParser).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    const spaceParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgb'), spaceSeparatedRGB, _tokenParser.TokenParser.tokens.CloseParen).map(_ref2 => {
      let [_fn, [r, g, b], _closeParen] = _ref2;
      return new Rgb(r, g, b);
    });
    return _tokenParser.TokenParser.oneOf(commaParser, spaceParser);
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
    const commaParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgba'), rgbNumberParser, _tokenParser.TokenParser.tokens.Comma, rgbNumberParser, _tokenParser.TokenParser.tokens.Comma, rgbNumberParser, _tokenParser.TokenParser.tokens.Comma, alphaAsNumber, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref3 => {
      let [_fn, r, _comma, g, _comma2, b, _comma3, a, _closeParen] = _ref3;
      return new Rgba(r, g, b, a);
    });
    const spaceParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'rgb'), _tokenParser.TokenParser.tokens.Whitespace.optional, rgbNumberParser, _tokenParser.TokenParser.tokens.Whitespace, rgbNumberParser, _tokenParser.TokenParser.tokens.Whitespace, rgbNumberParser, slashParser, alphaAsNumber, _tokenParser.TokenParser.tokens.Whitespace.optional, _tokenParser.TokenParser.tokens.CloseParen).map(_ref4 => {
      let [_fn, _preSpace, r, _space, g, _space2, b, _slash, a, _postSpace, _closeParen] = _ref4;
      return new Rgba(r, g, b, a);
    });
    return _tokenParser.TokenParser.oneOf(commaParser, spaceParser);
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
    const commaParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsl'), _angle.Angle.parser, _tokenParser.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(tokens => new Hsl(tokens[1], tokens[3], tokens[5]));
    const spaceParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsl'), _angle.Angle.parser, _tokenParser.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.Whitespace, _tokenParser.TokenParser.tokens.CloseParen).map(tokens => new Hsl(tokens[1], tokens[3], tokens[5]));
    return _tokenParser.TokenParser.oneOf(commaParser, spaceParser);
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
    const commaParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsla'), _angle.Angle.parser, _tokenParser.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.Comma, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.Comma, alphaAsNumber, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref5 => {
      let [_fn, h, _comma, s, _comma2, l, _comma3, a, _closeParen] = _ref5;
      return new Hsla(h, s, l, a);
    });
    const spaceParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'hsl'), _angle.Angle.parser, _tokenParser.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, _tokenParser.TokenParser.tokens.Whitespace, _commonTypes.Percentage.parser, slashParser, alphaAsNumber, _tokenParser.TokenParser.tokens.Whitespace.optional, _tokenParser.TokenParser.tokens.CloseParen).map(_ref6 => {
      let [_fn, h, _space, s, _space2, l, _slash, a, _postSpace, _closeParen] = _ref6;
      return new Hsla(h, s, l, a);
    });
    return _tokenParser.TokenParser.oneOf(commaParser, spaceParser);
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
    const l = _tokenParser.TokenParser.oneOf(_commonTypes.Percentage.parser.map(p => p.value), _tokenParser.TokenParser.tokens.Number.map(token => token[4].value).where(value => value >= 0), _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value).where(value => value === 'none').map(() => 0));
    const c = _tokenParser.TokenParser.oneOf(_commonTypes.Percentage.parser.map(p => 150 * p.value / 100), _tokenParser.TokenParser.tokens.Number.map(token => token[4].value).where(value => value >= 0));
    const h = _tokenParser.TokenParser.oneOf(_angle.Angle.parser, _tokenParser.TokenParser.tokens.Number.map(token => token[4].value));
    const a = _tokenParser.TokenParser.sequence(slashParser, alphaAsNumber).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref7 => {
      let [_, a] = _ref7;
      return a;
    });
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'lch'), _tokenParser.TokenParser.sequence(l, c, h).separatedBy(_tokenParser.TokenParser.tokens.Whitespace), a.suffix(_tokenParser.TokenParser.tokens.Whitespace.optional).optional, _tokenParser.TokenParser.tokens.CloseParen).map(_ref8 => {
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
    const lc = _tokenParser.TokenParser.oneOf(alphaAsNumber, _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value).where(value => value === 'none').map(() => 0)).prefix(_tokenParser.TokenParser.tokens.Whitespace.optional);
    const h = _tokenParser.TokenParser.oneOf(_angle.Angle.parser, lc.map(num => new _angle.Angle(num * 360, 'deg')));
    const a = _tokenParser.TokenParser.sequence(slashParser, alphaAsNumber).map(_ref9 => {
      let [_, a] = _ref9;
      return a;
    });
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'oklch'), lc, _tokenParser.TokenParser.tokens.Whitespace, lc, _tokenParser.TokenParser.tokens.Whitespace, h, a.suffix(_tokenParser.TokenParser.tokens.Whitespace.optional).optional, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref0 => {
      let [_fn, l, _comma, c, _comma2, h, a, _closeParen] = _ref0;
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
    const lab = _tokenParser.TokenParser.oneOf(alphaAsNumber, _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value).where(value => value === 'none').map(() => 0)).prefix(_tokenParser.TokenParser.tokens.Whitespace.optional);
    const a = _tokenParser.TokenParser.sequence(slashParser, alphaAsNumber).map(_ref1 => {
      let [_, a] = _ref1;
      return a;
    });
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(token => token[4].value).where(value => value === 'oklab'), lab, _tokenParser.TokenParser.tokens.Whitespace, lab, _tokenParser.TokenParser.tokens.Whitespace, lab, a.suffix(_tokenParser.TokenParser.tokens.Whitespace.optional).optional, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref10 => {
      let [_fn, l, _comma, a, _comma2, b, alpha, _closeParen] = _ref10;
      return new Oklab(l, a, b, alpha);
    });
  }
}
exports.Oklab = Oklab;