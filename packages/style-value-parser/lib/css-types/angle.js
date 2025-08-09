"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Angle = void 0;
var _tokenParser = require("../token-parser");
class Angle {
  constructor(value, unit) {
    this.value = value;
    this.unit = unit;
  }
  toString() {
    return `${this.value}${this.unit}`;
  }
  static get parser() {
    const withUnit = _tokenParser.TokenParser.tokens.Dimension.map(v => v[4]).where(v => v.unit === 'deg' || v.unit === 'grad' || v.unit === 'rad' || v.unit === 'turn').map(v => new Angle(v.value, v.unit));
    return _tokenParser.TokenParser.oneOf(withUnit, _tokenParser.TokenParser.tokens.Number.map(v => v[4].value === 0 ? new Angle(0, 'deg') : null).where(v => v != null));
  }
}
exports.Angle = Angle;