"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.lastMediaQueryWinsTransform = lastMediaQueryWinsTransform;
var _mediaQuery = require("./media-query.js");
function lastMediaQueryWinsTransform(styles) {
  return dfsProcessQueries(styles, 0);
}
function combineMediaQueryWithNegations(current, negations) {
  if (negations.length === 0) {
    return current;
  }
  let combinedAst;
  if (current.queries.type === 'or') {
    combinedAst = {
      type: 'or',
      rules: current.queries.rules.map(rule => ({
        type: 'and',
        rules: [rule, ...negations.map(mq => ({
          type: 'not',
          rule: mq.queries
        }))]
      }))
    };
  } else {
    combinedAst = {
      type: 'and',
      rules: [current.queries, ...negations.map(mq => ({
        type: 'not',
        rule: mq.queries
      }))]
    };
  }
  return new _mediaQuery.MediaQuery(combinedAst);
}
function dfsProcessQueries(obj, depth) {
  const result = {};
  Object.entries(obj).forEach(_ref => {
    let [key, value] = _ref;
    if (typeof value === 'object' && value !== null) {
      result[key] = dfsProcessQueries(value, depth + 1);
    } else {
      result[key] = value;
    }
  });
  if (depth >= 2 && Object.keys(result).some(key => key.startsWith('@media '))) {
    const mediaKeys = Object.keys(result).filter(key => key.startsWith('@media '));
    const negations = [];
    const accumulatedNegations = [];
    for (let i = mediaKeys.length - 1; i > 0; i--) {
      const mediaQuery = _mediaQuery.MediaQuery.parser.parseToEnd(mediaKeys[i]);
      negations.push(mediaQuery);
      accumulatedNegations.push([...negations]);
    }
    accumulatedNegations.reverse();
    accumulatedNegations.push([]);
    for (let i = 0; i < mediaKeys.length; i++) {
      const currentKey = mediaKeys[i];
      const currentValue = result[currentKey];
      const baseMediaQuery = _mediaQuery.MediaQuery.parser.parseToEnd(currentKey);
      const reversedNegations = [...accumulatedNegations[i]].reverse();
      const combinedQuery = combineMediaQueryWithNegations(baseMediaQuery, reversedNegations);
      const newMediaKey = combinedQuery.toString();
      delete result[currentKey];
      result[newMediaKey] = currentValue;
    }
  }
  return result;
}