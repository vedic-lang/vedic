const { red } = require('chalk');

module.exports = (requiredNodeVersion = "12") => {
    const currentNodeVersion = process.versions.node;
    if (currentNodeVersion.split('.')[0] < Math.abs(requiredNodeVersion)) {
        console.error(red.inverse.bold('✖ Node version issue!'));
        console.error(`\nRequired Node version: ${requiredNodeVersion} or higher.`);
        console.error(`Current Node version: ${currentNodeVersion}`);
        console.error('Please update version of Node.js to run this program.');
        console.error('https://nodejs.org/en/download');
        process.exit(0);
    };
};
