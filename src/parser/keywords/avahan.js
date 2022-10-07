/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import leafnl from '../nodeLiterals/leafnl.js';
import path from 'path';
import errorhandler from '../../errorhandler/main.js';

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

export default new KWavahan();
