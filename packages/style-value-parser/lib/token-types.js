"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.TokenList = void 0;
var _cssTokenizer = require("@csstools/css-tokenizer");
class TokenList {
  constructor(input) {
    const iterator = typeof input === 'string' ? (0, _cssTokenizer.tokenizer)({
      css: input
    }) : input;
    this.tokenIterator = iterator;
    this.consumedTokens = [];
    this.currentIndex = 0;
    this.isAtEnd = false;
  }
  consumeNextToken() {
    if (this.currentIndex < this.consumedTokens.length) {
      return this.consumedTokens[this.currentIndex++];
    }
    if (this.isAtEnd) {
      return null;
    }
    if (this.tokenIterator.endOfFile()) {
      this.isAtEnd = true;
      return null;
    }
    const token = this.tokenIterator.nextToken();
    this.consumedTokens.push(token);
    this.currentIndex++;
    if (this.tokenIterator.endOfFile()) {
      this.isAtEnd = true;
    }
    return token;
  }
  peek() {
    if (this.currentIndex < this.consumedTokens.length) {
      return this.consumedTokens[this.currentIndex];
    }
    if (this.isAtEnd || this.tokenIterator.endOfFile()) {
      return null;
    }
    const token = this.tokenIterator.nextToken();
    this.consumedTokens.push(token);
    return token;
  }
  get first() {
    return this.peek();
  }
  setCurrentIndex(newIndex) {
    if (newIndex < this.consumedTokens.length) {
      this.currentIndex = newIndex;
      return;
    }
    while (!this.isAtEnd && !this.tokenIterator.endOfFile() && this.consumedTokens.length <= newIndex) {
      const token = this.tokenIterator.nextToken();
      this.consumedTokens.push(token);
      if (this.tokenIterator.endOfFile()) {
        this.isAtEnd = true;
      }
    }
    this.currentIndex = Math.min(newIndex, this.consumedTokens.length);
  }
  rewind() {
    let positions = arguments.length > 0 && arguments[0] !== undefined ? arguments[0] : 1;
    this.currentIndex = Math.max(0, this.currentIndex - positions);
  }
  get isEmpty() {
    return this.isAtEnd || this.currentIndex >= this.consumedTokens.length && this.tokenIterator.endOfFile();
  }
  getAllTokens() {
    while (!this.isEmpty) {
      this.consumeNextToken();
    }
    return this.consumedTokens;
  }
  slice(start) {
    let end = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : this.currentIndex;
    const initialIndex = this.currentIndex;
    if (start < 0 || end < start) {
      return [];
    }
    this.setCurrentIndex(start);
    const result = [];
    while (this.currentIndex < end) {
      const token = this.consumeNextToken();
      if (token == null) {
        break;
      }
      result.push(token);
    }
    this.setCurrentIndex(initialIndex);
    return result;
  }
}
exports.TokenList = TokenList;