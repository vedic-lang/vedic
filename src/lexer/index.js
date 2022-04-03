// ॥ श्री गणेशाय नमः ॥ 

const { Tokenizer } = require('./Tokenizer');
const { Seperator } = require("./Seperator");
const { Cleaner } = require("./Cleaner");

const Lexer = (input) => {
  const cleanCode = Cleaner(input);
  const charList = Seperator(cleanCode);
  const tokens = Tokenizer(charList);
  return { cleanCode, charList, tokens }
}
module.exports = {
  Lexer
};