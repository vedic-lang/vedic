// ॥ श्री गणेशाय नमः ॥ 

const { reserved } = require('../TokenList');

const Parser = (input) => {
  finalcode = '';
  const add = (value) => {
    finalcode += " " + value + " "
  }
  input.forEach((item, index) => {
    switch (item.type) {
      case "KEYWORD":
        add(reserved[item.value]);
        break;
      case "STRING":
        add("`"+item.value+"`");
        break;
      case "IDENTIFIER":
        add(item.value);
        break;
      default:
        add(item.value)
    }
  });
  return { finalcode }
}

module.exports = {
  Parser
};