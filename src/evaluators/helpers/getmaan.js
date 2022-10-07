/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import pravarHelper from './pravarhelper.js';
import errorhandler from '../../errorhandler/main.js';
import symboltable from '../../symboltable/main.js';

class Getmaan {
  interpreteNode(node) {
    for (
      let index = Getmaan.getTopIndex(this, node.name);
      index >= 0;
      index--
    ) {
      if (
        this.environment().getmaan(this.scopeStack()[index], node.name) !==
        undefined
      ) {
        return this.environment().getmaan(this.scopeStack()[index], node.name);
      }
    }

    this.throwError(
      errorhandler.varDoesNotExist(symboltable.VARIABLE, node.name)
    );
  }

  static getTopIndex(context, maanName) {
    if (pravarHelper.ispravarVariable(context, maanName)) {
      return context.scopeStack().length - 2;
    }

    return context.scopeStack().length - 1;
  }
}

export default new Getmaan();
