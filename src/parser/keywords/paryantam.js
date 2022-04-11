const symboltable = require('../../symboltable');
const BaseNode = require('../basenode');
const bracketExpressionNl = require('../nodeLiterals/bracketnl');
class KWparyantam extends BaseNode {
  constructor () {
    super();
  }

  getNode () {
    this.skipKeyword(symboltable.KW.paryantam);

    return {
      operation: symboltable.KW.paryantam,
      condition: bracketExpressionNl.getNode.call(this, false),
      body: this.parseBlock(symboltable.KW.paryantam)
    };
  }
}

module.exports = new KWparyantam();
