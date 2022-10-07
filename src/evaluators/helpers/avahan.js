/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import Parser from '../../parser/main.js';

import Lexer from '../../lexer/main.js';
import InputStream from '../../inputstream/main.js';
import fetchSource from '../../fetchSource/main.js';

class avahan {
  interpreteNode(node) {
    const fileName = this.evaluateNode(node.path);
    const file = fetchSource(fileName);
    const parser = new Parser(new Lexer(new InputStream(file)));
    this.interpreteImportedProgram(parser);
  }
}

export default new avahan();
