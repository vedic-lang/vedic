const symboltable = require('./symboltable');
const errorhandler = require('./errorhandler');

class CallsutraNl {
  getNode(sutraNameToken) {
    sutraNameToken = sutraNameToken || {};
    const node = {};
    node.operation = 'CALL_SUTRA';
    node.name = sutraNameToken.value || this.lexer().next().value;
    node.paramValues = this.parseDelimited(
      symboltable.SYM.L_BRACKET,
      symboltable.SYM.R_BRACKET,
      symboltable.SYM.COMMA,
      this.parseExpression.bind(this),
      null
    );
    if (sutraNameToken.value === undefined) {
      this.skipPunctuation(symboltable.SYM.STATEMENT_TERMINATOR);
    }
    return node;
  }
}
const callsutranl = new CallsutraNl();

class KWNl {
  getNode() {
    if (KWNl.isBooleanKeywordNl(this)) {
      return leafnl.getNode.call(this);
    }
    this.throwError(errorhandler.expectBooleanMsg());
  }
  static isBooleanKeywordNl(context) {
    return [symboltable.KW.satya, symboltable.KW.asatya].includes(
      context.lexer().peek().value
    );
  }
}

class VariableNl {
  getNode() {
    const varNameToken = this.lexer().next();
    const nextTokenValue = this.lexer().peek().value;
    const variablenltypes = {};
    variablenltypes[symboltable.SYM.L_BRACKET] = callsutranl;
    variablenltypes[symboltable.SYM.L_SQ_BRACKET] = new SuchiNl(); // when current variable is an suchi element
    if (variablenltypes[nextTokenValue]) {
      const variableNlType = variablenltypes[nextTokenValue];
      return variableNlType.getNode.call(this, varNameToken);
    }
    return {
      operation: 'GET_MAAN',
      name: varNameToken.value,
    };
  }
}

class LeafNl {
  getNode() {
    return {
      value: this.lexer().next().value,
      left: null,
      right: null,
      operation: null,
    };
  }
}

class SuchiNl {
  getNode(suchiNameToken) {
    return !suchiNameToken
      ? SuchiNl.getParsedSuchiLiteral(this)
      : SuchiNl.getParsedSuchiElement(this, suchiNameToken);
  }
  static getParsedSuchiLiteral(context) {
    const node = {};
    node.operation = 'SUCHI';
    node.body = context.parseDelimited(
      symboltable.SYM.L_SQ_BRACKET,
      symboltable.SYM.R_SQ_BRACKET,
      symboltable.SYM.COMMA,
      context.parseExpression.bind(context),
      null
    );
    return node;
  }
  static getParsedSuchiElement(context, suchiNameToken) {
    const node = {};
    node.operation = 'SUCHI_ELEMENT';
    node.name = suchiNameToken.value;
    node.indexNodes = SuchiNl.getSuchiElementIndexNodes(context);
    return node;
  }
  static getSuchiElementIndexNodes(context) {
    const indexNodes = [SuchiNl.getSuchiElementIndexNode(context)];
    while (context.isNextTokenPunctuation(symboltable.SYM.L_SQ_BRACKET)) {
      indexNodes.push(SuchiNl.getSuchiElementIndexNode(context));
    }
    return indexNodes;
  }
  static getSuchiElementIndexNode(context) {
    let indexNode = { operation: null, right: null, left: null, value: '' };
    context.skipPunctuation(symboltable.SYM.L_SQ_BRACKET);
    if (SuchiNl.isNotEmptySuchiIndex(context)) {
      indexNode = context.parseExpression();
    }
    context.skipPunctuation(symboltable.SYM.R_SQ_BRACKET);
    return indexNode;
  }
  static isNotEmptySuchiIndex(context) {
    return context.lexer().peek().value !== symboltable.SYM.R_SQ_BRACKET;
  }
}

class BracketExpressionNl {
  getNode(config) {
    config = config || {
      isArithmeticExpression: true,
      isBracketExpected: true,
    };
    if (config.isBracketExpected) {
      this.skipPunctuation(symboltable.SYM.L_BRACKET);
    }
    this.setIsArithmeticExpression(config.isArithmeticExpression);
    const node = this.parseExpression();
    this.setIsArithmeticExpression(true);
    if (config.isBracketExpected) {
      this.skipPunctuation(symboltable.SYM.R_BRACKET);
    }
    return node;
  }
}

class NotOperatorNl {
  getNode() {
    return {
      operation: this.lexer().next().value,
      body: this.parseExpression(),
    };
  }
}

class NegativeNl {
  getNode() {
    this.skipOperator(symboltable.SYM.MINUS);
    return {
      operation: 'NEGATIVE_EXP',
      body: this.parseExpression(),
    };
  }
}

const nodeLiterals = {
  VARIABLE: new VariableNl(),
  NUMBER: new LeafNl(),
  STRING: new LeafNl(),
  KEYWORD: new KWNl(),
  CALL_SUTRA: callsutranl,
  EXP_PUNC: {},
};

nodeLiterals['EXP_PUNC'][symboltable.SYM.L_SQ_BRACKET] = new SuchiNl();
nodeLiterals['EXP_PUNC'][symboltable.SYM.L_BRACKET] = new BracketExpressionNl();
nodeLiterals['EXP_PUNC'][symboltable.SYM.EXCLAMATION_POINT] =
  new NotOperatorNl();
nodeLiterals['EXP_PUNC'][symboltable.SYM.MINUS] = new NegativeNl();
module.exports = nodeLiterals;
