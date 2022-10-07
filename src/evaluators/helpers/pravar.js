/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

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

export default new pravar();
