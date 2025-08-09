"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.dimension = void 0;
var _tokenParser = require("../token-parser");
var _frequency = require("./frequency");
var _length = require("./length");
var _resolution = require("./resolution");
var _time = require("./time");
function arrIncludes(arr, val) {
  return arr.includes(val);
}
const dimension = exports.dimension = _tokenParser.TokenParser.tokens.Dimension.map(token => {
  const {
    unit,
    value
  } = token[4];
  if (arrIncludes(_length.Length.UNITS, unit)) {
    return new _length.Length(value, unit);
  } else if (arrIncludes(_time.Time.UNITS, unit)) {
    return new _time.Time(value, unit);
  } else if (arrIncludes(_resolution.Resolution.UNITS, unit)) {
    return new _resolution.Resolution(value, unit);
  } else if (arrIncludes(_frequency.Frequency.UNITS, unit)) {
    return new _frequency.Frequency(value, unit);
  } else {
    null;
  }
}).where(val => val != null);