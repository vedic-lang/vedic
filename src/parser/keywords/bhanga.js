/*
 * ॥ श्री गणेशाय नमः ॥ 
 * © Copyright 2022 @ptprashanttripathi 
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const errorhandler = require('../../errorhandler');

class KWbhanga {
  getNode () {
    if (KWbhanga.isExpectedbhangaStatement(this)) {
      return KWbhanga.getParsedbhangaNode(this);
    }

    this.throwError(errorhandler.unexpectedDeclaration(symboltable.KW.bhanga));
  }

  static isExpectedbhangaStatement (context) {
    return (
      context.getBlockTypeStack().includes(symboltable.KW.chakra) ||
      context.getBlockTypeStack().includes(symboltable.KW.paryantam)
    );
  }

  static getParsedbhangaNode (context) {
    const node = {};
    node.operation = context.lexer().next().value;
    context.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);

    return node;
  }
}

module.exports = new KWbhanga();
