const { merge } = require('webpack-merge');
const common = require('./webpack.config.js');

module.exports = merge(common, {
    mode: 'development',
    devtool: "source-map",
    entry: './src/index.dev.tsx',

    module: {
        rules: [
            {
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "react-hot-loader/webpack"
                    },
                    {
                        loader: "ts-loader"
                    }
                ]
            },
        ]
    }
});
