// ॥ श्री गणेशाय नमः ॥ 

const vm = require('vm');
const libs = require('../libs');
module.exports = {
  run: (input) => {
    vm.createContext(libs);
    vm.runInContext(input, libs);
  }
};