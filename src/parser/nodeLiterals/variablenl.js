/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import variableNlTypes from './variablenltypes.js';

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

export default new VariableNl();
