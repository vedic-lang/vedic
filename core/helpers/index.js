const symboltable = require('../symboltable');

const evaluators = {};
evaluators[symboltable.SYM.PLUS] = require('./plus');
evaluators[symboltable.SYM.MINUS] = require('./minus');
evaluators[symboltable.SYM.DIVIDE] = require('./divide');
evaluators[symboltable.SYM.MULTIPLY] = require('./multiply');
evaluators[symboltable.SYM.REMAINDER] = require('./remainder');
evaluators[symboltable.SYM.ASSIGN] = require('./maan');
evaluators[symboltable.SYM.EQ] = require('./equals');
evaluators[symboltable.SYM.G_THAN] = require('./gthan');
evaluators[symboltable.SYM.OR] = require('./or');
evaluators[symboltable.SYM.AND] = require('./and');
evaluators[symboltable.SYM.L_THAN] = require('./lthan');
evaluators[symboltable.SYM.G_THAN_OR_EQ] = require('./gthanoreq');
evaluators[symboltable.SYM.L_THAN_OR_EQ] = require('./lthanoreq');
evaluators[symboltable.SYM.NOT_EQ] = require('./noteq');
evaluators[symboltable.SYM.EXCLAMATION_POINT] = require('./notoperator');

evaluators[symboltable.KW.vad] = require('./vad');
evaluators[symboltable.KW.yadi] = require('./yadi');
evaluators[symboltable.KW.paryantam] = require('./paryantam');
evaluators[symboltable.KW.viram] = require('./viram');
evaluators[symboltable.KW.chakra] = require('./chakra');
evaluators[symboltable.KW.nirdesa] = require('./nirdesa');
evaluators[symboltable.KW.sutra] = require('./sutra');
evaluators[symboltable.KW.phala] = require('./phala');
evaluators[symboltable.KW.avahan] = require('./avahan');
evaluators[symboltable.KW.pravar] = require('./pravar');

evaluators['CALL_SUTRA'] = require('./callsutra');
evaluators['GET_MAAN'] = require('./getmaan');
evaluators['SUCHI'] = require('./suchi');
evaluators['SUCHI_ELEMENT'] = require('./suchielem');
evaluators['NEGATIVE_EXP'] = require('./negativeexpression');

module.exports = evaluators;
