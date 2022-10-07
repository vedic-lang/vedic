/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import errorhandler from '../../errorhandler/main.js';

class KWpravar {
  getNode() {
    if (KWpravar.isExpectedpravarStatement(this)) {
      return KWpravar.getParsedpravarNode(this);
    }

    this.throwError(errorhandler.unexpectedDeclaration(symboltable.KW.pravar));
  }

  static isExpectedpravarStatement(context) {
    return context.getBlockTypeStack().includes(symboltable.KW.sutra);
  }

  static getParsedpravarNode(context) {
    context.skipKeyword(symboltable.KW.pravar);
    const node = {};
    node.operation = symboltable.KW.pravar;
    node.varNames = KWpravar.getpravarVarNames(context);
    context.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);

    return node;
  }

  static getpravarVarNames(context) {
    const varTokens = context.parseDelimited(
      '`',
      '`',
      ',',
      context.getTokenThatSatisfiesPredicate.bind(context),
      (token) => token.type === symboltable.VARIABLE
    );
    const varNames = [];
    varTokens.map((varToken) => {
      varNames.push(varToken.value);
    });

    return varNames;
  }
}

export default new KWpravar();
