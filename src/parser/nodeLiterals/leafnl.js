/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class LeafNl {
  getNode() {
    return {
      value: this.lexer().next().value,
      left: null,
      right: null,
      operation: null,
    };
  }
}

module.exports = new LeafNl();
