"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.TranslateAxis = exports.Translate3d = exports.Translate = exports.TransformFunction = exports.SkewAxis = exports.Skew = exports.ScaleAxis = exports.Scale3d = exports.Scale = exports.RotateXYZ = exports.Rotate3d = exports.Rotate = exports.Perspective = exports.Matrix3d = exports.Matrix = void 0;
var _length = require("./length");
var _tokenParser = require("../token-parser");
var _angle = require("./angle");
var _commonTypes = require("./common-types");
var _lengthPercentage = require("./length-percentage");
class TransformFunction {
  static get parser() {
    return _tokenParser.TokenParser.oneOf(Matrix.parser, Matrix3d.parser, Perspective.parser, Rotate.parser, RotateXYZ.parser, Rotate3d.parser, Scale.parser, Scale3d.parser, ScaleAxis.parser, Skew.parser, SkewAxis.parser, Translate3d.parser, Translate.parser, TranslateAxis.parser);
  }
}
exports.TransformFunction = TransformFunction;
class Matrix extends TransformFunction {
  constructor(a, b, c, d, tx, ty) {
    super();
    this.a = a;
    this.b = b;
    this.c = c;
    this.d = d;
    this.tx = tx;
    this.ty = ty;
  }
  toString() {
    return `matrix(${this.a}, ${this.b}, ${this.c}, ${this.d}, ${this.tx}, ${this.ty})`;
  }
  static get parser() {
    const sixNumbers = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value)).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('matrix'), sixNumbers, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref => {
      let [_fn, [a, b, c, d, tx, ty], _closeParen] = _ref;
      return new Matrix(a, b, c, d, tx, ty);
    });
  }
}
exports.Matrix = Matrix;
class Matrix3d extends TransformFunction {
  constructor(args) {
    super();
    this.args = args;
  }
  toString() {
    return `matrix3d(${this.args.join(', ')})`;
  }
  static get parser() {
    const number = _tokenParser.TokenParser.tokens.Number.map(v => v[4].value);
    const fourNumbers = _tokenParser.TokenParser.sequence(number, number, number, number).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    const sixteenNumbers = _tokenParser.TokenParser.sequence(fourNumbers, fourNumbers, fourNumbers, fourNumbers).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref2 => {
      let [f1, f2, f3, f4] = _ref2;
      return [...f1, ...f2, ...f3, ...f4];
    });
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'matrix3d'), sixteenNumbers, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref3 => {
      let [_, args] = _ref3;
      return new Matrix3d(args);
    });
  }
}
exports.Matrix3d = Matrix3d;
class Perspective extends TransformFunction {
  constructor(length) {
    super();
    this.length = length;
  }
  toString() {
    return `perspective(${this.length.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'perspective'), _length.Length.parser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref4 => {
      let [_, length] = _ref4;
      return new Perspective(length);
    });
  }
}
exports.Perspective = Perspective;
class Rotate extends TransformFunction {
  constructor(angle) {
    super();
    this.angle = angle;
  }
  toString() {
    return `rotate(${this.angle.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'rotate'), _angle.Angle.parser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref5 => {
      let [_, angle] = _ref5;
      return new Rotate(angle);
    });
  }
}
exports.Rotate = Rotate;
class RotateXYZ extends TransformFunction {
  constructor(x, axis) {
    super();
    this.x = x;
    this.axis = axis;
  }
  toString() {
    return `rotate${this.axis}(${this.x.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.fn('rotateX').map(() => 'X'), _tokenParser.TokenParser.fn('rotateY').map(() => 'Y'), _tokenParser.TokenParser.fn('rotateZ').map(() => 'Z')), _angle.Angle.parser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref6 => {
      let [axis, x] = _ref6;
      return new RotateXYZ(x, axis);
    });
  }
}
exports.RotateXYZ = RotateXYZ;
class Rotate3d extends TransformFunction {
  constructor(x, y, z, angle) {
    super();
    this.x = x;
    this.y = y;
    this.z = z;
    this.angle = angle;
  }
  toString() {
    const {
      x,
      y,
      z
    } = this;
    switch (true) {
      case x === 1 && y === 0 && z === 0:
        return `rotateX(${this.angle.toString()})`;
      case x === 0 && y === 1 && z === 0:
        return `rotateY(${this.angle.toString()})`;
      case x === 0 && y === 0 && z === 1:
        return `rotateZ(${this.angle.toString()})`;
      default:
        return `rotate3d(${this.x}, ${this.y}, ${this.z}, ${this.angle.toString()})`;
    }
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'rotate3d'), _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _tokenParser.TokenParser.tokens.Number.map(v => v[4].value), _angle.Angle.parser).separatedBy(_tokenParser.TokenParser.tokens.Comma.skip(_tokenParser.TokenParser.tokens.Whitespace.optional)), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref7 => {
      let [_, [x, y, z, angle]] = _ref7;
      return new Rotate3d(x, y, z, angle);
    });
  }
}
exports.Rotate3d = Rotate3d;
class Scale extends TransformFunction {
  constructor(sx, sy) {
    super();
    this.sx = sx;
    this.sy = sy ?? undefined;
  }
  toString() {
    const {
      sx,
      sy
    } = this;
    if (sy == null) {
      return `scale(${sx.toString()})`;
    }
    return `scale(${sx.toString()}, ${sy.toString()})`;
  }
  static get parser() {
    const scalesXY = _tokenParser.TokenParser.sequence(_commonTypes.numberOrPercentage.map(v => v instanceof _commonTypes.Percentage ? v.value / 100 : v), _commonTypes.numberOrPercentage.map(v => v instanceof _commonTypes.Percentage ? v.value / 100 : v).optional).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'scale'), scalesXY, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref8 => {
      let [_, [sx, sy]] = _ref8;
      return new Scale(sx, sy);
    });
  }
}
exports.Scale = Scale;
class Scale3d extends TransformFunction {
  constructor(sx, sy, sz) {
    super();
    this.sx = sx;
    this.sy = sy;
    this.sz = sz;
  }
  toString() {
    return `scale3d(${this.sx.toString()}, ${this.sy.toString()}, ${this.sz.toString()})`;
  }
  static get parser() {
    const numberOrPercentageAsNumber = _commonTypes.numberOrPercentage.map(v => v instanceof _commonTypes.Percentage ? v.value / 100 : v);
    const args = _tokenParser.TokenParser.sequence(numberOrPercentageAsNumber, numberOrPercentageAsNumber, numberOrPercentageAsNumber).separatedBy(_tokenParser.TokenParser.tokens.Comma).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional);
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.fn('scale3d'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref9 => {
      let [_, [sx, sy, sz]] = _ref9;
      return new Scale3d(sx, sy, sz);
    });
  }
}
exports.Scale3d = Scale3d;
class ScaleAxis extends TransformFunction {
  constructor(s, axis) {
    super();
    this.s = s;
    this.axis = axis;
  }
  toString() {
    return `scale${this.axis}(${this.s.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.fn('scaleX').map(() => 'X'), _tokenParser.TokenParser.fn('scaleY').map(() => 'Y'), _tokenParser.TokenParser.fn('scaleZ').map(() => 'Z')), _commonTypes.numberOrPercentage.map(v => v instanceof _commonTypes.Percentage ? v.value / 100 : v), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref0 => {
      let [axis, s] = _ref0;
      return new ScaleAxis(s, axis);
    });
  }
}
exports.ScaleAxis = ScaleAxis;
class Skew extends TransformFunction {
  constructor(ax, ay) {
    super();
    this.ax = ax;
    this.ay = ay ?? undefined;
  }
  toString() {
    const {
      ax,
      ay
    } = this;
    if (ay == null) {
      return `skew(${ax.toString()})`;
    }
    return `skew(${ax.toString()}, ${ay.toString()})`;
  }
  static get parser() {
    const args = _tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.sequence(_angle.Angle.parser, _angle.Angle.parser).separatedBy(_tokenParser.TokenParser.tokens.Comma.skip(_tokenParser.TokenParser.tokens.Whitespace.optional)), _angle.Angle.parser).map(arg => {
      if (Array.isArray(arg)) {
        return arg;
      }
      return [arg, null];
    });
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'skew'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref1 => {
      let [_, [ax, ay]] = _ref1;
      return new Skew(ax, ay);
    });
  }
}
exports.Skew = Skew;
class SkewAxis extends TransformFunction {
  constructor(a, axis) {
    super();
    this.a = a;
    this.axis = axis;
  }
  toString() {
    return `skew${this.axis}(${this.a.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.fn('skewX').map(() => 'X'), _tokenParser.TokenParser.fn('skewY').map(() => 'Y')), _angle.Angle.parser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref10 => {
      let [axis, a] = _ref10;
      return new SkewAxis(a, axis);
    });
  }
}
exports.SkewAxis = SkewAxis;
class Translate extends TransformFunction {
  constructor(tx, ty) {
    super();
    this.tx = tx;
    this.ty = ty ?? undefined;
  }
  toString() {
    const {
      tx,
      ty
    } = this;
    if (ty == null) {
      return `translate(${tx.toString()})`;
    }
    return `translate(${tx.toString()}, ${ty.toString()})`;
  }
  static get parser() {
    const oneArg = _lengthPercentage.lengthPercentage;
    const twoArgs = _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage).separatedBy(_tokenParser.TokenParser.tokens.Comma.skip(_tokenParser.TokenParser.tokens.Whitespace.optional));
    const args = _tokenParser.TokenParser.oneOf(twoArgs, oneArg).map(arg => {
      if (Array.isArray(arg)) {
        return arg;
      }
      return [arg, null];
    });
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'translate'), args, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref11 => {
      let [_, [tx, ty]] = _ref11;
      return new Translate(tx, ty);
    });
  }
}
exports.Translate = Translate;
class Translate3d extends TransformFunction {
  constructor(tx, ty, tz) {
    super();
    this.tx = tx;
    this.ty = ty;
    this.tz = tz;
  }
  toString() {
    return `translate3d(${this.tx.toString()}, ${this.ty.toString()}, ${this.tz.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(v => v[4].value).where(v => v === 'translate3d'), _tokenParser.TokenParser.sequence(_lengthPercentage.lengthPercentage, _lengthPercentage.lengthPercentage, _length.Length.parser).separatedBy(_tokenParser.TokenParser.tokens.Comma.skip(_tokenParser.TokenParser.tokens.Whitespace.optional)), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref12 => {
      let [_, [tx, ty, tz]] = _ref12;
      return new Translate3d(tx, ty, tz);
    });
  }
}
exports.Translate3d = Translate3d;
class TranslateAxis extends TransformFunction {
  constructor(t, axis) {
    super();
    this.t = t;
    this.axis = axis;
  }
  toString() {
    return `translate${this.axis}(${this.t.toString()})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.oneOf(_tokenParser.TokenParser.fn('translateX').map(() => 'X'), _tokenParser.TokenParser.fn('translateY').map(() => 'Y'), _tokenParser.TokenParser.fn('translateZ').map(() => 'Z')), _lengthPercentage.lengthPercentage, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref13 => {
      let [axis, t] = _ref13;
      return new TranslateAxis(t, axis);
    });
  }
}
exports.TranslateAxis = TranslateAxis;