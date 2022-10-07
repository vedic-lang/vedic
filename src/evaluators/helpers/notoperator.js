/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

class NotOperator {
  interpreteNode(node) {
    return this.evaluateNode(node.body) === symboltable.KW.asatya
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

export default new NotOperator();
