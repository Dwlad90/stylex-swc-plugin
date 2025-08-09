"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.anglePercentage = void 0;
var _core = require("../core2");
var _angle = require("./angle");
var _commonTypes = require("./common-types");
const anglePercentage = exports.anglePercentage = _core.TokenParser.oneOf(_angle.Angle.parser, _commonTypes.Percentage.parser);