"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.mediaInequalityRuleParser = exports.MediaQuery = void 0;
var _core = require("../core2");
const _mediaKeywordParser = _core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(key => key === 'screen' || key === 'print' || key === 'all', '=== "screen" | "print" | "all"');
function _isAndOrRule(rule) {
  return rule.type === 'and' || rule.type === 'or';
}
const mediaKeywordParser = _core.TokenParser.sequence(_core.TokenParser.string('not').optional, _mediaKeywordParser).separatedBy(_core.TokenParser.tokens.Whitespace).map(_ref => {
  let [not, keyword] = _ref;
  return {
    type: 'media-keyword',
    key: keyword,
    not: not === 'not'
  };
});
const mediaWordRuleParser = _core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen).where(key => key === 'color' || key === 'monochrome' || key === 'grid' || key === 'color-index', '=== "color" | "monochrome" | "grid" | "color-index"').map(key => ({
  type: 'word-rule',
  keyValue: key
}));
const mediaRuleValueParser = _core.TokenParser.oneOf(_core.TokenParser.tokens.Dimension.map(token => token[4]), _core.TokenParser.tokens.Ident.map(token => token[4].value), _core.TokenParser.sequence(_core.TokenParser.tokens.Number.map(token => token[4].value), _core.TokenParser.tokens.Delim.where(token => token[4].value === '/').map(() => '/'), _core.TokenParser.tokens.Number.map(token => token[4].value)).separatedBy(_core.TokenParser.tokens.Whitespace.optional));
const simplePairParser = _core.TokenParser.sequence(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue'), _core.TokenParser.tokens.Colon, mediaRuleValueParser, _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref2 => {
  let [_openParen, key, _colon, value, _closeParen] = _ref2;
  return {
    type: 'pair',
    key,
    value
  };
});
const notParser = _core.TokenParser.sequence(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(key => key === 'not'), _core.TokenParser.tokens.Whitespace, _core.TokenParser.oneOf(simplePairParser, mediaWordRuleParser), _core.TokenParser.tokens.CloseParen).map(_ref3 => {
  let [_openParen, _not, _space, rule, _closeParen] = _ref3;
  return {
    type: 'not',
    rule
  };
});
const mediaInequalityRuleParser = exports.mediaInequalityRuleParser = _core.TokenParser.sequence(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(val => val === 'width' || val === 'height'), _core.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _core.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _core.TokenParser.tokens.Dimension.map(token => token[4]), _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref4 => {
  let [_openParen, key, op, eq, value, _closeParen] = _ref4;
  const finalKey = op === '>' ? `min-${key}` : `max-${key}`;
  const finalValue = op === '>' && eq !== '=' ? {
    ...value,
    value: value.value + 0.01
  } : op === '<' && eq !== '=' ? {
    ...value,
    value: value.value - 0.01
  } : value;
  return {
    type: 'pair',
    key: finalKey,
    value: finalValue
  };
});
const doubleInequalityRuleParser = _core.TokenParser.sequence(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.Dimension.map(token => token[4]), _core.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _core.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(val => val === 'width' || val === 'height'), _core.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '>' || val === '<'), _core.TokenParser.tokens.Delim.map(token => token[4].value).where(val => val === '=').optional, _core.TokenParser.tokens.Dimension.map(token => token[4]), _core.TokenParser.tokens.CloseParen).separatedBy(_core.TokenParser.tokens.Whitespace.optional).map(_ref5 => {
  let [_openParen, lower, op, eq, key, op2, eq2, upper, _closeParen] = _ref5;
  const lowerKey = op === '<' ? `min-${key}` : `max-${key}`;
  const upperKey = op2 === '<' ? `max-${key}` : `min-${key}`;
  const lowerValue = op === '<' && eq !== '=' ? {
    ...lower,
    value: lower.value + 0.01
  } : op === '>' && eq !== '=' ? {
    ...lower,
    value: lower.value - 0.01
  } : lower;
  const upperValue = op2 === '<' && eq2 !== '=' ? {
    ...upper,
    value: upper.value - 0.01
  } : op2 === '>' && eq2 !== '=' ? {
    ...upper,
    value: upper.value + 0.01
  } : upper;
  const lowerPair = {
    type: 'pair',
    key: lowerKey,
    value: lowerValue
  };
  const upperPair = {
    type: 'pair',
    key: upperKey,
    value: upperValue
  };
  return {
    type: 'and',
    rules: [lowerPair, upperPair]
  };
});
const mediaAndRulesParser = _core.TokenParser.oneOrMore(_core.TokenParser.oneOf(mediaKeywordParser, () => mediaOrRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), notParser, doubleInequalityRuleParser, mediaInequalityRuleParser, simplePairParser, mediaWordRuleParser)).separatedBy(_core.TokenParser.string('and').surroundedBy(_core.TokenParser.tokens.Whitespace)).where(rules => rules.length > 1, 'rules.length > 1').map(rules => rules.length === 1 ? rules[0] : {
  type: 'and',
  rules
});
const mediaOrRulesParser = _core.TokenParser.oneOrMore(_core.TokenParser.oneOf(mediaKeywordParser, () => mediaOrRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), notParser, doubleInequalityRuleParser, mediaInequalityRuleParser, simplePairParser, mediaWordRuleParser)).separatedBy(_core.TokenParser.string('or').surroundedBy(_core.TokenParser.tokens.Whitespace)).where(rules => rules.length > 1, 'rules.length > 1').map(rules => rules.length === 1 ? rules[0] : {
  type: 'or',
  rules
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
        return queries.not ? `not ${queries.key}` : queries.key;
      case 'word-rule':
        return `(${queries.keyValue})`;
      case 'pair':
        {
          const valueStr = Array.isArray(queries.value) ? `${queries.value[0]} / ${queries.value[2]}` : typeof queries.value === 'string' ? queries.value : `${queries.value.value}${queries.value.unit}`;
          return `(${queries.key}: ${valueStr})`;
        }
      case 'not':
        return queries.rule && _isAndOrRule(queries.rule) ? `(not (${this.#toString(queries.rule)}))` : `(not ${this.#toString(queries.rule)})`;
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
          let inner = MediaQuery.normalize(rule.rule);
          while (inner.type === 'not') {
            count++;
            inner = MediaQuery.normalize(inner.rule);
          }
          if (inner.type === 'pair' || inner.type === 'word-rule') {
            return count % 2 === 0 ? inner : {
              type: 'not',
              rule: inner
            };
          }
          return inner;
        }
      default:
        return rule;
    }
  }
  static get parser() {
    const leadingNotParser = _core.TokenParser.sequence(_core.TokenParser.tokens.Ident.map(token => token[4].value, '.stringValue').where(key => key === 'not'), _core.TokenParser.oneOf(() => mediaOrRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), notParser, mediaInequalityRuleParser, simplePairParser, mediaWordRuleParser)).separatedBy(_core.TokenParser.tokens.Whitespace).map(_ref6 => {
      let [_not, queries] = _ref6;
      return queries;
    });
    const normalRuleParser = _core.TokenParser.oneOf(mediaAndRulesParser, mediaOrRulesParser, mediaKeywordParser, notParser, doubleInequalityRuleParser, mediaInequalityRuleParser, simplePairParser, mediaWordRuleParser, () => mediaOrRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen), () => mediaAndRulesParser.surroundedBy(_core.TokenParser.tokens.OpenParen, _core.TokenParser.tokens.CloseParen));
    return _core.TokenParser.sequence(_core.TokenParser.tokens.AtKeyword.where(token => token[4].value === 'media'), _core.TokenParser.oneOrMore(_core.TokenParser.oneOf(leadingNotParser, normalRuleParser)).separatedBy(_core.TokenParser.tokens.Comma.surroundedBy(_core.TokenParser.tokens.Whitespace.optional))).separatedBy(_core.TokenParser.tokens.Whitespace).map(_ref7 => {
      let [_at, querySets] = _ref7;
      const rule = querySets.length > 1 ? {
        type: 'or',
        rules: querySets
      } : querySets[0];
      return new MediaQuery(rule);
    });
  }
}
exports.MediaQuery = MediaQuery;