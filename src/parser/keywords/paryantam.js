/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import bracketExpressionNl from '../nodeLiterals/bracketnl.js';
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

export default new KWparyantam();
