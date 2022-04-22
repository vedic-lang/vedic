/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const errorhandler = require('../../errorhandler');

class NegateExpression {
  interpreteNode(node) {
    const expressionValue = this.evaluateNode(node.body);
    if (typeof expressionValue === 'number') {
      return -parseFloat(expressionValue);
    }

    this.throwError(errorhandler.cannotNegateMsg(expressionValue));
  }
}

module.exports = new NegateExpression();
