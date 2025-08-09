"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.MediaQuery = void 0;
exports.validateMediaQuery = validateMediaQuery;
var _tokenParser = require("../token-parser");
var _calc = require("../css-types/calc");
var _messages = require("./messages");
function adjustDimension(dimension, op, eq) {
  let reversed = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : false;
  let adjustedValue = dimension.value;
  const epsilon = 0.01;
  if (eq !== '=') {
    if (!reversed) {
      if (op === '>') {
        adjustedValue += epsilon;
      } else if (op === '<') {
        adjustedValue -= epsilon;
      }
    } else {
      if (op === '>') {
        adjustedValue -= epsilon;
      } else if (op === '<') {
        adjustedValue += epsilon;
      }
    }
  }
  return {
    ...dimension,
    value: adjustedValue
  };
}
const basicMediaTypeParser = _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(key => key === 'screen' || key === 'print' || key === 'all');
const mediaKeywordParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.string('not').optional, _tokenParser.TokenParser.string('only').optional, basicMediaTypeParser).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref => {
  let [not, only, keyword] = _ref;
  return {
    type: 'media-keyword',
    key: keyword,
    not: not === 'not',
    only: only === 'only'
  };
});
const mediaWordRuleParser = _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen).where(key => key === 'color' || key === 'monochrome' || key === 'grid' || key === 'color-index').map(key => ({
  type: 'word-rule',
  keyValue: key
}));
const mediaRuleValueParser = _tokenParser.TokenParser.oneOf(_calc.Calc.parser.map(calc => calc.toString()), _tokenParser.TokenParser.tokens.Dimension.map(token => token[4]), _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value), _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Number.map(token => token[4].value), _tokenParser.TokenParser.tokens.Delim.where(token => token[4].value === '/').map(() => '/'), _tokenParser.TokenParser.tokens.Number.map(token => token[4].value)).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional));
const simplePairParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue'), _tokenParser.TokenParser.tokens.Colon, mediaRuleValueParser, _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref2 => {
  let [_openParen, key, _colon, value, _closeParen] = _ref2;
  return {
    type: 'pair',
    key,
    value
  };
});
const mediaInequalityRuleParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(val => val === 'width' || val === 'height'), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _tokenParser.TokenParser.tokens.Dimension.map(token => token[4]), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref3 => {
  let [_openParen, key, op, eq, dimension, _closeParen] = _ref3;
  const finalKey = op === '>' ? `min-${key}` : `max-${key}`;
  const adjustedDimension = adjustDimension(dimension, op, eq);
  return {
    type: 'pair',
    key: finalKey,
    value: adjustedDimension
  };
});
const mediaInequalityRuleParserReversed = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.Dimension.map(token => token[4]), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(val => val === 'width' || val === 'height'), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref4 => {
  let [_openParen, dimension, op, eq, key, _closeParen] = _ref4;
  const finalKey = op === '>' ? `max-${key}` : `min-${key}`;
  const adjustedDimension = adjustDimension(dimension, op, eq, true);
  return {
    type: 'pair',
    key: finalKey,
    value: adjustedDimension
  };
});
const combinedInequalityParser = _tokenParser.TokenParser.oneOf(mediaInequalityRuleParser, mediaInequalityRuleParserReversed);
const doubleInequalityRuleParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.Dimension.map(token => token[4]), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(val => val === 'width' || val === 'height'), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _tokenParser.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _tokenParser.TokenParser.tokens.Dimension.map(token => token[4]), _tokenParser.TokenParser.tokens.CloseParen).separatedBy(_tokenParser.TokenParser.tokens.Whitespace.optional).map(_ref5 => {
  let [_openParen, lower, op, eq, key, op2, eq2, upper, _closeParen] = _ref5;
  const lowerKey = op === '<' ? `min-${key}` : `max-${key}`;
  const upperKey = op2 === '<' ? `max-${key}` : `min-${key}`;
  const lowerValue = adjustDimension(lower, op, eq);
  const upperValue = adjustDimension(upper, op2, eq2);
  return {
    type: 'and',
    rules: [{
      type: 'pair',
      key: lowerKey,
      value: lowerValue
    }, {
      type: 'pair',
      key: upperKey,
      value: upperValue
    }]
  };
});
const mediaAndRulesParser = _tokenParser.TokenParser.oneOrMore(_tokenParser.TokenParser.oneOf(mediaKeywordParser, () => mediaOrRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => notParser, doubleInequalityRuleParser, combinedInequalityParser, simplePairParser, mediaWordRuleParser)).separatedBy(_tokenParser.TokenParser.string('and').surroundedBy(_tokenParser.TokenParser.tokens.Whitespace)).where(rules => Array.isArray(rules) && rules.length > 1).map(rules => rules.length === 1 ? rules[0] : {
  type: 'and',
  rules
});
const mediaOrRulesParser = _tokenParser.TokenParser.oneOrMore(_tokenParser.TokenParser.oneOf(mediaKeywordParser, () => mediaOrRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => notParser, doubleInequalityRuleParser, combinedInequalityParser, simplePairParser, mediaWordRuleParser)).separatedBy(_tokenParser.TokenParser.string('or').surroundedBy(_tokenParser.TokenParser.tokens.Whitespace)).where(rules => rules.length > 1).map(rules => rules.length === 1 ? rules[0] : {
  type: 'or',
  rules
});
let notParser;
const getNormalRuleParser = () => _tokenParser.TokenParser.oneOf(() => basicMediaTypeParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen).map(keyword => ({
  type: 'media-keyword',
  key: keyword,
  not: false
})), mediaAndRulesParser, mediaOrRulesParser, simplePairParser, mediaWordRuleParser, () => notParser, () => mediaOrRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen)).skip(_tokenParser.TokenParser.tokens.Whitespace.optional);
notParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.string('not'), _tokenParser.TokenParser.tokens.Whitespace, getNormalRuleParser(), _tokenParser.TokenParser.tokens.CloseParen).map(_ref6 => {
  let [_openParen, _not, _space, rule, _closeParen] = _ref6;
  return {
    type: 'not',
    rule
  };
});
class MediaQuery {
  constructor(queries) {
    this.queries = MediaQuery.normalize(queries);
  }
  toString() {
    return `@media ${this.#toString(this.queries, true)}`;
  }
  #toString(queries) {
    let isTopLevel = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : false;
    switch (queries.type) {
      case 'media-keyword':
        {
          const prefix = queries.not ? 'not ' : queries.only ? 'only ' : '';
          return prefix + queries.key;
        }
      case 'word-rule':
        return `(${queries.keyValue})`;
      case 'pair':
        {
          const {
            key,
            value
          } = queries;
          if (Array.isArray(value)) {
            return `(${key}: ${value[0]} / ${value[2]})`;
          }
          if (typeof value === 'string') {
            return `(${key}: ${value})`;
          }
          if (value != null && typeof value === 'object' && typeof value.value === 'number' && typeof value.unit === 'string') {
            const len = value;
            return `(${key}: ${len.value}${len.unit})`;
          }
          if (value != null && typeof value.toString === 'function') {
            return `(${key}: ${value.toString()})`;
          }
          throw new Error(`cannot serialize media-pair value for key "${key}": ${String(value)}`);
        }
      case 'not':
        return queries.rule && (queries.rule.type === 'and' || queries.rule.type === 'or') ? `(not (${this.#toString(queries.rule)}))` : `(not ${this.#toString(queries.rule)})`;
      case 'and':
        return queries.rules.map(rule => this.#toString(rule)).join(' and ');
      case 'or':
        return isTopLevel ? queries.rules.map(rule => this.#toString(rule)).join(', ') : queries.rules.map(rule => this.#toString(rule)).join(' or ');
      default:
        return '';
    }
  }
  static normalize(rule) {
    switch (rule.type) {
      case 'and':
        {
          const flattened = [];
          for (const r of rule.rules) {
            const norm = MediaQuery.normalize(r);
            if (norm.type === 'and') {
              flattened.push(...norm.rules);
            } else {
              flattened.push(norm);
            }
          }
          return {
            type: 'and',
            rules: flattened
          };
        }
      case 'or':
        return {
          type: 'or',
          rules: rule.rules.map(r => MediaQuery.normalize(r))
        };
      case 'not':
        {
          let count = 1;
          let current = rule.rule;
          while (current && current.type === 'not') {
            count++;
            current = current.rule;
          }
          const normalizedOperand = MediaQuery.normalize(current);
          return count % 2 === 0 ? normalizedOperand : {
            type: 'not',
            rule: normalizedOperand
          };
        }
      default:
        return rule;
    }
  }
  static get parser() {
    const leadingNotParser = _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(key => key === 'not'), _tokenParser.TokenParser.oneOf(() => mediaOrRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => notParser, combinedInequalityParser, simplePairParser, mediaWordRuleParser)).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref7 => {
      let [_not, queries] = _ref7;
      return {
        type: 'not',
        rule: queries
      };
    });
    const normalRuleParser = _tokenParser.TokenParser.oneOf(mediaAndRulesParser, mediaOrRulesParser, mediaKeywordParser, () => notParser, doubleInequalityRuleParser, combinedInequalityParser, simplePairParser, mediaWordRuleParser, () => mediaOrRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_tokenParser.TokenParser.tokens.OpenParen, _tokenParser.TokenParser.tokens.CloseParen));
    return _tokenParser.TokenParser.sequence(_tokenParser.TokenParser.tokens.AtKeyword.where(token => token[4].value === 'media'), _tokenParser.TokenParser.oneOrMore(_tokenParser.TokenParser.oneOf(leadingNotParser, normalRuleParser)).separatedBy(_tokenParser.TokenParser.tokens.Comma.surroundedBy(_tokenParser.TokenParser.tokens.Whitespace.optional))).separatedBy(_tokenParser.TokenParser.tokens.Whitespace).map(_ref8 => {
      let [_at, querySets] = _ref8;
      const rule = querySets.length > 1 ? {
        type: 'or',
        rules: querySets
      } : querySets[0];
      return new MediaQuery(rule);
    });
  }
}
exports.MediaQuery = MediaQuery;
function _hasBalancedParens(str) {
  let count = 0;
  for (const char of Array.from(str)) {
    if (char === '(') count++;
    if (char === ')') count--;
    if (count < 0) return false;
  }
  return count === 0;
}
function validateMediaQuery(input) {
  if (!_hasBalancedParens(input)) {
    throw new Error(_messages.MediaQueryErrors.UNBALANCED_PARENS);
  }
  try {
    return MediaQuery.parser.parseToEnd(input);
  } catch (err) {
    throw new Error(_messages.MediaQueryErrors.SYNTAX_ERROR);
  }
}