/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import leafNl from './leafnl.js';
import errorhandler from '../../errorhandler/main.js';

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

export default new KWNl();
