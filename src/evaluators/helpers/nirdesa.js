/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

class nirdesa {
  interpreteNode(node) {
    const nirdesavalue = this.evaluateNode(node.nirdesavalue);

    for (let yadaIndex = 0; yadaIndex < node.nirdesabody.length; yadaIndex++) {
      if (
        nirdesa.isyadaValueMatchnirdesaValue(
          this,
          node.nirdesabody[yadaIndex].yadavalue,
          nirdesavalue
        )
      ) {
        return nirdesa.runMatchedBody(
          this,
          node.nirdesabody[yadaIndex].yadabody
        );
      }

      if (nirdesa.canRunyadabhave(yadaIndex, node)) {
        return nirdesa.runMatchedBody(this, node.yadabhave);
      }
    }
  }

  static isyadaValueMatchnirdesaValue(context, yadavalueNode, nirdesavalue) {
    return context.evaluateNode(yadavalueNode) === nirdesavalue;
  }

  static runMatchedBody(context, body) {
    for (let i = 0; i < body.length; i++) {
      const returnedValue = context.evaluateNode(body[i]);
      if (returnedValue !== undefined) return returnedValue;
    }
  }

  static canRunyadabhave(yadaIndex, node) {
    return (
      yadaIndex === node.nirdesabody.length - 1 && node.yadabhave !== undefined
    );
  }
}

module.exports = new nirdesa();
