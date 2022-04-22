/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const symboltable = require('../../symboltable');
const pravarHelper = require('./pravarhelper');
const errorhandler = require('../../errorhandler');

class maan {
  interpreteNode(node) {
    if (node.left.operation === symboltable.SUCHI_ELEM) {
      maan.setSuchiElement(this, node);
      return;
    }

    if (pravarHelper.ispravarVariable(this, node.left)) {
      maan.setpravarVariable(this, node);
      return;
    }

    this.environment().setmaan(
      this.getCurrentScope(),
      node.left,
      maan.getValue(this, node.right)
    );
  }

  static setpravarVariable(context, node) {
    const topIndex = context.scopeStack().length - 2;

    for (let index = topIndex; index >= 0; index--) {
      if (
        context
          .environment()
          .getmaan(context.scopeStack()[index], node.left) !== undefined
      ) {
        return context
          .environment()
          .setmaan(
            context.scopeStack()[index],
            node.left,
            maan.getValue(context, node.right)
          );
      }
    }
  }

  static setSuchiElement(context, node) {
    let suchiLiteral = maan.getSuchiLiteral(context, node);

    for (let i = 0; i < node.left.indexNodes.length; i++) {
      const suchiIndex = context.evaluateNode(node.left.indexNodes[i]);

      if (suchiIndex === '' && i === node.left.indexNodes.length - 1) {
        suchiLiteral.push(context.evaluateNode(node.right));
        return;
      }

      if (typeof suchiIndex === 'number') {
        if (
          !Array.isArray(suchiLiteral[suchiIndex]) &&
          i < node.left.indexNodes.length - 1
        ) {
          context.throwError(
            errorhandler.suchiIndexDoesNotExistMsg(node.left.name)
          );
        }

        if (
          Array.isArray(suchiLiteral[suchiIndex]) &&
          i < node.left.indexNodes.length - 1
        ) {
          suchiLiteral = suchiLiteral[suchiIndex];
        }

        if (i === node.left.indexNodes.length - 1) {
          suchiLiteral[suchiIndex] = context.evaluateNode(node.right);
        }
      } else {
        context.throwError(errorhandler.invalidSuchiIndexTypeMsg(node.name));
      }
    }
  }

  static getSuchiLiteral(context, node) {
    const maanNode = { name: node.left.name, operation: symboltable.GET_maan };
    return context.evaluateNode(maanNode);
  }

  static getValue(context, node) {
    const value = context.evaluateNode(node);
    if (value === undefined) {
      context.throwError(errorhandler.undefinedValueMsg(node.left));
    }
    return value;
  }
}

module.exports = new maan();
