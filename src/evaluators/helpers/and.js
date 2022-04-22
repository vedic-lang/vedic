/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class And {
  interpreteNode(node) {
    return this.evaluateNode(node.left) !== symboltable.KW.asatya &&
      this.evaluateNode(node.right) !== symboltable.KW.asatya
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

module.exports = new And();
