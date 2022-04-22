/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const errorhandler = require('../../errorhandler');

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

module.exports = new KWviram();
