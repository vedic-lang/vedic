/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';
import callsutranl from './callsutranl.js';
import suchinl from './suchinl.js';
const variableNlTypes = {};
variableNlTypes[symboltable.SYM.L_BRACKET] = callsutranl;
variableNlTypes[symboltable.SYM.L_SQ_BRACKET] = suchinl; // when current variable is an suchi element

export default variableNlTypes;
