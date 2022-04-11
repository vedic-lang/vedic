const symboltable = require('../../symboltable');
const BaseNode = require('../basenode');
const variableNlTypes = require('./variablenltypes');

class VariableNl extends BaseNode {
  getNode () {
    const varNameToken = this.lexer().next();

    const nextTokenValue = this.lexer().peek().value;
    if (variableNlTypes[nextTokenValue]) {
      const variableNlType = variableNlTypes[nextTokenValue];
      return variableNlType.getNode.call(this, varNameToken);
    }

    return {
      operation: symboltable.GET_maan,
      name: varNameToken.value
    };
  }
}

module.exports = new VariableNl();
