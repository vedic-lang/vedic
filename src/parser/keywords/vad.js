/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

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

module.exports = new KWvad();
