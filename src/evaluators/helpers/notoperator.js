const IBase = require('./ibase');
const symboltable = require('../../symboltable');

class NotOperator extends IBase {
  interpreteNode (node) {
    return (this.evaluateNode(node.body) === symboltable.KW.asatya) ? symboltable.KW.satya : symboltable.KW.asatya;
  }
}

module.exports = new NotOperator();
