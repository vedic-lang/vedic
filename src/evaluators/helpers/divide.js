/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import errorhandler from '../../errorhandler/main.js';

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

export default new Divide();
