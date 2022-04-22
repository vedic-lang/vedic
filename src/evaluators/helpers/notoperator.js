/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class NotOperator {
  interpreteNode(node) {
    return this.evaluateNode(node.body) === symboltable.KW.asatya
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

module.exports = new NotOperator();
