"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.DashedIdentifier = void 0;
var _core = require("../core2");
class DashedIdentifier {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return this.value;
  }
  static get parser() {
    return _core.TokenParser.tokens.Ident.map(token => token[4].value).where(str => str.startsWith('--') && str.length > 2).map(value => new DashedIdentifier(value));
  }
}
exports.DashedIdentifier = DashedIdentifier;