/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

function kul(args) {
  if (Array.isArray(args)) {
    const [param] = args;
    if (Array.isArray(param)) return param.length;

    throw new Error('Invalid param given to helper sutra kul.');
  }

  throw new Error(' system error');
}

module.exports = kul;
