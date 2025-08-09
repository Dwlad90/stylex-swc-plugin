"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.BorderRadiusShorthand = exports.BorderRadiusIndividual = void 0;
var _tokenParser = require("../token-parser");
var _lengthPercentage = require("../css-types/length-percentage");
class BorderRadiusIndividual {
  constructor(horizontal, vertical) {
    this.horizontal = horizontal;
    this.vertical = vertical ?? horizontal;
  }
  toString() {
    const horizontal = this.horizontal.toString();
    const vertical = this.vertical.toString();
    if (horizontal === vertical) {
      return horizontal;
    }
    return `${horizontal} ${vertical}`;
  }
  static get parse() {
    return _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage).separatedBy(_tokenParser.TokenParser.tokens.Whitespace), _lengthPercentage.lengthPercentage.map(p => [p, p])).map(_ref => {
      let [horizontal, vertical] = _ref;
      return new BorderRadiusIndividual(horizontal, vertical);
    });
  }
}
exports.BorderRadiusIndividual = BorderRadiusIndividual;
class BorderRadiusShorthand {
  constructor(horizontalTopLeft) {
    let horizontalTopRight = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : horizontalTopLeft;
    let horizontalBottomRight = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : horizontalTopLeft;
    let horizontalBottomLeft = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : horizontalTopRight;
    let verticalTopLeft = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : horizontalTopLeft;
    let verticalTopRight = arguments.length > 5 && arguments[5] !== undefined ? arguments[5] : verticalTopLeft;
    let verticalBottomRight = arguments.length > 6 && arguments[6] !== undefined ? arguments[6] : verticalTopLeft;
    let verticalBottomLeft = arguments.length > 7 && arguments[7] !== undefined ? arguments[7] : verticalTopRight;
    this.horizontalTopLeft = horizontalTopLeft;
    this.horizontalTopRight = horizontalTopRight;
    this.horizontalBottomRight = horizontalBottomRight;
    this.horizontalBottomLeft = horizontalBottomLeft;
    this.verticalTopLeft = verticalTopLeft;
    this.verticalTopRight = verticalTopRight;
    this.verticalBottomRight = verticalBottomRight;
    this.verticalBottomLeft = verticalBottomLeft;
  }
  toString() {
    const horizontalTopLeft = this.horizontalTopLeft.toString();
    const horizontalTopRight = this.horizontalTopRight.toString();
    const horizontalBottomRight = this.horizontalBottomRight.toString();
    const horizontalBottomLeft = this.horizontalBottomLeft.toString();
    let pStr = `${horizontalTopLeft} ${horizontalTopRight} ${horizontalBottomRight} ${horizontalBottomLeft}`;
    if (horizontalTopLeft === horizontalTopRight && horizontalTopRight === horizontalBottomRight && horizontalBottomRight === horizontalBottomLeft) {
      pStr = horizontalTopLeft;
    } else if (horizontalTopLeft === horizontalBottomRight && horizontalTopRight === horizontalBottomLeft) {
      pStr = `${horizontalTopLeft} ${horizontalTopRight}`;
    } else if (horizontalTopRight === horizontalBottomLeft) {
      pStr = `${horizontalTopLeft} ${horizontalTopRight} ${horizontalBottomRight}`;
    }
    const verticalTopLeft = this.verticalTopLeft.toString();
    const verticalTopRight = this.verticalTopRight.toString();
    const verticalBottomRight = this.verticalBottomRight.toString();
    const verticalBottomLeft = this.verticalBottomLeft.toString();
    let sStr = `${horizontalTopLeft} ${horizontalTopRight} ${horizontalBottomRight} ${horizontalBottomLeft}`;
    if (verticalTopLeft === verticalTopRight && verticalTopRight === verticalBottomRight && verticalBottomRight === verticalBottomLeft) {
      sStr = verticalTopLeft;
    } else if (verticalTopLeft === verticalBottomRight && verticalTopRight === verticalBottomLeft) {
      sStr = `${verticalTopLeft} ${verticalTopRight}`;
    } else if (verticalTopRight === verticalBottomLeft) {
      sStr = `${verticalTopLeft} ${verticalTopRight} ${verticalBottomRight}`;
    }
    if (pStr === sStr) {
      return pStr;
    }
    return `${pStr} / ${sStr}`;
  }
  static get parse() {
    const spaceSeparatedRadii = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage.prefix(_tokenParser.TokenParser.tokens.Whitespace).optional, _lengthPercentage.lengthPercentage.prefix(_tokenParser.TokenParser.tokens.Whitespace).optional, _lengthPercentage.lengthPercentage.prefix(_tokenParser.TokenParser.tokens.Whitespace).optional).map(_ref2 => {
      let [topLeft, topRight = topLeft, bottomRight = topLeft, bottomLeft = topRight] = _ref2;
      return [topLeft, topRight, bottomRight, bottomLeft];
    });
    const assymtricBorder = _tokenParser.TokenParser.sequence(spaceSeparatedRadii, spaceSeparatedRadii).separatedBy(_tokenParser.TokenParser.tokens.Delim.map(delim => delim[4].value).where(d => d === '/').surroundedBy(_tokenParser.TokenParser.tokens.Whitespace)).map(_ref3 => {
      let [pRadii, sRadii = pRadii] = _ref3;
      return new BorderRadiusShorthand(...pRadii, ...sRadii);
    });
    return _tokenParser.TokenParser.oneOf(assymtricBorder, spaceSeparatedRadii.map(borders => new BorderRadiusShorthand(...borders)));
  }
}
exports.BorderRadiusShorthand = BorderRadiusShorthand;