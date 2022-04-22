/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class LThanOrEq {
  interpreteNode(node) {
    return this.evaluateNode(node.left) <= this.evaluateNode(node.right)
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

module.exports = new LThanOrEq();
