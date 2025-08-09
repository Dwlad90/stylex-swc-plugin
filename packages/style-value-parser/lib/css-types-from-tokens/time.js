"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Time = void 0;
var _core = require("../core2");
class Time {
  constructor(value, unit) {
    this.value = value;
    this.unit = unit;
  }
  toString() {
    if (this.unit === 'ms') {
      return `${this.value / 1000}s`;
    }
    return `${this.value}${this.unit}`;
  }
  static UNITS = ['s', 'ms'];
  static get parser() {
    return _core.TokenParser.tokens.Dimension.map(v => v[4].unit === 's' || v[4].unit === 'ms' ? [v[4].value, v[4].unit] : null).where(v => v != null).map(_ref => {
      let [v, unit] = _ref;
      return new Time(v, unit);
    });
  }
}
exports.Time = Time;