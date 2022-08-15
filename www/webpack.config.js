const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  devServer: {
    disableHostCheck: true,
    client: {
      webSocketURL: 'auto://0.0.0.0:0/ws'
    },
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
