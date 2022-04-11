const IBase = require('./ibase');
const symboltable = require('../../symboltable');

class NotEq extends IBase {
  interpreteNode (node) {
    return this.evaluateNode(node.left) !== this.evaluateNode(node.right) ? symboltable.KW.satya : symboltable.KW.asatya;
  }
}

module.exports = new NotEq();
