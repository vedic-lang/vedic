/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

class GThan {
  interpreteNode(node) {
    return this.evaluateNode(node.left) > this.evaluateNode(node.right)
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

export default new GThan();
