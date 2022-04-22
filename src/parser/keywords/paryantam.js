/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const bracketExpressionNl = require('../nodeLiterals/bracketnl');
class KWparyantam {
  getNode() {
    this.skipKeyword(symboltable.KW.paryantam);

    return {
      operation: symboltable.KW.paryantam,
      condition: bracketExpressionNl.getNode.call(this, false),
      body: this.parseBlock(symboltable.KW.paryantam),
    };
  }
}

module.exports = new KWparyantam();
