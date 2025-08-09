"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Frequency = void 0;
var _core = require("../core2");
class Frequency {
  constructor(value, unit) {
    this.value = value;
    this.unit = unit;
  }
  toString() {
    if (this.unit === 'Hz') {
      return `${this.value / 1000}KHz`;
    }
    return `${this.value}${this.unit}`;
  }
  static UNITS = ['Hz', 'KHz'];
  static get parser() {
    return _core.TokenParser.tokens.Dimension.map(val => val[4].unit === 'Hz' || val[4].unit === 'KHz' ? [val[4].value, val[4].unit] : null).where(v => v !== null).map(_ref => {
      let [value, unit] = _ref;
      return new Frequency(value, unit);
    });
  }
}
exports.Frequency = Frequency;