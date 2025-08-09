"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.calcConstant = exports.allCalcConstants = void 0;
var _core = require("../core2");
const allCalcConstants = exports.allCalcConstants = ['pi', 'e', 'infinity', '-infinity', 'NaN'];
const calcConstant = exports.calcConstant = _core.TokenParser.oneOf(_core.TokenParser.string('pi'), _core.TokenParser.string('e'), _core.TokenParser.string('infinity'), _core.TokenParser.string('-infinity'), _core.TokenParser.string('NaN'));