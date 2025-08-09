"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Polygon = exports.Path = exports.Inset = exports.Ellipse = exports.Circle = void 0;
var _position = require("./position");
var _tokenParser = require("../token-parser");
var _lengthPercentage = require("./length-percentage");
class BasicShape {
  toString() {
    throw new Error('Not implemented. Use a sub-class instead.');
  }
}
class Inset extends BasicShape {
  constructor(top, right, bottom, left, round) {
    super();
    this.top = top;
    this.right = right;
    this.bottom = bottom;
    this.left = left;
    this.round = round;
  }
  toString() {
    const {
      top,
      right,
      bottom,
      left,
      round
    } = this;
    const roundStr = this.round != null ? ` round ${this.round.toString()}` : '';
    if (top === right && right === bottom && bottom === left && left === round) {
      return `inset(${top.toString()}${roundStr})`;
    }
    if (top === bottom && left === right) {
      return `inset(${top.toString()} ${right.toString()}${roundStr})`;
    }
    if (top === bottom) {
      return `inset(${top.toString()} ${right.toString()} ${bottom.toString()}${roundStr})`;
    }
    return `inset(${top.toString()} ${right.toString()} ${bottom.toString()} ${left.toString()} ${roundStr})`;
  }
  static get parser() {
    const insets = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage.optional, _lengthPercentage.lengthPercentage.optional, _lengthPercentage.lengthPercentage.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref => {
      let [t, r = t, b = t, l = r] = _ref;
      return [t, r, b, l];
    });
    const round = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.string('round'), _lengthPercentage.lengthPercentage).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref2 => {
      let [, v] = _ref2;
      return v;
    });
    const args = _tokenParser.TokenParser.sequence(insets, round.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('inset'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref3 => {
      let [_, [[t, r, b, l], round]] = _ref3;
      return new Inset(t, r, b, l, round);
    });
  }
}
exports.Inset = Inset;
class Circle extends BasicShape {
  constructor(radius, position) {
    super();
    this.radius = radius;
    this.position = position;
  }
  toString() {
    const {
      radius,
      position
    } = this;
    const positionStr = position != null ? ` at ${position.toString()}` : '';
    return `circle(${radius.toString()}${positionStr})`;
  }
  static get parser() {
    const radius = _tokenParser.TokenParser.oneOf(_lengthPercentage.lengthPercentage, _tokenParser.TokenParser.string('closest-side'), _tokenParser.TokenParser.string('farthest-side'));
    const position = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.string('at'), _position.Position.parser).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref4 => {
      let [, v] = _ref4;
      return v;
    });
    const args = _tokenParser.TokenParser.sequence(radius, position.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('circle'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref5 => {
      let [_, [radius, position]] = _ref5;
      return new Circle(radius, position);
    });
  }
}
exports.Circle = Circle;
class Ellipse extends BasicShape {
  constructor(radiusX, radiusY, position) {
    super();
    this.radiusX = radiusX;
    this.radiusY = radiusY;
    this.position = position;
  }
  toString() {
    const {
      radiusX,
      radiusY,
      position
    } = this;
    const positionStr = position != null ? ` at ${position.toString()}` : '';
    return `ellipse(${radiusX.toString()} ${radiusY.toString()}${positionStr})`;
  }
  static get parser() {
    const radius = _tokenParser.TokenParser.oneOf(_lengthPercentage.lengthPercentage, _tokenParser.TokenParser.string('closest-side'), _tokenParser.TokenParser.string('farthest-side'));
    const position = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.string('at'), _position.Position.parser).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref6 => {
      let [_at, v] = _ref6;
      return v;
    });
    const args = _tokenParser.TokenParser.sequence(radius, radius, position.optional).separatedBy(_tokenParser.TokenParser.tokens.Whitespace);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('ellipse'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref7 => {
      let [_, [radiusX, radiusY, position]] = _ref7;
      return new Ellipse(radiusX, radiusY, position);
    });
  }
}
exports.Ellipse = Ellipse;
const fillRule = _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.string('nonzero'), _tokenParser.TokenParser.string('evenodd'));
class Polygon extends BasicShape {
  constructor(points, fillRule) {
    super();
    this.points = points;
    this.fillRule = fillRule;
  }
  toString() {
    const fillRule = this.fillRule != null ? `${this.fillRule}, ` : '';
    return `polygon(${fillRule}${this.points.map(_ref8 => {
      let [x, y] = _ref8;
      return `${x.toString()} ${y.toString()}`;
    }).join(', ')})`;
  }
  static get parser() {
    const point = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage).separatedBy(_tokenParser.TokenParser.tokens.Whitespace);
    const points = _tokenParser.TokenParser.oneOrMore(point).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    const args = _tokenParser.TokenParser.sequence(fillRule.optional, points).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('polygon'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref9 => {
      let [_, [fillRule, points]] = _ref9;
      return new Polygon(points, fillRule);
    });
  }
}
exports.Polygon = Polygon;
class Path extends BasicShape {
  constructor(path, fillRule) {
    super();
    this.path = path;
    this.fillRule = fillRule;
  }
  toString() {
    const fillRule = this.fillRule != null ? `${this.fillRule}, ` : '';
    return `path(${fillRule}"${this.path}")`;
  }
  static get parser() {
    const args = _tokenParser.TokenParser.sequence(fillRule.optional, _tokenParser.TokenParser.tokens.String.map(v => v[4].value)).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('path'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref0 => {
      let [_, [fillRule, path]] = _ref0;
      return new Path(path, fillRule);
    });
  }
}
exports.Path = Path;