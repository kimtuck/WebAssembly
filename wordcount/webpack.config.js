// webpack.config.js
const path = require('path');

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
    },
    resolve: {
        alias: {
            forge: 'node_modules/forge/forge.bundle.js'
        }
    },
    mode: "development"
};