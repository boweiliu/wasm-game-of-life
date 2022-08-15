// const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  devServer: {
    // disableHostCheck: true,
    allowedHosts: ['all'],
    // client: { webSocketURL: 'auto://0.0.0.0:0/ws' },
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    // new CopyWebpackPlugin(['index.html'])
    new HtmlWebpackPlugin({ template: 'index.html' }),
  ],
};
