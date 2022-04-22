/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class Suchi {
  interpreteNode(suchiNode) {
    const arr = [];

    suchiNode.body.forEach((suchiItemNode) => {
      arr.push(this.evaluateNode(suchiItemNode));
    });

    return arr;
  }
}

module.exports = new Suchi();
