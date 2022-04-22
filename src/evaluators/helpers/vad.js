/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class vad {
  interpreteNode(node) {
    this.environment().vad(this.evaluateNode(node.body));
  }
}

module.exports = new vad();
