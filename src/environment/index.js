const sangraha = require("../sangraha");

class enviroment {
  constructor() {
    this.vars = {};
    this.sutraDeclarations = {};
  }

  setmaan(scope, name, value) {
    if (!this.vars[scope]) {
      this.vars[scope] = {};
    }

    this.vars[scope][name] = value;
  }

  getmaan(scope, name) {
    if (this.vars[scope]) {
      return this.vars[scope][name];
    }
  }

  setsutra(scope, sutraName, sutraNode) {
    if (!this.sutraDeclarations[scope]) {
      this.sutraDeclarations[scope] = {};
    }

    this.sutraDeclarations[scope][sutraName] = sutraNode;
  }

  getsutra(scope, sutraName) {
    if (this.sutraDeclarations[scope]) {
      return this.sutraDeclarations[scope][sutraName];
    }
  }

  existSutraSangraha(sutraName) {
    return sangraha[sutraName] !== undefined;
  }

  runSutraSangraha(sutraName, sutraArgs) {
    if (this.existSutraSangraha(sutraName)) {
      return sangraha[sutraName](sutraArgs);
    }
  }

  digitMapping = ["०", "१", "२", "३", "४", "५", "६", "७", "८", "९"];
  vad(value) {
    value = value
      .toString()
      .split("")
      .map((char) => {
        if (isNaN(parseInt(char))) return char;
        return this.digitMapping[char] || char;
      })
      .join("");
    console.log(value);
  }
}

module.exports = enviroment;
