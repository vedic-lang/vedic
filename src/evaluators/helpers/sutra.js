/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const errorhandler = require('../../errorhandler');

class sutra {
  interpreteNode(node) {
    if (
      this.environment().getsutra(this.getCurrentScope(), node.name) !==
      undefined
    ) {
      this.throwError(
        errorhandler.sutraAlreadyExist(node.name, this.getCurrentScope())
      );
    }

    this.environment().setsutra(this.getCurrentScope(), node.name, node);
  }
}

module.exports = new sutra();
