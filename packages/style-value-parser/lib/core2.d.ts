/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import type {
  CSSToken,
  TokenAtKeyword,
  TokenBadString,
  TokenBadURL,
  TokenCDC,
  TokenCDO,
  TokenColon,
  TokenComma,
  TokenComment,
  TokenDelim,
  TokenDimension,
  TokenEOF,
  TokenFunction,
  TokenHash,
  TokenIdent,
  TokenNumber,
  TokenPercentage,
  TokenSemicolon,
  TokenString,
  TokenURL,
  TokenWhitespace,
  TokenOpenParen,
  TokenCloseParen,
  TokenOpenSquare,
  TokenCloseSquare,
  TokenOpenCurly,
  TokenCloseCurly,
  TokenUnicodeRange,
} from '@csstools/css-tokenizer';
import { TokenList } from './token-types';
import { TokenType } from '@csstools/css-tokenizer';
type TokenNameToTokenType = {
  Comment: TokenComment;
  AtKeyword: TokenAtKeyword;
  BadString: TokenBadString;
  BadURL: TokenBadURL;
  CDC: TokenCDC;
  CDO: TokenCDO;
  Colon: TokenColon;
  Comma: TokenComma;
  Delim: TokenDelim;
  Dimension: TokenDimension;
  EOF: TokenEOF;
  Function: TokenFunction;
  Hash: TokenHash;
  Ident: TokenIdent;
  Number: TokenNumber;
  Percentage: TokenPercentage;
  Semicolon: TokenSemicolon;
  String: TokenString;
  URL: TokenURL;
  Whitespace: TokenWhitespace;
  OpenParen: TokenOpenParen;
  CloseParen: TokenCloseParen;
  OpenSquare: TokenOpenSquare;
  CloseSquare: TokenCloseSquare;
  OpenCurly: TokenOpenCurly;
  CloseCurly: TokenCloseCurly;
  UnicodeRange: TokenUnicodeRange;
};
export declare class TokenParser<T> {
  readonly run: (input: TokenList) => T | Error;
  readonly label: string;
  constructor(parser: (input: TokenList) => T | Error, label: string);
  parse(css: string): T | Error;
  parseToEnd(css: string): T;
  map<NewT>(f: (value: T) => NewT, label?: string): TokenParser<NewT>;
  flatMap<U>(f: (value: T) => TokenParser<U>, label?: string): TokenParser<U>;
  or<U>(parser2: TokenParser<U>): TokenParser<T | U>;
  surroundedBy(
    prefix: TokenParser<unknown>,
    suffix: TokenParser<unknown>,
  ): TokenParser<T>;
  skip(skipParser: TokenParser<unknown>): TokenParser<T>;
  get optional(): TokenParser<void | T>;
  prefix(prefixParser: TokenParser<unknown>): TokenParser<T>;
  suffix(suffixParser: TokenParser<unknown>): TokenParser<T>;
  where<Refined extends T = T>(
    predicate: (value: T) => value is Refined,
    label?: string,
  ): TokenParser<Refined>;
  toString(): string;
  static never<T>(): TokenParser<T>;
  static always<T>(output: T): TokenParser<T>;
  static token<TT extends CSSToken>(
    tokenType: TT[0],
    label: string,
  ): TokenParser<TT>;
  static string<S extends string>(str: S): TokenParser<S>;
  static fn(name: string): TokenParser<string>;
  static tokens: {
    [Key in keyof typeof TokenType]: TokenParser<TokenNameToTokenType[Key]>;
  };
  static oneOf<T>(
    ...parsers: ReadonlyArray<TokenParser<T> | (() => TokenParser<T>)>
  ): TokenParser<T>;
  static sequence<T extends ConstrainedTuple<TokenParser<unknown>>>(
    ...parsers: T
  ): TokenParserSequence<T>;
  static setOf<T extends ConstrainedTuple<TokenParser<unknown>>>(
    ...parsers: T
  ): TokenParserSet<T>;
  static zeroOrMore<T>(parser: TokenParser<T>): TokenZeroOrMoreParsers<T>;
  static oneOrMore<T>(parser: TokenParser<T>): TokenOneOrMoreParsers<T>;
}
declare class TokenZeroOrMoreParsers<T> extends TokenParser<ReadonlyArray<T>> {
  readonly parser: TokenParser<T>;
  readonly separator: null | undefined | TokenParser<void>;
  constructor(parser: TokenParser<T>, separator?: TokenParser<void>);
  separatedBy(separator: TokenParser<unknown>): TokenZeroOrMoreParsers<T>;
}
export declare class TokenOneOrMoreParsers<T> extends TokenParser<
  ReadonlyArray<T>
> {
  readonly parser: TokenParser<T>;
  readonly separator: null | undefined | TokenParser<void>;
  constructor(parser: TokenParser<T>, separator?: TokenParser<void>);
  separatedBy(separator: TokenParser<unknown>): TokenOneOrMoreParsers<T>;
}
declare class TokenParserSequence<
  T extends ConstrainedTuple<TokenParser<unknown>>,
> extends TokenParser<ValuesFromParserTuple<T>> {
  readonly parsers: T;
  readonly separator: null | undefined | TokenParser<void>;
  constructor(parsers: T, _separator?: TokenParser<unknown>);
  separatedBy(separator: TokenParser<unknown>): TokenParserSequence<T>;
}
declare class TokenParserSet<
  T extends ConstrainedTuple<TokenParser<unknown>>,
> extends TokenParser<ValuesFromParserTuple<T>> {
  readonly parsers: T;
  readonly separator: null | undefined | TokenParser<void>;
  constructor(_parsers: T, separator?: null | undefined | TokenParser<void>);
  separatedBy(separator: TokenParser<unknown>): TokenParserSet<T>;
}
type ConstrainedTuple<T> =
  | Readonly<[T]>
  | Readonly<[T, T]>
  | Readonly<[T, T, T]>
  | Readonly<[T, T, T, T]>
  | Readonly<[T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T, T, T, T, T, T]>
  | Readonly<[T, T, T, T, T, T, T, T, T, T, T, T]>;
export type FromParser<
  T extends TokenParser<unknown>,
  Fallback = /**
   * > 168 | export type FromParser<+T: TokenParser<mixed>, Fallback = empty> =
   *       |                                                           ^^^^^ Unsupported feature: Translating "empty type" is currently not supported.
   **/
  any,
> =
  Fallback | T extends TokenParser<infer V>
    ? V
    : /**
       * > 170 |   | T extends TokenParser<infer V> ? V : empty;
       *       |                                          ^^^^^ Unsupported feature: Translating "empty type" is currently not supported.
       **/
      any;
type ValuesFromParserTuple<
  T extends ConstrainedTuple<TokenParser<unknown>>,
  Fallback = /**
   * > 174 |   Fallback = empty,
   *       |              ^^^^^ Unsupported feature: Translating "empty type" is currently not supported.
   **/
  any,
> = { [Key in keyof T]: FromParser<T[Key], Fallback> };
