const { merge } = require("webpack-merge");
const common = require("./webpack.config.js");
const ReactRefreshWebpackPlugin = require("@pmmmwh/react-refresh-webpack-plugin");

module.exports = merge(common, {
    mode: "development",
    devtool: "source-map",
    entry: "./src/index.dev.tsx",

    module: {
        rules: [
            {
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "babel-loader",
                        options: { plugins: ["react-refresh/babel"] },
                    },
                    {
                        loader: "ts-loader",
                        options: {
                            transpileOnly: true,
                        },
                    },
                ],
            },
        ],
    },
    plugins: [new ReactRefreshWebpackPlugin()],
});
