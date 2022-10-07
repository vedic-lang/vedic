/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

class KWvad {
  getNode() {
    this.skipKeyword(symboltable.KW.vad);
    const node = {};
    node.operation = symboltable.KW.vad;
    node.body = this.parseExpression();
    this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);

    return node;
  }
}

export default new KWvad();
