/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../symboltable');
const errorhandler = require('../errorhandler');
const keywords = require('./keywords');
const nodeLiterals = require('./nodeLiterals');

class Parser {
  constructor(lexer) {
    this.lexer = () => lexer;
    this.file = lexer.file;
    this.initBlockTypeStack();
    this.initIsArithmeticExpression();
  }

  initBlockTypeStack() {
    const _blockTypeStack = [];
    this.pushToBlockTypeStack = (blockName) => {
      _blockTypeStack.push(blockName);
    };
    this.popBlockTypeStack = () => _blockTypeStack.pop();
    this.peekBlockTypeStack = () => _blockTypeStack[_blockTypeStack.length - 1];
    this.getBlockTypeStack = () => [..._blockTypeStack];
  }

  initIsArithmeticExpression() {
    let _isArithmeticExpression = true;
    this.setIsArithmeticExpression = (isArithmetic) => {
      _isArithmeticExpression = isArithmetic;
    };
    this.isArithmeticExpression = () => _isArithmeticExpression;
  }

  isNextTokenPunctuation(punc) {
    const token = this.lexer().peek();
    return (
      token && token.type === symboltable.PUNCTUATION && token.value === punc
    );
  }

  isNextTokenOperator(op) {
    const token = this.lexer().peek();
    return token && token.type === symboltable.OPERATOR && token.value === op;
  }

  isNextTokenKeyword(kw) {
    const token = this.lexer().peek();
    return token && token.type === symboltable.KEYWORD && token.value === kw;
  }

  skipPunctuation(punc) {
    if (this.isNextTokenPunctuation(punc)) this.lexer().next();
    else {
      this.throwError(
        errorhandler.unexpectedtoken(this.getCurrentTokenValue())
      );
    }
  }

  skipOperator(op) {
    if (this.isNextTokenOperator(op)) this.lexer().next();
    else {
      this.throwError(
        errorhandler.unexpectedtoken(this.getCurrentTokenValue())
      );
    }
  }

  skipKeyword(kw) {
    if (this.isNextTokenKeyword(kw)) this.lexer().next();
    else {
      this.throwError(
        errorhandler.unexpectedtoken(this.getCurrentTokenValue())
      );
    }
  }

  getCurrentTokenValue() {
    return this.lexer().peek() ? this.lexer().peek().value : null;
  }

  parseExpression() {
    return this.parseAssign();
  }

  parseAssign() {
    return this.parseWhile([symboltable.SYM.ASSIGN], this.parseOr);
  }

  parseOr() {
    return this.parseWhile([symboltable.SYM.OR], this.parseAnd);
  }

  parseAnd() {
    return this.parseWhile(
      [symboltable.SYM.AND],
      this.parseGreaterLesserEquality
    );
  }

  parseGreaterLesserEquality() {
    const operatorList = [
      symboltable.SYM.L_THAN,
      symboltable.SYM.G_THAN,
      symboltable.SYM.G_THAN_OR_EQ,
      symboltable.SYM.L_THAN_OR_EQ,
      symboltable.SYM.EQ,
      symboltable.SYM.NOT_EQ,
    ];

    if (this.isArithmeticExpression()) {
      return this.parseWhile(operatorList, this.parsePlusMinus);
    } else return this.parseWhile(operatorList, this.parseNodeLiteral);
  }

  parsePlusMinus() {
    return this.parseWhile(
      [symboltable.SYM.PLUS, symboltable.SYM.MINUS],
      this.parseMultiplyDivisionRemainder
    );
  }

  parseMultiplyDivisionRemainder() {
    return this.parseWhile(
      [
        symboltable.SYM.MULTIPLY,
        symboltable.SYM.DIVIDE,
        symboltable.SYM.REMAINDER,
      ],
      this.parseNodeLiteral
    );
  }

  parseWhile(operatorList, parseOperationWithLesserPrecedence) {
    let node = parseOperationWithLesserPrecedence.bind(this)();

    while (this.isNextTokenInOperatorList(operatorList)) {
      node = {
        left: node,
        operation: this.lexer().next().value,
        right: parseOperationWithLesserPrecedence.bind(this)(),
        value: null,
      };
    }

    return node;
  }

  isNextTokenInOperatorList(operatorList) {
    return (
      this.isNotEndOfFile() && operatorList.includes(this.lexer().peek().value)
    );
  }

  parseNodeLiteral() {
    const token = this.lexer().peek();

    if (nodeLiterals[token.type]) {
      const nodeliteral = nodeLiterals[token.type];
      return nodeliteral.getNode.call(this);
    }

    if (nodeLiterals[symboltable.EXP_PUNC][token.value]) {
      const nodeliteral = nodeLiterals[symboltable.EXP_PUNC][token.value];
      return nodeliteral.getNode.call(this);
    }

    this.lexer().throwError(errorhandler.unexpectedtoken(token.value));
  }

  parseBlock(currentBlock) {
    this.pushToBlockTypeStack(currentBlock);
    this.skipPunctuation(symboltable.SYM.L_PAREN);
    const block = [];
    while (this.isNotEndOfBlock()) {
      block.push(this.parseAst());
    }
    this.skipPunctuation(symboltable.SYM.R_PAREN);
    this.popBlockTypeStack();

    return block;
  }

  isNotEndOfBlock() {
    return (
      this.isNotEndOfFile() &&
      this.lexer().peek().value !== symboltable.SYM.R_PAREN
    );
  }

  parseVarname() {
    return this.lexer().peek().type === symboltable.VARIABLE
      ? this.lexer().next().value
      : this.lexer().throwError(
          errorhandler.unexpectedtoken(this.lexer().peek().value)
        );
  }

  parseDelimited(start, stop, separator, parser, predicate) {
    const varList = [];
    let firstVar = true;

    this.skipPunctuation(start);
    while (this.isNotEndOfFile()) {
      if (this.isNextTokenPunctuation(stop)) break;
      if (firstVar) firstVar = false;
      else this.skipPunctuation(separator);
      if (this.isNextTokenPunctuation(stop)) break;
      varList.push(parser(predicate));
    }
    this.skipPunctuation(stop);

    return varList;
  }

  getTokenThatSatisfiesPredicate(predicate) {
    const token = this.lexer().next();
    if (predicate(token)) return token;

    this.throwError(errorhandler.unexpectedtoken(token.type));
  }

  parseAst() {
    const token = this.lexer().peek();

    if (keywords[token.value]) {
      const keyword = keywords[token.value];
      return keyword.getNode.call(this);
    }

    if (token.type === symboltable.VARIABLE) {
      const callsutraNodeLiteral = nodeLiterals[symboltable.CALL_sutra];
      return callsutraNodeLiteral.getNode.call(this);
    }

    this.throwError(errorhandler.unexpectedtoken(token.value));
  }

  isNotEndOfFile() {
    return this.lexer().isNotEndOfFile();
  }

  throwError(msg) {
    this.lexer().throwError(msg);
  }
}

module.exports = Parser;
