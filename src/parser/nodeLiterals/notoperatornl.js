const BaseNode = require('../basenode');

class NotOperatorNl extends BaseNode {
  getNode () {
    return {
      operation: this.lexer().next().value,
      body: this.parseExpression()
    };
  }
}

module.exports = new NotOperatorNl();
