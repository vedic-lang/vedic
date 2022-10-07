/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import errorhandler from '../../errorhandler/main.js';

class NegateExpression {
  interpreteNode(node) {
    const expressionValue = this.evaluateNode(node.body);
    if (typeof expressionValue === 'number') {
      return -parseFloat(expressionValue);
    }

    this.throwError(errorhandler.cannotNegateMsg(expressionValue));
  }
}

export default new NegateExpression();
