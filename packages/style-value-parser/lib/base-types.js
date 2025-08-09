"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.SubString = void 0;
class SubString {
  constructor(str) {
    this.string = str;
    this.startIndex = 0;
    this.endIndex = str.length - 1;
  }
  startsWith(str) {
    for (let i = 0; i < str.length; i++) {
      if (this.startIndex + i > this.endIndex || this.string[this.startIndex + i] !== str[i]) {
        return false;
      }
    }
    return true;
  }
  get first() {
    return this.string[this.startIndex];
  }
  get(relativeIndex) {
    return this.string[this.startIndex + relativeIndex];
  }
  toString() {
    return this.string.slice(this.startIndex, this.endIndex + 1);
  }
  get isEmpty() {
    return this.startIndex > this.endIndex;
  }
}
exports.SubString = SubString;