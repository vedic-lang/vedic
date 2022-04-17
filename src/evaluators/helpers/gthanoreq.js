
const symboltable = require('../../symboltable');

class GThanOrEq {
  interpreteNode (node) {
    return this.evaluateNode(node.left) >= this.evaluateNode(node.right) ? symboltable.KW.satya : symboltable.KW.asatya;
  }
}

module.exports = new GThanOrEq();
