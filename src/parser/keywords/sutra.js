/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const errorhandler = require('../../errorhandler');

class KWsutra {
  getNode() {
    if (KWsutra.isExpectedIseDeclaration(this)) {
      return KWsutra.getParsedsutraNode(this);
    }

    this.throwError(errorhandler.unexpectedDeclaration(symboltable.KW.sutra));
  }

  static isExpectedIseDeclaration(context) {
    return (
      context.getBlockTypeStack().length === 0 ||
      context.peekBlockTypeStack() === symboltable.PROGRAM ||
      context.peekBlockTypeStack() === symboltable.KW.sutra
    );
  }

  static getParsedsutraNode(context) {
    context.skipKeyword(symboltable.KW.sutra);

    return {
      operation: symboltable.KW.sutra,
      name: context.parseVarname(),
      paramTokens: context.parseDelimited(
        symboltable.SYM.L_BRACKET,
        symboltable.SYM.R_BRACKET,
        symboltable.SYM.COMMA,
        context.getTokenThatSatisfiesPredicate.bind(context),
        (token) => token.type === symboltable.VARIABLE
      ),
      body: context.parseBlock(symboltable.KW.sutra),
    };
  }
}

module.exports = new KWsutra();
