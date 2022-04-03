// ॥ श्री गणेशाय नमः ॥ 
// नमामि देवं सकलार्थदं तं सुवर्णवर्णं भुजगोपवीतम्ं।
// गजाननं भास्करमेकदन्तं लम्बोदरं वारिभावसनं च॥

const fs = require('fs');
const ora = require('ora');
const pth = require('path');
const chalk = require('chalk');
const cli = require('../src/cli');
const Environment = require('../src/environment');
const createLogs = require("../src/cli/createLogs");
const checkupdate = require("../src/cli/checkupdate");
const handleError = require("../src/cli/handleError")
const { Interpreter } = require('../src/interpreter');
const checkNodeVersion = require("../src/cli/checkNodeVersion");

//HANDLING ERROR
process.on('unhandledRejection', err => {
    handleError(`UNHANDLED ERROR`, err);
});

// Checking Required Node version:
checkNodeVersion();

//checking Update for npm pkg 
checkupdate();

// VERSION
if (cli.input.version) {
    cli.showVersion();
    cli.end();
}

// HELP
else if (cli.input.help) {
    cli.showHelp();
    cli.end();
}

//'help', 'version', 'log', 'debug'
else if (!cli.input._[0]) {
    cli.showWelcomeMsg();
}
else {
    const { d, l } = cli.input;
    const spinner = ora({ text: '' });
    if (d) console.log(chalk.hex('#9966cc').inverse.bold(' Debug Mod '), '\n');
    const fetchSource = (file) => {
        try {
            let path = pth.resolve(file);
            return {
                path,
                dir: pth.dirname(path),
                name: pth.basename(path),
                data: fs.readFileSync(path, 'utf8')
            };
        }
        catch (e) {
            spinner.fail(' Data loading failed!')
            console.log(
                '\n Error:', JSON.stringify(e.message),
                chalk.hex('#FFF').bold('\n Usage: '),
                chalk.green('vedic'),
                chalk.cyan(' [path/to/script.ved]'),
            );
            cli.end();
        };
    };

    if (d) console.time('⌛ Total Runtime');
    if (d) spinner.start({ text: 'Loading file...' });
    let file = fetchSource(cli.input._[0]);
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