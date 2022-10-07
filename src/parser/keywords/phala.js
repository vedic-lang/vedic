/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import errorhandler from '../../errorhandler/main.js';

class KWphala {
  getNode() {
    if (KWphala.isExpectedphalaStatement(this)) {
      return KWphala.getParsedphalaNode(this);
    }

    this.throwError(errorhandler.unexpectedDeclaration(symboltable.KW.phala));
  }

  static isExpectedphalaStatement(context) {
    return context.getBlockTypeStack().includes(symboltable.KW.sutra);
  }

  static getParsedphalaNode(context) {
    context.skipKeyword(symboltable.KW.phala);
    const node = {};
    node.operation = symboltable.KW.phala;
    node.body = context.parseExpression();
    context.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);

    return node;
  }
}

export default new KWphala();
