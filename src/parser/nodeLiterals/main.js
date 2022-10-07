/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';

import leafNl from './leafnl.js';
import variablenl from './variablenl.js';
import keywordnl from './keywordnl.js';
import callsutranl from './callsutranl.js';
import suchinl from './suchinl.js';
import bracketnl from './bracketnl.js';
import notoperatornl from './notoperatornl.js';
import negativenl from './negativenl.js';
const nodeLiterals = {};
nodeLiterals[symboltable.EXP_PUNC] = {};
nodeLiterals[symboltable.VARIABLE] = variablenl;
nodeLiterals[symboltable.NUMBER] = leafNl;
nodeLiterals[symboltable.STRING] = leafNl;
nodeLiterals[symboltable.KEYWORD] = keywordnl;
nodeLiterals[symboltable.CALL_sutra] = callsutranl;

nodeLiterals[symboltable.EXP_PUNC][symboltable.SYM.L_SQ_BRACKET] = suchinl;
nodeLiterals[symboltable.EXP_PUNC][symboltable.SYM.L_BRACKET] = bracketnl;
nodeLiterals[symboltable.EXP_PUNC][symboltable.SYM.EXCLAMATION_POINT] =
  notoperatornl;
nodeLiterals[symboltable.EXP_PUNC][symboltable.SYM.MINUS] = negativenl;

export default nodeLiterals;
