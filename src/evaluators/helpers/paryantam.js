const IBase = require('./ibase');
const symboltable = require('../../symboltable');

class paryantam extends IBase {
  interpreteNode (node) {
    while (this.evaluateNode(node.condition) !== symboltable.KW.asatya) {
      for (let i = 0; i < node.body.length; i++) {
        const returnedValue = this.evaluateNode(node.body[i]);
        if (returnedValue === symboltable.KW.bhanga) return;
        if (returnedValue !== undefined) return returnedValue;
      }
    }
  }
}

module.exports = new paryantam();
