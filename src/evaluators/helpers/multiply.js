/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class Multiply {
  interpreteNode(node) {
    return this.evaluateNode(node.left) * this.evaluateNode(node.right);
  }
}

module.exports = new Multiply();
