/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const KEYWORDS = {
  maan: 'मान', // var
  vad: 'वद', // print
  padh: 'पठ', // read
  yadi: 'यदि', // if
  atha: 'अथ', // else
  satya: 'सत्य', // true
  asatya: 'असत्य', // false
  sutra: 'सूत्र', // function
  phala: 'फल', // return
  chakra: 'चक्रम्', // forloop
  paryantam: 'पर्यन्तम्', // whileloop
  viram: 'विराम्', // break
  nirdesa: 'निर्देश', // Switch
  yada: 'यद', // Case
  yadabhave: 'यदभावे', // Default
  avahan: 'अवहन्', // Import
};

const SYMBOLS = {
  DBL_QUOTE: '"',
  SGL_QUOTE: "'",
  PERIOD: '.',
  OR: '||',
  AND: '&&',
  BINARY_AND: '&',
  L_THAN: '<',
  G_THAN: '>',
  G_THAN_OR_EQ: '>=',
  L_THAN_OR_EQ: '<=',
  NOT_EQ: '!=',
  EQ: '==',
  ASSIGN: '=',
  PLUS: '+',
  MINUS: '-',
  MULTIPLY: '*',
  DIVIDE: '/',
  REMAINDER: '%',
  L_PAREN: '{',
  R_PAREN: '}',
  COMMA: ',',
  COMMENT: '#',
  L_BRACKET: '(',
  R_BRACKET: ')',
  L_SQ_BRACKET: '[',
  R_SQ_BRACKET: ']',
  STATEMENT_TERMINATOR: ';',
  NEW_LINE: '\n',
  TAB_SPACE: '\t',
  EMPTY_SPACE: ' ',
  EXCLAMATION_POINT: '!',
  PIPE: '|',
  COLON: ':',
};

const LIST = {
  PUNCTUATIONS: [
    SYMBOLS.L_BRACKET,
    SYMBOLS.R_BRACKET,
    SYMBOLS.L_PAREN,
    SYMBOLS.R_PAREN,
    SYMBOLS.STATEMENT_TERMINATOR,
    SYMBOLS.COMMA,
    SYMBOLS.L_SQ_BRACKET,
    SYMBOLS.R_SQ_BRACKET,
    SYMBOLS.COLON,
    SYMBOLS.SGL_QUOTE,
  ],
  OPERATORS: [
    SYMBOLS.PLUS,
    SYMBOLS.MINUS,
    SYMBOLS.MULTIPLY,
    SYMBOLS.DIVIDE,
    SYMBOLS.REMAINDER,
    SYMBOLS.L_THAN,
    SYMBOLS.G_THAN,
    SYMBOLS.EQ,
    SYMBOLS.EXCLAMATION_POINT,
    SYMBOLS.PIPE,
    SYMBOLS.BINARY_AND,
    SYMBOLS.ASSIGN,
  ],
  WHITESPACES: [SYMBOLS.EMPTY_SPACE, SYMBOLS.TAB_SPACE, SYMBOLS.NEW_LINE],
  KEYWORDS: [
    KEYWORDS.maan,
    KEYWORDS.paryantam,
    KEYWORDS.yadi,
    KEYWORDS.vad,
    KEYWORDS.atha,
    KEYWORDS.satya,
    KEYWORDS.asatya,
    KEYWORDS.sutra,
    KEYWORDS.chakra,
    KEYWORDS.phala,
    KEYWORDS.viram,
    KEYWORDS.nirdesa,
    KEYWORDS.yada,
    KEYWORDS.yadabhave,
    KEYWORDS.avahan,
    KEYWORDS.pravar,
  ],
};

const REGEX = {
  DIGIT: /[\u0966-\u096F]/i,
  IDENTIFIER: /[\u0900-\u097F]/i,
};

const constants = {
  KW: KEYWORDS,
  SYM: SYMBOLS,
  LIST: LIST,
  REGEX: REGEX,
  KEYWORD: 'keyword',
  VARIABLE: 'variable',
  STRING: 'string',
  SUCHI: 'suchi',
  SUCHI_ELEM: 'suchiElement',
  NUMBER: 'number',
  PROGRAM: 'program',
  PUNCTUATION: 'punctuation',
  OPERATOR: 'operator',
  GET_maan: 'getmaan',
  CALL_sutra: 'callsutra',
  EXP_PUNC: 'expression_punctuations',
  Extensions: ['v', 'ved', 'veda'],
  NEGATIVE_EXPRESSION: 'negative_exp',
};

module.exports = constants;
