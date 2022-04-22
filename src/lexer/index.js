/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../symboltable');

class Lexer {
  constructor(inputStream) {
    this.file = inputStream.file;
    this.inputStream = inputStream;
    this.currentToken = null;
  }

  isWhiteSpace(whiteSpaceChar) {
    return symboltable.LIST.WHITESPACES.includes(whiteSpaceChar);
  }

  isPunctuation(punctuationChar) {
    return symboltable.LIST.PUNCTUATIONS.includes(punctuationChar);
  }

  isIdentifier(id) {
    return symboltable.REGEX.IDENTIFIER.test(id);
  }

  ivadrator(operatorChar) {
    return symboltable.LIST.OPERATORS.includes(operatorChar);
  }

  isKeyword(keywordChar) {
    return symboltable.LIST.KEYWORDS.includes(keywordChar);
  }

  isDigit(digitChar) {
    return symboltable.REGEX.DIGIT.test(digitChar);
  }

  readWhile(predicate) {
    let str = '';

    while (
      this.inputStream.isNotEndOfFile() &&
      predicate(this.inputStream.peek())
    ) {
      str += this.inputStream.next();
    }

    return str;
  }

  readString(stringEnd) {
    this.inputStream.next();
    const str = this.readWhile((ch) => {
      return ch !== stringEnd;
    });

    if (this.inputStream.peek() === stringEnd) this.inputStream.next();
    else {
      this.throwError(`Expecting '${stringEnd}' but found unexpected char`);
    }

    return { type: symboltable.STRING, value: str };
  }

  readIdentifier() {
    const identifier = this.readWhile(this.isIdentifier);

    return {
      type: this.isKeyword(identifier)
        ? symboltable.KEYWORD
        : symboltable.VARIABLE,
      value: identifier,
    };
  }
  digitMapping = ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'];
  devtoengNum(str) {
    return parseFloat(
      str
        .toString()
        .split('')
        .map((char) => {
          const englishDigit = this.digitMapping.findIndex(
            (digit) => digit === char
          );
          return englishDigit === -1 ? char : `${englishDigit}`;
        })
        .join('')
    );
  }
  readNumber() {
    let hasDot = false;
    const num = this.readWhile((ch) => {
      if (ch === symboltable.SYM.PERIOD) {
        if (hasDot) return false;
        hasDot = true;
        return true;
      }
      return this.isDigit(ch);
    });

    return { type: symboltable.NUMBER, value: this.devtoengNum(num) };
  }

  skipComments() {
    this.readWhile((ch) => {
      return ch !== symboltable.SYM.NEW_LINE;
    });
    this.inputStream.next();
  }

  readNext() {
    this.readWhile(this.isWhiteSpace);
    if (this.inputStream.isEndOfFile()) return null;

    const ch = this.inputStream.peek();
    if (ch === symboltable.SYM.COMMENT) {
      this.skipComments();
      return this.readNext();
    }
    if (ch === symboltable.SYM.DBL_QUOTE)
      return this.readString(symboltable.SYM.DBL_QUOTE);
    if (ch === symboltable.SYM.SGL_QUOTE)
      return this.readString(symboltable.SYM.SGL_QUOTE);
    if (this.isDigit(ch)) return this.readNumber();
    if (this.isIdentifier(ch)) return this.readIdentifier();
    if (this.isPunctuation(ch))
      return { type: symboltable.PUNCTUATION, value: this.inputStream.next() };
    if (this.ivadrator(ch))
      return {
        type: symboltable.OPERATOR,
        value: this.readWhile(this.ivadrator),
      };

    this.throwError(`Cant handle character  '${ch}'`);
  }

  peek() {
    return this.current || (this.current = this.readNext());
  }

  throwError(msg) {
    this.inputStream.throwError(msg);
  }

  next() {
    const token = this.current;
    this.current = null;
    return token || this.readNext();
  }

  isEndOfFile() {
    return this.peek() == null;
  }

  isNotEndOfFile() {
    return this.peek() != null;
  }
}

module.exports = Lexer;
