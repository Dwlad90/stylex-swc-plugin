"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.valueParser = exports.Calc = void 0;
var _calcConstant = require("./calc-constant");
var _commonTypes = require("./common-types");
var _tokenParser = require("../token-parser");
const valueParser = exports.valueParser = _tokenParser.TokenParser.oneOf(_calcConstant.calcConstant, _tokenParser.TokenParser.tokens.Number.map(number => number[4].value), _tokenParser.TokenParser.tokens.Dimension.map(dimension => dimension[4]), _commonTypes.Percentage.parser);
const composeAddAndSubtraction = valuesAndOperators => {
  if (valuesAndOperators.length === 1) {
    if (typeof valuesAndOperators[0] === 'string') {
      if (_calcConstant.allCalcConstants.includes(valuesAndOperators[0])) {
        return valuesAndOperators[0];
      }
      throw new Error('Invalid operator');
    }
    return valuesAndOperators[0];
  }
  const firstOperator = valuesAndOperators.findIndex(op => op === '+' || op === '-');
  if (firstOperator === -1) {
    throw new Error('No valid operator found');
  }
  const left = valuesAndOperators.slice(0, firstOperator);
  const right = valuesAndOperators.slice(firstOperator + 1);
  if (valuesAndOperators[firstOperator] === '+') {
    return {
      type: '+',
      left: composeAddAndSubtraction(left),
      right: composeAddAndSubtraction(right)
    };
  }
  return {
    type: '-',
    left: composeAddAndSubtraction(left),
    right: composeAddAndSubtraction(right)
  };
};
const splitByMultiplicationOrDivision = valuesAndOperators => {
  if (valuesAndOperators.length === 1) {
    if (typeof valuesAndOperators[0] === 'string') {
      throw new Error('Invalid operator');
    }
    return valuesAndOperators[0];
  }
  const firstOperator = valuesAndOperators.findIndex(op => op === '*' || op === '/');
  if (firstOperator === -1) {
    return composeAddAndSubtraction(valuesAndOperators);
  }
  const left = valuesAndOperators.slice(0, firstOperator);
  const right = valuesAndOperators.slice(firstOperator + 1);
  if (valuesAndOperators[firstOperator] === '*') {
    return {
      type: '*',
      left: composeAddAndSubtraction(left),
      right: splitByMultiplicationOrDivision(right)
    };
  }
  return {
    type: '/',
    left: composeAddAndSubtraction(left),
    right: splitByMultiplicationOrDivision(right)
  };
};
let operationsParser;
const parenthesizedParser = _tokenParser.TokenParser.tokens.OpenParen.skip(_tokenParser.TokenParser.tokens.Whitespace.optional).flatMap(() => operationsParser.skip(_tokenParser.TokenParser.tokens.Whitespace.optional).skip(_tokenParser.TokenParser.tokens.CloseParen)).map(expr => ({
  type: 'group',
  expr
}));
operationsParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.oneOf(valueParser, parenthesizedParser), _tokenParser.TokenParser.zeroOrMore(_tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Delim.map(delim => delim[4].value).where(delim => delim === '*' || delim === '/' || delim === '+' || delim === '-'), _tokenParser.TokenParser.oneOf(valueParser, parenthesizedParser)).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional)).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional)).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref => {
  let [firstValue, restOfTheValues] = _ref;
  if (restOfTheValues == null || restOfTheValues.length === 0) {
    return firstValue;
  }
  const valuesAndOperators = [firstValue, ...restOfTheValues.flat()];
  return splitByMultiplicationOrDivision(valuesAndOperators);
});
class Calc {
  constructor(value) {
    this.value = value;
  }
  toString() {
    return `calc(${calcValueToString(this.value)})`;
  }
  static get parser() {
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Function.map(func => func[4].value).where(func => func === 'calc'), _tokenParser.TokenParser.oneOf(operationsParser, valueParser), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref2 => {
      let [_, value, _closeParen] = _ref2;
      return new Calc(value);
    });
  }
}
exports.Calc = Calc;
function calcValueToString(value) {
  if (typeof value === 'number') {
    return value.toString();
  }
  if (typeof value === 'string') {
    return value;
  }
  if (value != null && typeof value === 'object' && 'expr' in value) {
    const group = value;
    return '(' + calcValueToString(group.expr) + ')';
  }
  if (value != null && typeof value === 'object' && 'left' in value && 'right' in value && typeof value.type === 'string') {
    const opNode = value;
    return [calcValueToString(opNode.left), opNode.type, calcValueToString(opNode.right)].join(' ');
  }
  if (value != null && typeof value === 'object' && 'value' in value && 'unit' in value) {
    const d = value;
    return `${d.value}${d.unit}`;
  }
  return String(value);
}