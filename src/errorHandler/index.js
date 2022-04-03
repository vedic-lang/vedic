// ॥ श्री गणेशाय नमः ॥ 

class Logger {
  constructor() {
    this.filename = null;
    this.values = [];
  };

  log(value) {
    this.values.push({
      'log': value,
    });
  };

  error(e, ln) {
    this.values.push({
      'error': {
        'message': e.message,
        'line': ln,
      }
    });
  };

  clear() {
    this.values = [];
  };
  
  init(filename) {
    this.clear();
    this.filename = filename;
    return this;
  }
};

let globalLog = new Logger();

module.exports = {
  globalLog,
};

const errorHandler = (msg, line, col) => {
  let err;
  if (line && col) {
    err = `\n\n${msg} -- ln: ${line}, col: ${col}\n`
  } else {
    err = `\n\n${msg}\n`
  };
  try {
    // this.log.error(e, line);
    console.error(err);
  } catch (e) {
    console.error(e);
    process.exit(1);
  };
}

module.exports = {
  errorHandler
};