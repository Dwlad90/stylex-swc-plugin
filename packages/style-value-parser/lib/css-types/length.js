"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.UNITS_BASED_ON_VIEWPORT = exports.UNITS_BASED_ON_FONT = exports.UNITS_BASED_ON_CONTAINER = exports.UNITS_BASED_ON_ABSOLUTE_UNITS = exports.Length = void 0;
var _tokenParser = require("../token-parser");
const UNITS_BASED_ON_FONT = exports.UNITS_BASED_ON_FONT = ['ch', 'em', 'ex', 'ic', 'lh', 'rem', 'rlh'];
const UNITS_BASED_ON_VIEWPORT = exports.UNITS_BASED_ON_VIEWPORT = ['vh', 'svh', 'lvh', 'dvh', 'vw', 'svw', 'lvw', 'dvw', 'vmin', 'svmin', 'lvmin', 'dvmin', 'vmax', 'svmax', 'lvmax', 'dvmax'];
const UNITS_BASED_ON_CONTAINER = exports.UNITS_BASED_ON_CONTAINER = ['cqw', 'cqi', 'cqh', 'cqb', 'cqmin', 'cqmax'];
const UNITS_BASED_ON_ABSOLUTE_UNITS = exports.UNITS_BASED_ON_ABSOLUTE_UNITS = ['px', 'cm', 'mm', 'in', 'pt'];
class Length {
  constructor(value, unit) {
    this.value = value;
    this.unit = unit;
  }
  toString() {
    return `${this.value}${this.unit}`;
  }
  static UNITS = (() => [...UNITS_BASED_ON_FONT, ...UNITS_BASED_ON_VIEWPORT, ...UNITS_BASED_ON_CONTAINER, ...UNITS_BASED_ON_ABSOLUTE_UNITS])();
  static get parser() {
    const united = _tokenParser.TokenParser.tokens.Dimension.map(token => [token[4].value, token[4].unit]).where(tuple => Length.UNITS.includes(tuple[1])).map(_ref => {
      let [value, unit] = _ref;
      return new Length(value, unit);
    });
    return _tokenParser.TokenParser.oneOf(united, _tokenParser.TokenParser.tokens.Number.map(token => token[4].value === 0 ? new Length(0, '') : null).where(value => value != null));
  }
}
exports.Length = Length;