"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.blendMode = void 0;
var _tokenParser = require("../token-parser");
const blendMode = exports.blendMode = _tokenParser.TokenParser.tokens.Ident.map(v => v[4].value).where(str => str === 'normal' || str === 'multiply' || str === 'screen' || str === 'overlay' || str === 'darken' || str === 'lighten' || str === 'color-dodge' || str === 'color-burn' || str === 'hard-light' || str === 'soft-light' || str === 'difference' || str === 'exclusion' || str === 'hue' || str === 'saturation' || str === 'color' || str === 'luminosity');