/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import InputStream from '../inputstream/main.js';
import Lexer from '../lexer/main.js';
import Parser from '../parser/main.js';
import Environment from '../environment/main.js';
import Evaluator from '../evaluators/main.js';

const Interpreter = (code, name = 'script', dir = '') => {
  const is = new InputStream({ name, dir, code });
  const lexed = new Lexer(is);
  const ast = new Parser(lexed);
  const env = new Environment();
  const evaluator = new Evaluator(env, ast);
  evaluator.interpreteProgram();
};

export default Interpreter;
