/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const helpers = require('./helpers');
const symboltable = require('../symboltable');

class Mainevaluator {
  constructor(environment, parser) {
    this.environment = () => environment;
    this.parser = () => parser;
    this.initScopeStack();
  }

  initScopeStack() {
    const _scopeStack = ['global'];
    this.getCurrentScope = () => _scopeStack[_scopeStack.length - 1];
    this.scopeStack = () => [..._scopeStack];
    this.pushToScopeStack = (scope) => _scopeStack.push(scope);
    this.popFromScopeStack = () => _scopeStack.pop();
  }

  getLeafValue(leaf) {
    if (leaf.value != null) {
      return leaf.value;
    }

    return null;
  }

  evaluateNode(node) {
    const leafValue = this.getLeafValue(node);
    if (leafValue == null) {
      const evaluator = helpers[node.operation];
      return evaluator.interpreteNode.call(this, node);
    }

    return leafValue;
  }

  throwError(msg) {
    this.parser().throwError(msg);
  }

  interpreteProgram() {
    this.parser().pushToBlockTypeStack(symboltable.PROGRAM);
    while (this.parser().isNotEndOfFile()) {
      this.evaluateNode(this.parser().parseAst());
    }
    this.parser().popBlockTypeStack();
  }

  interpreteImportedProgram(parser) {
    new Mainevaluator(this.environment(), parser).interpreteProgram();
  }
}

module.exports = Mainevaluator;
