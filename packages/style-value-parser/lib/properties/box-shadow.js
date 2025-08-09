"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.BoxShadowList = exports.BoxShadow = void 0;
var _tokenParser = require("../token-parser");
var _length = require("../css-types/length");
var _color = require("../css-types/color");
class BoxShadow {
  constructor(offsetX, offsetY, blurRadius, spreadRadius, color) {
    let inset = arguments.length > 5 && arguments[5] !== undefined ? arguments[5] : false;
    this.offsetX = offsetX;
    this.offsetY = offsetY;
    this.blurRadius = blurRadius;
    this.spreadRadius = spreadRadius;
    this.color = color;
    this.inset = inset;
  }
  static get parse() {
    const outerShadow = _tokenParser.TokenParser.sequence(_length.Length.parser, _length.Length.parser, _length.Length.parser.optional, _length.Length.parser.optional, _color.Color.parser).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref => {
      let [offsetX, offsetY, blurRadius, spreadRadius, color] = _ref;
      return new BoxShadow(offsetX, offsetY, blurRadius ?? new _length.Length(0, 'px'), spreadRadius ?? new _length.Length(0, 'px'), color);
    });
    const insetShadow = _tokenParser.TokenParser.sequence(outerShadow, _tokenParser.TokenParser.string('inset')).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref2 => {
      let [shadow, _inset] = _ref2;
      return new BoxShadow(shadow.offsetX, shadow.offsetY, shadow.blurRadius, shadow.spreadRadius, shadow.color, true);
    });
    return _tokenParser.TokenParser.oneOf(insetShadow, outerShadow);
  }
}
exports.BoxShadow = BoxShadow;
class BoxShadowList {
  constructor(shadows) {
    this.shadows = shadows;
  }
  static get parse() {
    return _tokenParser.TokenParser.oneOrMore(BoxShadow.parse).separatedBy(_tokenParser.TokenParser.tokens.Comma.surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional)).map(shadows => new BoxShadowList(shadows));
  }
}
exports.BoxShadowList = BoxShadowList;