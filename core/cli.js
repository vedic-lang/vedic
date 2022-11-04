const chalk = require('chalk');
const yargsparser = require('yargs-parser');
const pkgJSON = require('../package.json');
const Interpreter = require('./interpreter');
const fetchSource = require('./fetchSource');
const { Extensions } = require('./symboltable');

const cli = {
  showWelcomeMsg: () => {
    console.log(`
    ${chalk.hex('#ff9933').inverse.bold(' Vedic ')} ${chalk
      .hex('#FFF')
      .bold('v' + pkgJSON.version)}

      ${chalk.hex('#FFF').bold('Use:')} ${chalk.green('vedic')} ${chalk.cyan(
      '[path/to/script.ved]'
    )} 
      
      ❯ To see help run command ${chalk.green('vedic')} ${chalk.yellow(
      '--help'
    )}

    `);
  },
  showVersion: () => {
    console.log(chalk.hex('#FFFFFF').bold('v' + pkgJSON.version));
  },
  showHelp: () => {
    console.log(`
    ${chalk.hex('#ff9933').inverse.bold(' Vedic ')} 

    ${chalk.hex('#FFF').bold(' Version :')} ${chalk.whiteBright(
      pkgJSON.version
    )}

    ${chalk.hex('#FFF').bold(' Description: ')}
      ${chalk.whiteBright(pkgJSON.description)}

    ${chalk.hex('#FFF').bold(' Usage: ')}
      ${chalk.green('vedic')} ${chalk.cyan(
      '[path/to/script.ved]'
    )} ${chalk.yellow('[--options]')}

    ${chalk.hex('#FFF').bold(' Options: ')}
      ${chalk.yellow('-d --debug')}    Run in Debug Mod
      ${chalk.yellow('-v --version')}  Print version number
      ${chalk.yellow('-h --help')}     Print Vedic CLI help

    ${chalk.hex('#FFF').bold(' Examples: ')}    
      ${chalk.green('vedic')} ${chalk.cyan('script.ved')}

      ❯ You can also run command + option at once:
      ${chalk.green('vedic')} ${chalk.cyan('script.ved')} ${chalk.yellow(
      '-d'
    )}  
      `);
  },
  end: () => {
    process.exit(0);
  },
  input: yargsparser(process.argv.slice(2), {
    alias: {
      help: ['h'],
      version: ['v'],
      debug: ['d'],
    },
    boolean: ['help', 'version', 'debug'],
    // string: [{ key: 'output' }],
    default: {
      help: false,
      version: false,
      log: false,
      debug: false,
    },
  }),
};

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
    if (!Extensions.includes(cli.input._[0].split('.').pop())) {
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
