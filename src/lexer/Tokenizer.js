// ॥ श्री गणेशाय नमः ॥ 

const { tokenList, reserved } = require('../TokenList');
const { errorHandler } = require('../errorHandler');

const Tokenizer = (input) => {  
  let index = 0;
  let char = input[index];
  let currentToken = {};
  let tokens = [];

  const resetCurrentToken = () => {
    currentToken = {};
  }

  const next = () => {
    index++;
    char = input[index];
  };

  const peakNext = () => {
    try {
      return input[index + 1];
    } catch {
      return false;
    };
  };

  const handleStr = (type) => {
    currentToken = {
      "type": "STRING",
      "line": char.line,
      "col": char.col,
      "value": "",
    };

    if (peakNext()) {
      next();
    } else {
      errorHandler('EOF WHILE PARSING STRING', currentToken.line, currentToken.col);
    }

    while (char.type != type) {
      currentToken.value = currentToken.value.concat(char.value);
      next();
      if (!char) {
        errorHandler('EOF WHILE PARSING STRING', currentToken.line, currentToken.col);
      };
    };
    tokens.push(currentToken);
    resetCurrentToken();
    next();
    lex();
  };
  const handleNum = () => {
    let hasDecimal = false;

    currentToken = {
      "type": "NUMBER",
      "line": char.line,
      "col": char.col,
      "value": "",
    };

    // Long statements to handle decimal and int numbers
    while (char.type == "NUMBER" || (!hasDecimal && char.type == "DOT" && peakNext() && peakNext().type == "NUMBER")) {
      if (char.type == "DOT") {
        hasDecimal = true;
      };
      currentToken.value = currentToken.value.concat(char.value);
      next();
      if (!char) {
        break;
      };
    };
    tokens.push(currentToken);
    resetCurrentToken();
    lex();
  };

  const handleLetter = (type) => {
    currentToken = {
      "type": "IDENTIFIER",
      "line": char.line,
      "col": char.col,
      "value": "",
    };

    while (char.type == "NUMBER" || char.type == "LETTER") {
      currentToken.value = currentToken.value.concat(char.value);
      next();
      if (!char) {
        break;
      };
    };

    if (Object.keys(reserved).includes(currentToken.value)) {
      currentToken.type = "KEYWORD";
    };

    tokens.push(currentToken);
    resetCurrentToken();
    lex();
  };
  const isDoubleCharOperator = () => {
    if (peakNext()) {
      let operatorToTest = char.value.concat(peakNext().value)
      for (let [type, verification] of Object.entries(tokenList)) {
        if (verification(operatorToTest) && (type != "UNRECOGNIZED")) {
          return {
            "type": type,
            "line": char.line,
            "col": char.col,
            "value": operatorToTest,
          };
        };
      };
      return false;
    };
  };
  const handleOperator = () => {
    if (char.type != "WHITESPACE") {
      let doubleCharOperator = isDoubleCharOperator();
      if (doubleCharOperator) {
        next();
        currentToken = doubleCharOperator;
        tokens.push(currentToken);
        resetCurrentToken();
      } else {
        currentToken = char;
        tokens.push(currentToken);
        resetCurrentToken();
      };
    };
    next();
    lex();
  };
  const lex = () => {
    if (char) {
      switch (char.type) {
        case "QUOTE":
        case "DOUBLEQUOTE":
        case "BACKTICK":
          handleStr(char.type);
          break;
        case "NUMBER":
          handleNum();
          break;
        case "LETTER":
          handleLetter(char.type);
          break;
        default:
          handleOperator();
      };
    };
  };
  lex();
  return tokens;
};

module.exports = {
  Tokenizer
};