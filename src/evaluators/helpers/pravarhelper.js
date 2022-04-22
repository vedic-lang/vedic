/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class pravarHelper {
  static ispravarVariable(context, maanName) {
    const pravarList = context
      .environment()
      .getmaan(context.getCurrentScope(), symboltable.KW.pravar);
    return pravarList && pravarList.includes(maanName);
  }
}

module.exports = pravarHelper;
