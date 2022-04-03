const chalk = require('chalk');
const cli = require('./cli');
const createLogs = require("./createLogs");
const checkupdate = require("./checkupdate");
const handleError = require("./handleError");
const spinner = require('ora')({ text: '' });
const Environment = require('../environment');
const { Interpreter } = require('../interpreter');
const checkNodeVersion = require("./checkNodeVersion");
const fetchSource = require("./fetchSource");

//HANDLING ERROR
process.on('unhandledRejection', err => {
    handleError(`UNHANDLED ERROR`, err);
});

// module.export = async () => {

// Checking Required Node version:
checkNodeVersion();

// Checking Update for npm pkg 
checkupdate();

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

    //'help', 'version', 'log', 'debug'
if (!cli.input._[0]) {
    cli.showWelcomeMsg();
}
    else {
        let allowedFileTypes = ['v','ved','veda'];
        if (!allowedFileTypes.includes(cli.input._[0].split('.')[2])){
            console.log(
                '\n' + chalk.hex('#f44336').inverse.bold(' ERROR: ')+'\n\n' + 
                chalk.hex('#ff1867')('This file type not supported.')+'\n\n' +
                chalk.bold('Supported file types: ') + chalk.yellow('.veda, .ved, .v')+'\n\n'
            );
            cli.end();
        }        
        const { d, l } = cli.input;

        if (d) console.log(chalk.hex('#9966cc').inverse.bold(' Debug Mod '), '\n');

        if (d) console.time('⌛ Total Runtime');

        if (d) spinner.start({ text: 'Loading file...' });

        let file = fetchSource({ file: cli.input._[0], d });

        if (d) spinner.succeed('Data loaded!');

        if (d) spinner.start({ text: 'Interpreter file...' });

        let program = Interpreter(file.data, file.name);

        if (d) spinner.succeed('Interpretered');

        if (d) console.info('\n', chalk.bgHex('#1a7f37').bold(' Result '), '\n');

        Environment.run(program.parser.finalcode);

        console.log();
        if (d) console.timeEnd('⌛ Total Runtime');

        // logs
        if (l) createLogs(file, program);
    }
