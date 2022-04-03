const yargsparser = require('yargs-parser');
const pkgJSON = require('../../package.json');
const chalk = require('chalk');

module.exports = {
  showWelcomeMsg: () => {
    console.log(`
    ${chalk.hex('#ff9933').inverse.bold(' Vedic.JS ')} ${chalk.hex('#FFF').bold('v'+pkgJSON.version)}

      ${chalk.hex('#FFF').bold('Use:')} ${chalk.green('vedic')} ${chalk.cyan('[path/to/script.ved]')} 
      
      ❯ To see help run command ${chalk.green('vedic')} ${chalk.yellow('--help')}

    `);
  },
  showVersion: () => {
    console.log(chalk.hex('#FFFFFF').bold('v' + pkgJSON.version));
  },
  showHelp: () => {
    console.log(`
    ${chalk.hex('#ff9933').inverse.bold(' Vedic.JS ')} 

    ${chalk.hex('#FFF').bold(' Version :')} ${chalk.whiteBright(pkgJSON.version)}

    ${chalk.hex('#FFF').bold(' Description: ')}
      ${chalk.whiteBright(pkgJSON.description)}

    ${chalk.hex('#FFF').bold(' Usage: ')}
      ${chalk.green('vedic')} ${chalk.cyan('[path/to/script.ved]')} ${chalk.yellow('[--options]')}

    ${chalk.hex('#FFF').bold(' Options: ')}
      ${chalk.yellow('-d --debug')}    Run in Debug Mod
      ${chalk.yellow('-l --log')}      Print logs
      ${chalk.yellow('-v --version')}  Print version number
      ${chalk.yellow('-h --help')}     Print Vedic CLI help

    ${chalk.hex('#FFF').bold(' Examples: ')}    
      ${chalk.green('vedic')} ${chalk.cyan('script.ved')}

      ❯ You can also run command + option at once:
      ${chalk.green('vedic')} ${chalk.cyan('script.ved')} ${chalk.yellow(`-d`)}  
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
      log: ['l']
    },
    boolean: ['help', 'version', 'log', 'debug'],
    // string: [{ key: 'output' }],
    default: {
      help: false,
      version: false,
      log: false,
      debug: false
    }
  })
}