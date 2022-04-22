/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');
const leafNl = require('./leafnl');

const errorhandler = require('../../errorhandler');

class KWNl {
  getNode() {
    if (KWNl.isBooleanKeywordNl(this)) {
      return leafNl.getNode.call(this);
    }

    this.throwError(errorhandler.expectBooleanMsg());
  }

  static isBooleanKeywordNl(context) {
    return [symboltable.KW.satya, symboltable.KW.asatya].includes(
      context.lexer().peek().value
    );
  }
}

module.exports = new KWNl();
