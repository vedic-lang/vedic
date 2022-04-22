/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class chakra {
  interpreteNode(node) {
    this.evaluateNode(node.init);

    while (this.evaluateNode(node.condition) !== symboltable.KW.asatya) {
      for (let i = 0; i < node.body.length; i++) {
        const returnedValue = this.evaluateNode(node.body[i]);
        if (returnedValue === symboltable.KW.viram) return;
        if (returnedValue !== undefined) return returnedValue;
      }

      this.evaluateNode(node.increment);
    }
  }
}

module.exports = new chakra();
