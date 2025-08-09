/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
import { Length } from './length';
import { Percentage } from './common-types';
export type LengthPercentage = Length | Percentage;
export declare const lengthPercentage: TokenParser<LengthPercentage>;
