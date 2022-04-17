const { compile } = require('/home/pandit/.nvm/versions/node/v16.14.2/bin/nexe');

compile({
    input: './main.js',
    output: './vedic.exe',
    ico: './vedic.ico',
    // target: ['linux-x64', 'win-x64', 'mac-x64'],
    build: true
}).then(() => {
    console.log('success')
})