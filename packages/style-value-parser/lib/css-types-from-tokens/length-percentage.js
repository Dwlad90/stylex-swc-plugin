"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.lengthPercentage = void 0;
var _core = require("../core2");
var _length = require("./length");
var _commonTypes = require("./common-types");
const lengthPercentage = exports.lengthPercentage = _core.TokenParser.oneOf(_commonTypes.Percentage.parser, _length.Length.parser);