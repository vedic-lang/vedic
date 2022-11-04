const Parser = require('../parser');
const Lexer = require('../lexer');
const InputStream = require('../inputstream');
const fetchSource = require('../fetchSource');

class avahan {
  interpreteNode(node) {
    const fileName = this.evaluateNode(node.path);
    const file = fetchSource(fileName);
    const parser = new Parser(new Lexer(new InputStream(file)));
    this.interpreteImportedProgram(parser);
  }
}

module.exports = new avahan();
