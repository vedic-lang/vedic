/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const keywordmaan = require('./maan');
const errorhandler = require('../../errorhandler');
const bracketExpressionNl = require('../nodeLiterals/bracketnl');

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
      // make sure there is variable 'i' in atleast one child of the incrementNode
      // i.e maan i = i + 1 or maan i = 1 + i or maan i = i + i
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
