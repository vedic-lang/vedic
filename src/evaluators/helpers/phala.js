const IBase = require('./ibase');

class phala extends IBase {
  interpreteNode (node) {
    return this.evaluateNode(node.body);
  }
}

module.exports = new phala();
