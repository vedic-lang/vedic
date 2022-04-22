/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const errorhandler = require('../../errorhandler');

class Divide {
  interpreteNode(node) {
    const leftNodeValue = this.evaluateNode(node.left);
    const rightNodeValue = this.evaluateNode(node.right);
    if (rightNodeValue === 0) {
      this.throwError(errorhandler.ArithmeticException());
    }

    return leftNodeValue / rightNodeValue;
  }
}

module.exports = new Divide();
