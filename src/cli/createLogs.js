const fs = require('fs');
module.exports = (file, program) => {
    try {
        let dir = `${file.dir}/logs`;
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
        }
        fs.writeFileSync(`${file.dir}/${file.name}.log`,
            JSON.stringify(program),
            { encoding: "utf8", flag: "w" }
        );
        console.info(`\nLog saved at : ${file.dir}/${file.name}.log\n`);

    } catch (err) {
        console.error(err);
    }
};