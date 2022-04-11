const IBase = require('./ibase');
class Suchi extends IBase {
  interpreteNode (suchiNode) {
    const arr = [];

    suchiNode.body.forEach(suchiItemNode => {
      arr.push(this.evaluateNode(suchiItemNode));
    });

    return arr;
  }
}

module.exports = new Suchi();
