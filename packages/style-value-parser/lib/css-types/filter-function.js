"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.SepiaFilterFunction = exports.SaturateFilterFunction = exports.OpacityFilterFunction = exports.InverFilterFunction = exports.HueRotateFilterFunction = exports.GrayscaleFilterFunction = exports.FilterFunction = exports.ContrastFilterFunction = exports.BrightnessFilterFunction = exports.BlurFilterFunction = void 0;
var _tokenParser = require("../token-parser");
var _length = require("./length");
var _commonTypes = require("./common-types");
var _angle = require("./angle");
class FilterFunction {
  toString() {
    return '';
  }
  static get parser() {
    return _tokenParser.TokenParser.oneOf(BlurFilterFunction.parser, BrightnessFilterFunction.parser, ContrastFilterFunction.parser, GrayscaleFilterFunction.parser, HueRotateFilterFunction.parser, InverFilterFunction.parser, OpacityFilterFunction.parser, SaturateFilterFunction.parser, SepiaFilterFunction.parser);
  }
}
exports.FilterFunction = FilterFunction;
class BlurFilterFunction extends FilterFunction {
  constructor(radius) {
    super();
    this.radius = radius;
  }
  toString() {
    return `blur(${this.radius.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _length.Length.parser.surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref => {
      let [_, radius, _1] = _ref;
      return new BlurFilterFunction(radius);
    });
  }
}
exports.BlurFilterFunction = BlurFilterFunction;
class BrightnessFilterFunction extends FilterFunction {
  constructor(percentage) {
    super();
    this.percentage = percentage;
  }
  toString() {
    return `brightness(${this.percentage})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref2 => {
      let [_, percentage, _1] = _ref2;
      return new BrightnessFilterFunction(percentage);
    });
  }
}
exports.BrightnessFilterFunction = BrightnessFilterFunction;
class ContrastFilterFunction extends FilterFunction {
  constructor(amount) {
    super();
    this.amount = amount;
  }
  toString() {
    return `contrast(${this.amount})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref3 => {
      let [_, amount, _1] = _ref3;
      return new ContrastFilterFunction(amount);
    });
  }
}
exports.ContrastFilterFunction = ContrastFilterFunction;
class GrayscaleFilterFunction extends FilterFunction {
  constructor(amount) {
    super();
    this.amount = amount;
  }
  toString() {
    return `grayscale(${this.amount})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref4 => {
      let [_, amount, _1] = _ref4;
      return new GrayscaleFilterFunction(amount);
    });
  }
}
exports.GrayscaleFilterFunction = GrayscaleFilterFunction;
class HueRotateFilterFunction extends FilterFunction {
  constructor(angle) {
    super();
    this.angle = angle;
  }
  toString() {
    return `hue-rotate(${this.angle.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _angle.Angle.parser, _tokenParser.TokenParser.tokens.CloseParen).map(_ref5 => {
      let [_, angle, _1] = _ref5;
      return new HueRotateFilterFunction(angle);
    });
  }
}
exports.HueRotateFilterFunction = HueRotateFilterFunction;
class InverFilterFunction extends FilterFunction {
  constructor(amount) {
    super();
    this.amount = amount;
  }
  toString() {
    return `invert(${this.amount})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref6 => {
      let [_, amount, _1] = _ref6;
      return new InverFilterFunction(amount);
    });
  }
}
exports.InverFilterFunction = InverFilterFunction;
class OpacityFilterFunction extends FilterFunction {
  constructor(amount) {
    super();
    this.amount = amount;
  }
  toString() {
    return `opacity(${this.amount})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref7 => {
      let [_, amount, _1] = _ref7;
      return new OpacityFilterFunction(amount);
    });
  }
}
exports.OpacityFilterFunction = OpacityFilterFunction;
class SaturateFilterFunction extends FilterFunction {
  constructor(amount) {
    super();
    this.amount = amount;
  }
  toString() {
    return `saturate(${this.amount})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref8 => {
      let [_, amount, _1] = _ref8;
      return new SaturateFilterFunction(amount);
    });
  }
}
exports.SaturateFilterFunction = SaturateFilterFunction;
class SepiaFilterFunction extends FilterFunction {
  constructor(amount) {
    super();
    this.amount = amount;
  }
  toString() {
    return `sepia(${this.amount})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function, _commonTypes.numberOrPercentage.map(p => typeof p === 'number' ? p : p.value / 100).surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional), _tokenParser.TokenParser.tokens.CloseParen).map(_ref9 => {
      let [_, amount, _1] = _ref9;
      return new SepiaFilterFunction(amount);
    });
  }
}
exports.SepiaFilterFunction = SepiaFilterFunction;