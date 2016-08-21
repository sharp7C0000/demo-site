var babelLoader           = require('babel-loader');
var babelTransformRuntime = require('babel-plugin-transform-runtime');

var path        = require("path");
var glob        = require('glob');
var webpack     = require('webpack');

var ENTRY_PATTERN = "./scripts/**/*.entry.js";
var VENDOR_CHUNK  = "vendor";
var VENDORS = [
  "vue",
  "vue/dist/vue.js",
  "vue-router",
  "vuex/dist/vuex.js",
  "vuex",
  "jquery",
  "tether",
  "bootstrap/dist/js/bootstrap.js"
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
  babel: {
    presets: ['es2015'],
    plugins: ['transform-runtime']
  },
  module: {
    loaders: [
      // vue loader
      {
        test: /\.vue$/, 
        loader: 'vue'
      },
      // babel loader
      {
        test: /\.js$/,
        exclude: /(node_modules|bower_components)/,
        loader: 'babel',
        query: {
          presets: ['es2015'],
          plugins: ['transform-runtime']
        }
      },
      // sass loader
      {
        test: /\.scss$/,
        loaders: ["style", "css", "sass"]
      },
      // css loader
      { 
        test: /\.css$/, 
        loader: "style-loader!css-loader" 
      },
      // font loader
      {
        test: /\.(eot|svg|ttf|woff(2)?)(\?v=\d+\.\d+\.\d+)?/,
        loader: 'url'
      }
    ]
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin(VENDOR_CHUNK, VENDOR_CHUNK + ".js"),
    new webpack.ProvidePlugin({
      '$'            : 'jquery',
      'jQuery'       : 'jquery',
      'window.jQuery': 'jquery',
      'window.Tether': 'tether'
    })
  ]
};