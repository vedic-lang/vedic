const IBase = require('./ibase');

class Minus extends IBase {
  interpreteNode (node) {
    return this.evaluateNode(node.left) - this.evaluateNode(node.right);
  }
}

module.exports = new Minus();
