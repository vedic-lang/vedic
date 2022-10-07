/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import errorhandler from '../../errorhandler/main.js';

class KWviram {
  getNode() {
    if (KWviram.isExpectedviramStatement(this)) {
      return KWviram.getParsedviramNode(this);
    }

    this.throwError(errorhandler.unexpectedDeclaration(symboltable.KW.viram));
  }

  static isExpectedviramStatement(context) {
    return (
      context.getBlockTypeStack().includes(symboltable.KW.chakra) ||
      context.getBlockTypeStack().includes(symboltable.KW.paryantam)
    );
  }

  static getParsedviramNode(context) {
    const node = {};
    node.operation = context.lexer().next().value;
    context.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);

    return node;
  }
}

export default new KWviram();
