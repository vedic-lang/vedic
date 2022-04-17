/*
 * ॥ श्री गणेशाय नमः ॥ 
 * © Copyright 2022 @ptprashanttripathi 
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

class bhanga {
  interpreteNode (node) {
    return symboltable.KW.bhanga;
  }
}

module.exports = new bhanga();
