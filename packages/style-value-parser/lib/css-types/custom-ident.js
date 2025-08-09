"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.CustomIdentifier = void 0;
var _tokenParser = require("../token-parser");
class CustomIdentifier {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return this.value;
  }
  static get parser() {
    return _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value).where(str => !reservedKeywords.includes(str.toLowerCase())).map(value => new CustomIdentifier(value));
  }
}
exports.CustomIdentifier = CustomIdentifier;
const reservedKeywords = ['unset', 'initial', 'inherit', 'default', 'none', 'auto', 'normal', 'hidden', 'visible', 'revert', 'revert-layer'];