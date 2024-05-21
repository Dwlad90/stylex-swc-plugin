"use strict";
/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */
Object.defineProperty(exports, "__esModule", { value: true });
const constants_1 = require("./constants");
function stylexLoader(inputCode) {
    const callback = this.async();
    const { stylexPlugin } = this.getOptions();
    const logger = this._compiler?.getInfrastructureLogger(constants_1.PLUGIN_NAME);
    stylexPlugin?.transformCode(inputCode, this.resourcePath, logger).then(({ code, map }) => {
        callback(null, code, map);
    }, (error) => {
        callback(error);
    });
}
exports.default = stylexLoader;
module.exports = stylexLoader;
