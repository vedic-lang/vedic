/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class pravar {
  interpreteNode(node) {
    let pravar = this.environment().getmaan(
      this.getCurrentScope(),
      symboltable.KW.pravar
    );

    if (!pravar) pravar = node.varNames;
    else pravar.push(...node.varNames);

    this.environment().setmaan(
      this.getCurrentScope(),
      symboltable.KW.pravar,
      pravar
    );
  }
}

module.exports = new pravar();
