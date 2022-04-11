const BaseNode = require('../basenode');
const symboltable = require('../../symboltable');

class BracketExpressionNl extends BaseNode {
  getNode (config) {
    config = config || { isArithmeticExpression: true, isBracketExpected: true };

    if (config.isBracketExpected) this.skipPunctuation(symboltable.SYM.L_BRACKET);
    this.setIsArithmeticExpression(config.isArithmeticExpression);
    const node = this.parseExpression();
    this.setIsArithmeticExpression(true);
    if (config.isBracketExpected) this.skipPunctuation(symboltable.SYM.R_BRACKET);

    return node;
  }
}

module.exports = new BracketExpressionNl();
