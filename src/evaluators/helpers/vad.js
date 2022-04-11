const IBase = require('./ibase');

class vad extends IBase {
  interpreteNode (node) {
    this.environment().vad(this.evaluateNode(node.body));
  }
}

module.exports = new vad();
