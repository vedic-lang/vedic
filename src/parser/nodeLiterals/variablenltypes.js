const symboltable = require('../../symboltable');

const variableNlTypes = {};
variableNlTypes[symboltable.SYM.L_BRACKET] = require('./callsutranl');
variableNlTypes[symboltable.SYM.L_SQ_BRACKET] = require('./suchinl'); // when current variable is an suchi element

module.exports = variableNlTypes;
