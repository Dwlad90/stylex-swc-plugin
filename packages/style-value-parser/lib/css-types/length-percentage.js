"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.lengthPercentage = void 0;
var _tokenParser = require("../token-parser");
var _length = require("./length");
var _commonTypes = require("./common-types");
const lengthPercentage = exports.lengthPercentage = _tokenParser.TokenParser.oneOf(_commonTypes.Percentage.parser, _length.Length.parser);