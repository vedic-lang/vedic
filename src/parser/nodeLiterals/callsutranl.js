/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class CallsutraNl {
  getNode(sutraNameToken) {
    sutraNameToken = sutraNameToken || {};

    const node = {};
    node.operation = symboltable.CALL_sutra;
    node.name = sutraNameToken.value || this.lexer().next().value;
    node.paramValues = this.parseDelimited(
      symboltable.SYM.L_BRACKET,
      symboltable.SYM.R_BRACKET,
      symboltable.SYM.COMMA,
      this.parseExpression.bind(this),
      null
    );

    if (sutraNameToken.value === undefined) {
      this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
    }

    return node;
  }
}

module.exports = new CallsutraNl();
