const { red, blue, yellow } = require('chalk');

module.exports = (heading = `ERROR: `, err, displayError = true, exit = true) => {
    if (err) {
        console.log();
        if (displayError) {
            console.log(red('✖',heading));
            console.log(red('✖ ERROR →'),err.name);
            console.log(blue('ℹ'),red('REASON →'),err.message);
            console.log(blue('ℹ'),red('ERROR STACK ↓ \n'),err.stack,'\n');
        } else {
            console.log(yellow('⚠',heading),'\n');
        }
        if (exit) {
            process.exit(0);
        } else {
            return false;
        }
    }
};
