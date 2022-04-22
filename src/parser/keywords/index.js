/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');

const keywords = {};
keywords[symboltable.KW.maan] = require('./maan');
keywords[symboltable.KW.vad] = require('./vad');
keywords[symboltable.KW.yadi] = require('./yadi');
keywords[symboltable.KW.paryantam] = require('./paryantam');
keywords[symboltable.KW.chakra] = require('./chakra');
keywords[symboltable.KW.phala] = require('./phala');
keywords[symboltable.KW.viram] = require('./viram');
keywords[symboltable.KW.sutra] = require('./sutra');
keywords[symboltable.KW.nirdesa] = require('./nirdesa');
keywords[symboltable.KW.avahan] = require('./avahan');
keywords[symboltable.KW.pravar] = require('./pravar');

module.exports = keywords;
