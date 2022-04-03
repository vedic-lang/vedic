const ora = require('ora');
const fs = require('fs');
const path = require('path');
const chalk = require('chalk');
const spinner = ora({ text: '' });

module.exports = ({file,d})=> {
    try {
        let output = {}
        output.path = path.resolve(file);
        output.dir = path.dirname(output.path);
        output.name = path.basename(output.path);
        output.data = fs.readFileSync(output.path, 'utf8');
        return output;
    }
    catch (e) {
        if (d) spinner.fail(' Data loading failed!');
        console.log(
            '\n Error:', JSON.stringify(e.message),
            chalk.hex('#FFF').bold('\n Usage: '),
            chalk.green('vedic'),
            chalk.cyan(' [path/to/script.ved]')
        );
        cli.end();
    };
};
