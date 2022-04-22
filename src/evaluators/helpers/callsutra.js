/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const cleaner = require('./cleaner');
const errorhandler = require('../../errorhandler');
const symboltable = require('../../symboltable');

class Callsutra {
  interpreteNode(node) {
    const sutraNode = Callsutra.getsutraNode(this, node.name);

    if (sutraNode == null) {
      if (this.environment().existSutraSangraha(node.name)) {
        return cleaner(
          this.environment().runSutraSangraha(
            node.name,
            Callsutra.getsutraHelperParams(this, node.paramValues)
          )
        );
      }

      this.throwError(
        errorhandler.varDoesNotExist(symboltable.KW.sutra, node.name)
      );
    }

    return Callsutra.startNewScope(
      this,
      sutraNode,
      Callsutra.getResolvedParameterValues(this, node.paramValues)
    );
  }

  static getsutraNode(context, sutraName) {
    for (let index = context.scopeStack().length - 1; index >= 0; index--) {
      if (
        context
          .environment()
          .getsutra(context.scopeStack()[index], sutraName) !== undefined
      ) {
        return context
          .environment()
          .getsutra(context.scopeStack()[index], sutraName);
      }
    }
    return null;
  }

  static getsutraHelperParams(context, paramNodeList) {
    const params = [];
    paramNodeList.forEach((paramNode) => {
      params.push(context.evaluateNode(paramNode));
    });
    return params;
  }

  static getResolvedParameterValues(context, paramValueNodes) {
    const paramValues = [];
    paramValueNodes.forEach((paramValueNode) => {
      paramValues.push(context.evaluateNode(paramValueNode));
    });

    return paramValues;
  }

  static startNewScope(context, sutraNode, paramValues) {
    context.pushToScopeStack(sutraNode.name);
    Callsutra.setsutraNodeParam(context, sutraNode.paramTokens, paramValues);
    const returnedValue = Callsutra.runsutraNodeBody(context, sutraNode.body);
    context.popFromScopeStack();

    return returnedValue;
  }

  static setsutraNodeParam(context, sutraNodeParamTokens, sutraParamValues) {
    for (let i = 0; i < sutraNodeParamTokens.length; i++) {
      context
        .environment()
        .setmaan(
          context.getCurrentScope(),
          sutraNodeParamTokens[i].value,
          sutraParamValues[i]
        );
    }
  }

  static runsutraNodeBody(context, sutraNodeBody) {
    for (let i = 0; i < sutraNodeBody.length; i++) {
      const returnedValue = context.evaluateNode(sutraNodeBody[i]);
      if (returnedValue !== undefined) return returnedValue;
    }
  }
}

module.exports = new Callsutra();
