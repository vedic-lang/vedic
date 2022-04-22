/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const bracketExpressionNl = require('../nodeLiterals/bracketnl');

class KWyadi {
  getNode() {
    this.skipKeyword(symboltable.KW.yadi);

    const node = {};
    node.operation = symboltable.KW.yadi;
    node.condition = bracketExpressionNl.getNode.call(this, {
      isArithmeticExpression: false,
      isBracketExpected: true,
    });
    node.then = this.parseBlock(symboltable.KW.yadi);

    if (this.isNextTokenKeyword(symboltable.KW.atha)) {
      node.else = KWyadi.getathaNode(this);
    }

    return node;
  }

  static getathaNode(context) {
    context.skipKeyword(symboltable.KW.atha);

    if (context.isNextTokenKeyword(symboltable.KW.yadi)) {
      return new KWyadi().getNode.call(context);
    }

    return context.parseBlock(symboltable.KW.atha);
  }
}

module.exports = new KWyadi();
