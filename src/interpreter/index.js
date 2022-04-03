// ॥ श्री गणेशाय नमः ॥ 

const { Parser } = require('../parser');
const { Lexer } = require('../lexer');

const Interpreter = (code,filename=null) => {
  const lexer = Lexer(code); //{ cleanCode, charList, tokens }
  const parser = Parser(lexer.tokens); //{ finalCode}
  return { lexer, parser}
};

module.exports = {
  Interpreter
};