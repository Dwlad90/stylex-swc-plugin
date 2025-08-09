"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Flex = void 0;
var _core = require("../core2");
class Flex {
  constructor(fraction) {
    this.fraction = fraction;
  }
  toString() {
    return `${this.fraction}fr`;
  }
  static get parser() {
    return _core.TokenParser.tokens.Dimension.map(dim => dim[4].unit === 'fr' && dim[4].signCharacter !== '-' ? dim[4].value : null).where(val => val != null).map(value => new Flex(value));
  }
}
exports.Flex = Flex;