/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../symboltable');

class InputStream {
  constructor(input) {
    this.file = { path: input.name, dir: input.dir };
    this.code = input.code;
    this.line = 1;
    this.column = 0;
    this.position = 0;
  }

  next() {
    const character = this.code.charAt(this.position++);

    if (character === symboltable.SYM.NEW_LINE) {
      this.column = 0;
      this.line++;
    } else {
      this.column++;
    }

    return character;
  }

  peek() {
    return this.code.charAt(this.position);
  }

  throwError(msg) {
    throw new Error(
      `${msg} \n\tat ${this.file.path}:${this.line}:${this.column}`
    );
  }

  isEndOfFile() {
    return this.peek() === '';
  }

  isNotEndOfFile() {
    return this.peek() !== '';
  }
}

module.exports = InputStream;
