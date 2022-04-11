const IBase = require('./ibase');

class Multiply extends IBase {
  interpreteNode (node) {
    return this.evaluateNode(node.left) * this.evaluateNode(node.right);
  }
}

module.exports = new Multiply();
