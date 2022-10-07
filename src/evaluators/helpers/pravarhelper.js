/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

class pravarHelper {
  static ispravarVariable(context, maanName) {
    const pravarList = context
      .environment()
      .getmaan(context.getCurrentScope(), symboltable.KW.pravar);
    return pravarList && pravarList.includes(maanName);
  }
}

export default pravarHelper;
