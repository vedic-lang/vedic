/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');
const leafNl = require('./leafnl');

const nodeLiterals = {};
nodeLiterals[symboltable.EXP_PUNC] = {};
nodeLiterals[symboltable.VARIABLE] = require('./variablenl');
nodeLiterals[symboltable.NUMBER] = leafNl;
nodeLiterals[symboltable.STRING] = leafNl;
nodeLiterals[symboltable.KEYWORD] = require('./keywordnl');
nodeLiterals[symboltable.CALL_sutra] = require('./callsutranl');

nodeLiterals[symboltable.EXP_PUNC][
  symboltable.SYM.L_SQ_BRACKET
] = require('./suchinl');
nodeLiterals[symboltable.EXP_PUNC][
  symboltable.SYM.L_BRACKET
] = require('./bracketnl');
nodeLiterals[symboltable.EXP_PUNC][
  symboltable.SYM.EXCLAMATION_POINT
] = require('./notoperatornl');
nodeLiterals[symboltable.EXP_PUNC][
  symboltable.SYM.MINUS
] = require('./negativenl');

module.exports = nodeLiterals;
