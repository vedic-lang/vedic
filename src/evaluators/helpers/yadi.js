/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class yadi {
  interpreteNode(node) {
    if (this.evaluateNode(node.condition) !== symboltable.KW.asatya) {
      return yadi.runBody(this, node.then);
    } else if (node.else !== undefined) {
      return yadi.runBody(this, node.else);
    }
  }

  static runBody(context, body) {
    if (!(body instanceof Array)) return context.evaluateNode(body);

    for (let i = 0; i < body.length; i++) {
      const returnedValue = context.evaluateNode(body[i]);
      if (returnedValue !== undefined) return returnedValue;
    }
  }
}

module.exports = new yadi();
