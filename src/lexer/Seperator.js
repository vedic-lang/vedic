// ॥ श्री गणेशाय नमः ॥ 

const { tokenList } = require('../TokenList');
const { errorHandler } = require('../errorHandler');

const Seperator = (input) => {
    const charList = [];
    let line = 1;
    let col = 1;
    for (let char of input.split('')) {
        for (let [token, verification] of Object.entries(tokenList)) {
            if (verification(char)) {
                const charDescription = {};
                charDescription['type'] = token;
                charDescription['line'] = line;
                charDescription['col'] = col;
                charDescription['value'] = char;
                if (charDescription.type == "UNRECOGNIZED") {
                    errorHandler(`UNRECOGNIZED SYNTAX`, charDescription['line'], charDescription['col']);
                } else {
                    charList.push(charDescription);
                };
                break;
            }
        }
        if (char == '\n') {
            col = 1;
            line++;
        } else {
            col++;
        };
    };
    return charList;
}

module.exports = {
    Seperator
};