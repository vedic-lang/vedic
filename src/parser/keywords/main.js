/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import symboltable from '../../symboltable/main.js';
import maan from './maan.js';
import vad from './vad.js';
import yadi from './yadi.js';
import paryantam from './paryantam.js';
import chakra from './chakra.js';
import phala from './phala.js';
import viram from './viram.js';
import sutra from './sutra.js';
import nirdesa from './nirdesa.js';
import avahan from './avahan.js';
import pravar from './pravar.js';

const keywords = {};
keywords[symboltable.KW.maan] = maan;
keywords[symboltable.KW.vad] = vad;
keywords[symboltable.KW.yadi] = yadi;
keywords[symboltable.KW.paryantam] = paryantam;
keywords[symboltable.KW.chakra] = chakra;
keywords[symboltable.KW.phala] = phala;
keywords[symboltable.KW.viram] = viram;
keywords[symboltable.KW.sutra] = sutra;
keywords[symboltable.KW.nirdesa] = nirdesa;
keywords[symboltable.KW.avahan] = avahan;
keywords[symboltable.KW.pravar] = pravar;

export default keywords;
