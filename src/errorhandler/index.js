/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

const message = {
  unexpectedtoken: (arg) => `Cannot process unexpected token : ${arg}`,
  chakraIncrementAndDecrementMsg: () =>
    'Invalid decrement or increment operation',
  invalidFileMsg: () => 'File with invalid extension',
  invalidAssignment: () => 'Cannot assign value to  sutra call',
  invalidSuchiIndexTypeMsg: (arg) =>
    `Typeof index given for suchi ${arg} must be a number`,
  suchiIndexDoesNotExistMsg: (arg) =>
    `Index given for suchi ${arg} does not exist`,
  varDoesNotExist: (type, name) => `${type} ${name} is undefined`,
  sutraAlreadyExist: (name, scope) =>
    `sutra with name ${name} already exists within the ${scope} scope`,
  expectStringMsg: (arg) => `${arg} expects a string`,
  expectBooleanMsg: () => 'Expecting keyword value e.g boolean (satya|asatya)',
  unexpectedDeclaration: (arg) => ` ${arg} keyword not expected`,
  ArithmeticException: () => 'ArithmeticException - cannot divide by zero',
  undefinedValueMsg: (arg) => `Cannot set value undefined to variable ${arg}`,
  cannotNegateMsg: (arg) =>
    `Cannot apply negation operator to the given expression: ${arg}`,
};

module.exports = message;
