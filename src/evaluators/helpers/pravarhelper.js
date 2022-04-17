const symboltable = require('../../symboltable');

class pravarHelper {
  static ispravarVariable (context, maanName) {
    const pravarList = context
      .environment()
      .getmaan(context.getCurrentScope(), symboltable.KW.pravar);
    return pravarList && pravarList.includes(maanName);
  }
}

module.exports = pravarHelper;
