const errorhandler = require('../errorhandler');

class sutra {
  interpreteNode(node) {
    if (
      this.environment().getsutra(this.getCurrentScope(), node.name) !==
      undefined
    ) {
      this.throwError(
        errorhandler.sutraAlreadyExist(node.name, this.getCurrentScope())
      );
    }

    this.environment().setsutra(this.getCurrentScope(), node.name, node);
  }
}

module.exports = new sutra();
