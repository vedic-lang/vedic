
class vad {
  interpreteNode (node) {
    this.environment().vad(this.evaluateNode(node.body));
  }
}

module.exports = new vad();
