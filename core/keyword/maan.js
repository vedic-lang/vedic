const symboltable = require('../symboltable');
const nodeLiterals = require('../nodeLiterals');
const variableNl = nodeLiterals['VARIABLE'];
const errorhandler = require('../errorhandler');

class KWmaan {
  getNode(config) {
    config = config || { shouldExpectTerminator: true };

    this.skipKeyword(symboltable.KW.maan);

    const node = {};
    node.operation = symboltable.SYM.ASSIGN;
    const varNode = variableNl.getNode.call(this);
    if (varNode.operation === 'CALL_SUTRA') {
      this.throwError(errorhandler.invalidAssignment());
    }
    node.left = varNode.operation === 'GET_MAAN' ? varNode.name : varNode;
    this.skipOperator(symboltable.SYM.ASSIGN);
    node.right = this.parseExpression();
    if (config.shouldExpectTerminator) {
      this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
    }

    return node;
  }
}

module.exports = new KWmaan();
