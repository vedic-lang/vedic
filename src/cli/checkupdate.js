const pkgJSON = require('../../package.json');
const checkForUpdate = require('update-check');
const chalk = require('chalk');
const boxen = require('boxen');

module.exports = async () => {
    try {
        const update = await checkForUpdate(pkgJSON); //{ name: "vedicscript", version: "1.0.3" }
        if (update) {
            console.log(
                boxen(`
${chalk.hex('#ff9933').inverse.bold(' UPDATE AVAILABLE! ')}
${chalk.green(`Current Version: ${pkgJSON.version}`)}
Run ${chalk.green(`npm i -g ${pkgJSON.name}@latest`)} for v${update.latest}
${pkgJSON.homepage}\n`, { padding: 1, dimBorder: false, align: `center` })
            );
        }
    } catch (err) {
        console.error(`Failed to check for updates: ${err}`);
    }
};

