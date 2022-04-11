const IBase = require('./ibase');
const errorhandler = require('../../errorhandler');

class Divide extends IBase {
  interpreteNode (node) {
    const leftNodeValue = this.evaluateNode(node.left);
    const rightNodeValue = this.evaluateNode(node.right);
    if (rightNodeValue === 0) this.throwError(errorhandler.ArithmeticException());

    return leftNodeValue / rightNodeValue;
  }
}

module.exports = new Divide();
