/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const InputStream = require('../inputstream');
const Lexer = require('../lexer');
const Parser = require('../parser');
const Environment = require('../environment');
const Evaluator = require('../evaluators');

const Interpreter = (code, name = 'script', dir = '') => {
  const is = new InputStream({ name, dir, code });
  const lexed = new Lexer(is);
  const ast = new Parser(lexed);
  const env = new Environment();
  const evaluator = new Evaluator(env, ast);
  evaluator.interpreteProgram();
};

module.exports = Interpreter;
