/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const variableNl = require('../nodeLiterals/variablenl');
const errorhandler = require('../../errorhandler');

class KWmaan {
  getNode(config) {
    config = config || { shouldExpectTerminator: true };

    this.skipKeyword(symboltable.KW.maan);

    const node = {};
    node.operation = symboltable.SYM.ASSIGN;
    const varNode = variableNl.getNode.call(this);
    if (varNode.operation === symboltable.CALL_sutra) {
      this.throwError(errorhandler.invalidAssignment());
    }
    node.left =
      varNode.operation === symboltable.GET_maan ? varNode.name : varNode;
    this.skipOperator(symboltable.SYM.ASSIGN);
    node.right = this.parseExpression();
    if (config.shouldExpectTerminator) {
      this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
    }

    return node;
  }
}

module.exports = new KWmaan();
