/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class phala {
  interpreteNode(node) {
    return this.evaluateNode(node.body);
  }
}

export default new phala();
