const path = require('path');
const symboltable = require('../symboltable');
const nodeLiterals = require('../nodeLiterals');
const errorhandler = require('../errorhandler');

const leafnl = nodeLiterals['NUMBER'];
class KWavahan {
  getNode() {
    this.skipKeyword(symboltable.KW.avahan);
    const node = {};
    node.operation = symboltable.KW.avahan;

    if (this.lexer().peek().type === 'STRING') {
      node.path = leafnl.getNode.call(this);
      node.path.value = path.resolve(this.file.dir, node.path.value);

      if (!symboltable.Extensions.includes(node.path.value.split('.').pop())) {
        this.throwError(errorhandler.invalidFileMsg());
      }

      this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
      return node;
    }

    this.lexer().throwError(
      errorhandler.expectStringMsg(symboltable.KW.avahan)
    );
  }
}

module.exports = new KWavahan();
