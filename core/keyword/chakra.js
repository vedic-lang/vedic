const symboltable = require('../symboltable');

const keywordmaan = require('./maan');
const errorhandler = require('../errorhandler');
const nodeLiterals = require('../nodeLiterals');
const bracketExpressionNl = nodeLiterals['EXP_PUNC'][symboltable.SYM.L_BRACKET];

class KWchakra {
  getNode() {
    this.skipKeyword(symboltable.KW.chakra);

    this.skipPunctuation(symboltable.SYM.L_BRACKET);
    const node = {};
    node.operation = symboltable.KW.chakra;
    node.init = keywordmaan.getNode.call(this);
    node.condition = bracketExpressionNl.getNode.call(this, {
      isArithmeticExpression: false,
      isBracketExpected: false,
    });

    this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
    node.increment = keywordmaan.getNode.call(this, {
      shouldExpectTerminator: false,
    });

    if (KWchakra.isInValidchakraIncrementStatement(node)) {
      this.throwError(errorhandler.chakraIncrementAndDecrementMsg());
    }
    this.skipPunctuation(symboltable.SYM.R_BRACKET);

    node.body = this.parseBlock(symboltable.KW.chakra);

    return node;
  }

  static isInValidchakraIncrementStatement(chakraNode) {
    const incrementNode = chakraNode.increment.right;

    if (
      [symboltable.SYM.PLUS, symboltable.SYM.MINUS].includes(
        incrementNode.operation
      )
    ) {
      if (
        [incrementNode.left.name, incrementNode.right.name].includes(
          chakraNode.init.left
        )
      ) {
        return false;
      }
    }

    return true;
  }
}

module.exports = new KWchakra();
