"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.calcConstant = exports.allCalcConstants = void 0;
var _tokenParser = require("../token-parser");
const allCalcConstants = exports.allCalcConstants = ['pi', 'e', 'infinity', '-infinity', 'NaN'];
const calcConstant = exports.calcConstant = _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.string('pi'), _tokenParser.TokenParser.string('e'), _tokenParser.TokenParser.string('infinity'), _tokenParser.TokenParser.string('-infinity'), _tokenParser.TokenParser.string('NaN'));