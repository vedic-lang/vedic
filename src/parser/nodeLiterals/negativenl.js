/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

class NegativeNl {
  getNode() {
    this.skipOperator(symboltable.SYM.MINUS);

    return {
      operation: symboltable.NEGATIVE_EXPRESSION,
      body: this.parseExpression(),
    };
  }
}

export default new NegativeNl();
