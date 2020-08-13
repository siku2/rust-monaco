const webpack = require("webpack");
const EsmWebpackPlugin = require("@purtuga/esm-webpack-plugin");
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");

module.exports = {
  mode: "production",
  entry: "./monaco.js",
  output: {
    filename: "monaco.js",
    library: "MONACO_MOD",
    libraryTarget: "var",
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.ttf$/,
        use: ["file-loader"],
      },
    ],
  },
  plugins: [
    new MonacoWebpackPlugin({ languages: ["css", "html", "json", "rust"] }),
    new webpack.optimize.LimitChunkCountPlugin({
      maxChunks: 1,
    }),
    new EsmWebpackPlugin(),
  ],
};
