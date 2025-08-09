/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
import { Frequency } from './frequency';
import { Length } from './length';
import { Resolution } from './resolution';
import { Time } from './time';
export type Dimension = Length | Time | Resolution | Frequency;
export declare const dimension: TokenParser<Dimension>;
