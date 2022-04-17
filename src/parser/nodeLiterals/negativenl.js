const symboltable = require('../../symboltable');

class NegativeNl {
  getNode () {
    this.skipOperator(symboltable.SYM.MINUS);

    return {
      operation: symboltable.NEGATIVE_EXPRESSION,
      body: this.parseExpression()
    };
  }
}

module.exports = new NegativeNl();
