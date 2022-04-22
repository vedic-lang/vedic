/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const pravarHelper = require('./pravarhelper');
const errorhandler = require('../../errorhandler');
const symboltable = require('../../symboltable');

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

module.exports = new Getmaan();
