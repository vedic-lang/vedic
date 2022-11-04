class Remainder {
  interpreteNode(node) {
    return this.evaluateNode(node.left) % this.evaluateNode(node.right);
  }
}

module.exports = new Remainder();
