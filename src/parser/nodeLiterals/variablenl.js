/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const variableNlTypes = require('./variablenltypes');

class VariableNl {
  getNode() {
    const varNameToken = this.lexer().next();

    const nextTokenValue = this.lexer().peek().value;
    if (variableNlTypes[nextTokenValue]) {
      const variableNlType = variableNlTypes[nextTokenValue];
      return variableNlType.getNode.call(this, varNameToken);
    }

    return {
      operation: symboltable.GET_maan,
      name: varNameToken.value,
    };
  }
}

module.exports = new VariableNl();
