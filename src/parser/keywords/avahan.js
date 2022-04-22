/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const leafnl = require('../nodeLiterals/leafnl');
const path = require('path');
const errorhandler = require('../../errorhandler');

class KWavahan {
  getNode() {
    this.skipKeyword(symboltable.KW.avahan);
    const node = {};
    node.operation = symboltable.KW.avahan;

    if (this.lexer().peek().type === symboltable.STRING) {
      node.path = leafnl.getNode.call(this);
      node.path.value = path.resolve(this.file.dir, node.path.value);
      if (!symboltable.Extensions.includes(node.path.value.split('.').pop())) {
        this.throwError(errorhandler.invalidFileMsg());
      }

      this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
      return node;
    }

    this.lexer().throwError(
      errorhandler.expectStringMsg(symboltable.KW.avahan)
    );
  }
}

module.exports = new KWavahan();
