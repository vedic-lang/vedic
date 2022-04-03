// ॥ श्री गणेशाय नमः ॥ 

module.exports = {
    bolo: (...args) => {
        try {
            console.log(...args);
        } catch (e) {
            console.error(e);
        }
    }
};