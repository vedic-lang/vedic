/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import fs from 'fs';
import path from 'path';

export default (file) => {
  try {
    const name = path.resolve(file);
    const dir = path.dirname(name);
    let code = fs.readFileSync(name, 'utf8');
    code = code.replace(/\r\n|\r/g, '\n');
    return { name, dir, code };
  } catch (e) {
    console.log('Error: no such file or directory');
  }
};
