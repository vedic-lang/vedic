const symboltable = require('../symboltable');

class NotEq {
  interpreteNode(node) {
    return this.evaluateNode(node.left) !== this.evaluateNode(node.right)
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

module.exports = new NotEq();
