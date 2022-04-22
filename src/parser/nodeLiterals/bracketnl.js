/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class BracketExpressionNl {
  getNode(config) {
    config = config || {
      isArithmeticExpression: true,
      isBracketExpected: true,
    };

    if (config.isBracketExpected) {
      this.skipPunctuation(symboltable.SYM.L_BRACKET);
    }
    this.setIsArithmeticExpression(config.isArithmeticExpression);
    const node = this.parseExpression();
    this.setIsArithmeticExpression(true);
    if (config.isBracketExpected) {
      this.skipPunctuation(symboltable.SYM.R_BRACKET);
    }

    return node;
  }
}

module.exports = new BracketExpressionNl();
