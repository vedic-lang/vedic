const IBase = require('./ibase');
const symboltable = require('../../symboltable');

class pravar extends IBase {
  interpreteNode (node) {
    let pravar = this.environment().getmaan(this.getCurrentScope(), symboltable.KW.pravar);

    if (!pravar) pravar = node.varNames;
    else pravar.push(...node.varNames);

    this.environment().setmaan(this.getCurrentScope(), symboltable.KW.pravar, pravar);
  }
}

module.exports = new pravar();
