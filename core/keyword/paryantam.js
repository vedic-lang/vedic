const symboltable = require('../symboltable');

const nodeLiterals = require('../nodeLiterals');
const bracketExpressionNl = nodeLiterals['EXP_PUNC'][symboltable.SYM.L_BRACKET];

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
