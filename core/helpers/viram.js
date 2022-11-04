const symboltable = require('../symboltable');

class viram {
  interpreteNode(node) {
    return symboltable.KW.viram;
  }
}

module.exports = new viram();
