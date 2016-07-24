var babelLoader = require('babel-loader');
var path        = require("path");
var glob        = require('glob');
var webpack     = require('webpack');

var ENTRY_PATTERN = "./scripts/**/*.entry.js";
var VENDOR_CHUNK  = "vendor";
var VENDORS = [
  "vue",
  "jquery"
]

var webpackEntry = {};

// common bundles
glob.sync(ENTRY_PATTERN).forEach(function(file){
  var entryName  = path.basename(file).replace(".entry.js", "");
  webpackEntry[entryName] = file;
});

// vendor
webpackEntry[VENDOR_CHUNK] = VENDORS

module.exports = {
  entry: webpackEntry,
  output: {
    path    : __dirname,
    filename: "[name].bundle.js"
  },
  module: {
    // babel loader
    loaders: [
      {
        test: /\.js$/,
        exclude: /(node_modules|bower_components)/,
        loader: 'babel',
        query: {
          presets: ['es2015']
        }
      },
      {
        test: /\.vue$/, 
        loader: 'vue'
      }
    ]
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin(VENDOR_CHUNK, VENDOR_CHUNK + ".js")
  ]
};