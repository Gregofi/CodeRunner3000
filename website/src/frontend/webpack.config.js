const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin');
const path = require('path');

module.exports = {
    entry: './ts/index.ts',
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
				test: /\.css$/,
				use: ['style-loader', 'css-loader']
			},
            {
				test: /\.ttf$/,
				type: 'asset/resource'
			}
        ],
    },
    resolve: {
        extensions: ['.ts' ,'.js']
    },
    output: {
        filename: 'bundled.js',
        path: path.resolve(__dirname, '../static/js'),
    },
    plugins: [
        new MonacoWebpackPlugin({
            languages: ['typescript', 'javascript', 'css', 'python']
        })
    ],
}
