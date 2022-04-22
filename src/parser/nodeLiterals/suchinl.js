/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class SuchiNl {
  getNode(suchiNameToken) {
    return !suchiNameToken
      ? SuchiNl.getParsedSuchiLiteral(this)
      : SuchiNl.getParsedSuchiElement(this, suchiNameToken);
  }

  static getParsedSuchiLiteral(context) {
    const node = {};
    node.operation = symboltable.SUCHI;
    node.body = context.parseDelimited(
      symboltable.SYM.L_SQ_BRACKET,
      symboltable.SYM.R_SQ_BRACKET,
      symboltable.SYM.COMMA,
      context.parseExpression.bind(context),
      null
    );

    return node;
  }

  static getParsedSuchiElement(context, suchiNameToken) {
    const node = {};
    node.operation = symboltable.SUCHI_ELEM;
    node.name = suchiNameToken.value;
    node.indexNodes = SuchiNl.getSuchiElementIndexNodes(context);

    return node;
  }

  static getSuchiElementIndexNodes(context) {
    const indexNodes = [SuchiNl.getSuchiElementIndexNode(context)];

    while (context.isNextTokenPunctuation(symboltable.SYM.L_SQ_BRACKET)) {
      indexNodes.push(SuchiNl.getSuchiElementIndexNode(context));
    }

    return indexNodes;
  }

  static getSuchiElementIndexNode(context) {
    let indexNode = { operation: null, right: null, left: null, value: '' };

    context.skipPunctuation(symboltable.SYM.L_SQ_BRACKET);
    if (SuchiNl.isNotEmptySuchiIndex(context)) {
      indexNode = context.parseExpression();
    }
    context.skipPunctuation(symboltable.SYM.R_SQ_BRACKET);

    return indexNode;
  }

  static isNotEmptySuchiIndex(context) {
    return context.lexer().peek().value !== symboltable.SYM.R_SQ_BRACKET;
  }
}

module.exports = new SuchiNl();
