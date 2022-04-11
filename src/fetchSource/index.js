const fs = require('fs');
const path = require('path');
const chalk = require('chalk');
module.exports = (file, check) => {
  try {
    const name = path.resolve(file);
    const dir = path.dirname(name);
    let code = fs.readFileSync(name, 'utf8');
    code = code.replace(/\r\n|\r/g, '\n');
    return { name, dir, code };
  } catch (e) {
    console.log(
      chalk.hex('#FFF').bold('\n Error: '),
      'no such file or directory\n'
    );
    if (check) {
      console.log(
        chalk.hex('#FFF').bold('\n Usage: '),
        chalk.green('vedic'),
        chalk.cyan(' [path/to/script.ved]\n')
      );
    }
  };
};
