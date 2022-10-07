/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';
import plus from './plus.js';
import minus from './minus.js';
import divide from './divide.js';
import multiply from './multiply.js';
import remainder from './remainder.js';
import maan from './maan.js';
import equals from './equals.js';
import gthan from './gthan.js';
import or from './or.js';
import and from './and.js';
import lthan from './lthan.js';
import gthanoreq from './gthanoreq.js';
import lthanoreq from './lthanoreq.js';
import noteq from './noteq.js';
import notoperator from './notoperator.js';
import vad from './vad.js';
import yadi from './yadi.js';
import paryantam from './paryantam.js';
import viram from './viram.js';
import chakra from './chakra.js';
import nirdesa from './nirdesa.js';
import sutra from './sutra.js';
import phala from './phala.js';
import avahan from './avahan.js';
import pravar from './pravar.js';
import callsutra from './callsutra.js';
import getmaan from './getmaan.js';
import suchi from './suchi.js';
import suchielem from './suchielem.js';
import negativeexpression from './negativeexpression.js';

const evaluators = {};
evaluators[symboltable.SYM.PLUS] = plus;
evaluators[symboltable.SYM.MINUS] = minus;
evaluators[symboltable.SYM.DIVIDE] = divide;
evaluators[symboltable.SYM.MULTIPLY] = multiply;
evaluators[symboltable.SYM.REMAINDER] = remainder;
evaluators[symboltable.SYM.ASSIGN] = maan;
evaluators[symboltable.SYM.EQ] = equals;
evaluators[symboltable.SYM.G_THAN] = gthan;
evaluators[symboltable.SYM.OR] = or;
evaluators[symboltable.SYM.AND] = and;
evaluators[symboltable.SYM.L_THAN] = lthan;
evaluators[symboltable.SYM.G_THAN_OR_EQ] = gthanoreq;
evaluators[symboltable.SYM.L_THAN_OR_EQ] = lthanoreq;
evaluators[symboltable.SYM.NOT_EQ] = noteq;
evaluators[symboltable.SYM.EXCLAMATION_POINT] = notoperator;

evaluators[symboltable.KW.vad] = vad;
evaluators[symboltable.KW.yadi] = yadi;
evaluators[symboltable.KW.paryantam] = paryantam;
evaluators[symboltable.KW.viram] = viram;
evaluators[symboltable.KW.chakra] = chakra;
evaluators[symboltable.KW.nirdesa] = nirdesa;
evaluators[symboltable.KW.sutra] = sutra;
evaluators[symboltable.KW.phala] = phala;
evaluators[symboltable.KW.avahan] = avahan;
evaluators[symboltable.KW.pravar] = pravar;

evaluators[symboltable.CALL_sutra] = callsutra;
evaluators[symboltable.GET_maan] = getmaan;
evaluators[symboltable.SUCHI] = suchi;
evaluators[symboltable.SUCHI_ELEM] = suchielem;
evaluators[symboltable.NEGATIVE_EXPRESSION] = negativeexpression;

export default evaluators;
