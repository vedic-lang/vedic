const symboltable = require('../symboltable');

const nodeLiterals = require('../nodeLiterals');
const bracketExpressionNl = nodeLiterals['EXP_PUNC'][symboltable.SYM.L_BRACKET];

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
