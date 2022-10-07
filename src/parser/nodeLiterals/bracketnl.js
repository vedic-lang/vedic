/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

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

export default new BracketExpressionNl();
