const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");

module.exports = {
    mode: 'development',

    entry: './src/index.tsx',

    resolve: {
        extensions: [".ts", ".tsx", ".mjs", ".cjs", ".js", ".json"],
        alias: { 'react-dom': '@hot-loader/react-dom' },
        modules: ["node_modules"]
    },

    optimization: {
        splitChunks: {
            cacheGroups: {
                commons: {
                    test: /[\\/]node_modules[\\/]monaco-editor[\\/]/,
                    name: "vendor-editor",
                    chunks: "initial",
                },
                container: {
                    name: "container",
                    chunks: "initial",
                    minChunks: 2
                }
            }
        }
    },

    module: {
        rules: [
            {
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader"
                    }
                ]
            },
            {
                test: /\.html$/,
                loader: "html-loader"
            },
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"]
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
                loader: "source-map-loader"
            }
        ]
    },

    plugins: [
        new webpack.HotModuleReplacementPlugin(),
        new HtmlWebpackPlugin({
            template: "./src/index.html"
        }),
        new MonacoWebpackPlugin({
            languages: ["yaml", "json"]
        })
    ],
};
