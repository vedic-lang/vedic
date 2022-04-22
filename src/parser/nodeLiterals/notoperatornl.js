/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class NotOperatorNl {
  getNode() {
    return {
      operation: this.lexer().next().value,
      body: this.parseExpression(),
    };
  }
}

module.exports = new NotOperatorNl();
