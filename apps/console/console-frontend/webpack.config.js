const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");
const Dotenv = require("dotenv-webpack");

module.exports = {
    mode: "development",

    entry: "./src/index.tsx",

    resolve: {
        extensions: [".ts", ".tsx", ".mjs", ".cjs", ".js", ".json"],
        modules: ["node_modules"],
    },

    optimization: {
        splitChunks: {
            cacheGroups: {
                commons: {
                    test: /[\\/]node_modules[\\/]monaco-editor[\\/]/,
                    name: "vendor-editor",
                    chunks: "initial",
                },
            },
        },
    },

    module: {
        rules: [
            {
                test: /\.html$/,
                loader: "html-loader",
            },
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"],
            },
            {
                test: /\.ttf$/,
                use: ["file-loader"],
            },
            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            {
                enforce: "pre",
                test: /\.js$/,
                exclude: /node_modules\/@mrblenny/, //avoid warinig...
                loader: "source-map-loader",
            },
        ],
    },

    plugins: [
        new webpack.HotModuleReplacementPlugin(),
        new HtmlWebpackPlugin({
            template: "./src/index.html",
        }),
        new MonacoWebpackPlugin({
            languages: ["yaml", "json"],
        }),
        new Dotenv({}),
    ],
};
