const symboltable = require('../../symboltable');
const leafNl = require('./leafnl');
const BaseNode = require('../basenode');
const errorhandler = require('../../errorhandler');

class KWNl extends BaseNode {
  constructor () {
    super();
  }

  getNode () {
    if (KWNl.isBooleanKeywordNl(this)) {
      return leafNl.getNode.call(this);
    }

    this.throwError(errorhandler.expectBooleanMsg());
  }

  static isBooleanKeywordNl (context) {
    return [symboltable.KW.satya, symboltable.KW.asatya].includes(context.lexer().peek().value);
  }
}

module.exports = new KWNl();
