const symboltable = require('../symboltable');

class LThanOrEq {
  interpreteNode(node) {
    return this.evaluateNode(node.left) <= this.evaluateNode(node.right)
      ? symboltable.KW.satya
      : symboltable.KW.asatya;
  }
}

module.exports = new LThanOrEq();
