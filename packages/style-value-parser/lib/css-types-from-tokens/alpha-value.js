"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.AlphaValue = void 0;
var _core = require("../core2");
class AlphaValue {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return this.value.toString();
  }
  static parser = (() => _core.TokenParser.oneOf(_core.TokenParser.tokens.Percentage.map(v => new AlphaValue((v[4].signCharacter === '-' ? -1 : 1) * v[4].value / 100)), _core.TokenParser.tokens.Number.map(v => new AlphaValue((v[4].signCharacter === '-' ? -1 : 1) * v[4].value))))();
}
exports.AlphaValue = AlphaValue;