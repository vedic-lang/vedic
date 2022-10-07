/*
 * ॥ श्री गणेशाय नमः ॥
 * © Copyright 2022 @ptprashanttripathi
 * https://github.com/ptprashanttripathi
 */

import chalk from 'chalk';

import cli from './cli.js';
import Interpreter from '../interpreter/main.js';
import fetchSource from '../fetchSource/main.js';
import symboltable from '../symboltable/main.js';

// HANDLING ERROR
process.on('unhandledRejection', (err) => {
  console.error('UNHANDLED ERROR', err);
  process.exit();
});

// VERSION
if (cli.input.version) {
  cli.showVersion();
  cli.end();
}

// HELP
if (cli.input.help) {
  cli.showHelp();
  cli.end();
}

// 'help', 'version', 'debug'
if (!cli.input._[0]) {
  cli.showWelcomeMsg();
} else {
  try {
    if (!symboltable.Extensions.includes(cli.input._[0].split('.').pop())) {
      console.log(
        chalk.hex('#f44336').inverse.bold(' ERROR: ') +
          '\n\n' +
          chalk.bold(
            'This file type not supported.\n\nSupported file types: '
          ) +
          chalk.yellow("['v', 'ved', 'veda']") +
          '\n\n'
      );
      cli.end();
    }
    const { d } = cli.input;

    if (d) console.log(chalk.hex('#9966cc').inverse.bold(' Debug Mod '), '\n');

    if (d) console.time('⌛ Total Runtime');

    const file = fetchSource(cli.input._[0], true);
    if (!file) {
      console.log(
        chalk.hex('#FFF').bold('\n Usage: '),
        chalk.green('vedic'),
        chalk.cyan(' [path/to/script.ved]\n')
      );
      cli.end();
    }

    Interpreter(file.code, file.name, file.dir);

    if (d) console.timeEnd('⌛ Total Runtime');
  } catch (err) {
    console.log('\n', chalk.yellow('✖ ERROR →'), err.message, '\n');
  }
}
