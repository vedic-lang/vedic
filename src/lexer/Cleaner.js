// ॥ श्री गणेशाय नमः ॥ 

const Cleaner = (input) => {
  var cleanCode = input;

  var regexps = {
    SINGLELINE_REGEX: /\/\/.*/g,
    MULTILINE_REGEX: /\/\*([\s\S]*?)\*\//g,
    NEWLINE_REGEX: /(\r\n|\r)/g,
    MULTINEWLINE_REGEX: /[\n]+/g
  };

  for (const key in regexps) {
    cleanCode = cleanCode.replace(regexps[key], '\n');
  }
  
  return cleanCode.trim();
};

module.exports = {
  Cleaner
};