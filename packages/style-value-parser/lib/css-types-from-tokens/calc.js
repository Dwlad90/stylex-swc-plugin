"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.Calc = void 0;
var _calcConstant = require("./calc-constant");
var _commonTypes = require("./common-types");
var _core = require("../core2");
const valueParser = _core.TokenParser.oneOf(_calcConstant.calcConstant, _core.TokenParser.tokens.Number.map(number => number[4].value), _core.TokenParser.tokens.Dimension.map(dimension => dimension[4]), _commonTypes.Percentage.parser);
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
const operationsParser = _core.TokenParser.sequence(_core.TokenParser.oneOf(valueParser, () => _core.TokenParser.sequence(_core.TokenParser.tokens.OpenParen, operationsParser, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref => {
  let [_, value] = _ref;
  return value;
})), _core.TokenParser.zeroOrMore(_core.TokenParser.sequence(_core.TokenParser.tokens.Delim.map(delim => delim[4].value).where(delim => delim === '*' || delim === '/' || delim === '+' || delim === '-'), _core.TokenParser.oneOf(valueParser, () => _core.TokenParser.sequence(_core.TokenParser.tokens.OpenParen, operationsParser, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref2 => {
  let [_, value] = _ref2;
  return value;
}))).separatedBy(_core.TokenParser.tokens.Whitespace.optional)).separatedBy(_core.TokenParser.tokens.Whitespace.optional)).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref3 => {
  let [firstValue, restOfTheValues] = _ref3;
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
    return this.value.toString();
  }
  static get parser() {
    return _core.TokenParser.sequence(_core.TokenParser.tokens.Function.map(func => func[4].value).where(func => func === 'calc'), _core.TokenParser.oneOf(operationsParser, valueParser), _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref4 => {
      let [_, value, _closeParen] = _ref4;
      return new Calc(value);
    });
  }
}
exports.Calc = Calc;