/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

function cleaner(returnedValue) {
  switch (typeof returnedValue) {
    case 'string':
    case 'number':
      return returnedValue;
    case 'boolean':
      return returnedValue ? 'सत्य' : 'असत्य';
    case 'object':
      if (Array.isArray(returnedValue)) return returnedValue;
  }

  throw new Error(
    ' system error: invalid result returned from helper function'
  );
}

module.exports = cleaner;
