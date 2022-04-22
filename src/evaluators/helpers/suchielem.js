/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const contansts = require('../../symboltable');
const errorhandler = require('../../errorhandler');

class SuchiElement {
  interpreteNode(node) {
    const maanNode = { name: node.name, operation: contansts.GET_maan };
    const suchiLiteral = this.evaluateNode(maanNode);

    return SuchiElement.getSuchiElement(this, node, suchiLiteral);
  }

  static getSuchiElement(context, node, suchiLiteral) {
    let suchiElement;
    let isOnedimensionalSuchi = true;

    node.indexNodes.map((indexNode) => {
      const index = context.evaluateNode(indexNode);

      if (typeof index === 'number') {
        suchiElement = isOnedimensionalSuchi
          ? suchiLiteral[index]
          : suchiElement[index];
        isOnedimensionalSuchi = false;
      } else {
        context.throwError(errorhandler.invalidSuchiIndexTypeMsg(node.name));
      }
    });

    if (!suchiElement) {
      context.throwError(errorhandler.suchiIndexDoesNotExistMsg(node.name));
    }

    return suchiElement;
  }
}

module.exports = new SuchiElement();
