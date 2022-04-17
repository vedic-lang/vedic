const symboltable = require('../../symboltable');

class bhanga {
  interpreteNode (node) {
    return symboltable.KW.bhanga;
  }
}

module.exports = new bhanga();
