const IBase = require('./ibase');
const symboltable = require('../../symboltable');

class bhanga extends IBase {
  interpreteNode (node) {
    return symboltable.KW.bhanga;
  }
}

module.exports = new bhanga();
