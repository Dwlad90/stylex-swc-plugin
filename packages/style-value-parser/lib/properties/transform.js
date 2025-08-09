"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Transform = void 0;
var _tokenParser = require("../token-parser");
var _transformFunction = require("../css-types/transform-function");
class Transform {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return this.value.join(' ');
  }
  static get parse() {
    return _tokenParser.TokenParser.oneOrMore(_transformFunction.TransformFunction.parser).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(value => new Transform(value));
  }
}
exports.Transform = Transform;