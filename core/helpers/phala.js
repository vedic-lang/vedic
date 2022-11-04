class phala {
  interpreteNode(node) {
    return this.evaluateNode(node.body);
  }
}

module.exports = new phala();
