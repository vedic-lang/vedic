/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const bracketExpressionNl = require('../nodeLiterals/bracketnl');

class KWnirdesa {
  getNode() {
    const node = {};
    node.operation = symboltable.KW.nirdesa;
    this.pushToBlockTypeStack(symboltable.KW.nirdesa);
    this.skipKeyword(symboltable.KW.nirdesa);
    node.nirdesavalue = bracketExpressionNl.getNode.call(this);
    this.skipPunctuation(symboltable.SYM.L_PAREN);
    node.nirdesabody = KWnirdesa.getnirdesaBody(this);
    node.yadabhave = KWnirdesa.getyadabhave(this);
    this.skipPunctuation(symboltable.SYM.R_PAREN);
    this.popBlockTypeStack();

    return node;
  }

  static getnirdesaBody(context) {
    const nirdesaBody = [];
    const keywordyada = new KWyada();

    while (KWnirdesa.isNextTokenyada(context)) {
      nirdesaBody.push(keywordyada.getNode.call(context));
    }

    return nirdesaBody;
  }

  static isNextTokenyada(context) {
    return (
      context.isNotEndOfFile() &&
      context.lexer().peek().value === symboltable.KW.yada
    );
  }

  static getyadabhave(context) {
    const yadabhave = [];

    if (context.isNextTokenKeyword(symboltable.KW.yadabhave)) {
      context.skipKeyword(symboltable.KW.yadabhave);
      context.skipPunctuation(symboltable.SYM.COLON);

      while (context.isNotEndOfBlock()) {
        yadabhave.push(context.parseAst());
      }
    }

    return yadabhave;
  }
}

class KWyada {
  getNode() {
    const node = {};
    node.operation = symboltable.KW.yada;
    this.skipKeyword(symboltable.KW.yada);
    node.yadavalue = this.parseExpression();
    this.skipPunctuation(symboltable.SYM.COLON);
    node.yadabody = KWyada.getyadaBody(this);

    return node;
  }

  static getyadaBody(context) {
    const yadaBody = [];

    while (KWyada.canParseyadaStatements(context)) {
      yadaBody.push(context.parseAst());
    }

    return yadaBody;
  }

  static canParseyadaStatements(context) {
    return (
      context.isNotEndOfFile() &&
      context.lexer().peek().value !== symboltable.KW.yada &&
      context.lexer().peek().value !== symboltable.KW.yadabhave &&
      context.lexer().peek().value !== symboltable.SYM.R_PAREN
    );
  }
}

module.exports = new KWnirdesa();
