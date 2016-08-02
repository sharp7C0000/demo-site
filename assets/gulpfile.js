'use strict';

var gulp = require('gulp');
var gutil = require('gulp-util');;

var ncu              = require('npm-check-updates');
var WebpackDevServer = require("webpack-dev-server");
var webpackConfig    = require("./webpack.config.js");
var webpack          = require('webpack');
var KarmaServer      = require('karma').Server;

/**
 * 업그레이드 해야 할 node package가 있는지 확인
 */
gulp.task('ncu', function(done) {
  ncu.run({
    packageFile: 'package.json',
    silent: false,
    jsonUpgraded: true
  }).then(function(upgraded) {
    gutil.log('dependencies to upgrade:', upgraded);
    done();
  });
});

/**
 * webpack dev server 실행
 */
gulp.task("webpack-dev-server", function(callback) {

  var IP   = "0.0.0.0";
  var PORT = 6768;

	var conf = Object.create(webpackConfig);
	conf.devtool = "eval";
	conf.debug   = true;

	new WebpackDevServer(webpack(conf), {
		publicPath : "/assets/",
    historyApiFallback: true,
    hot               : false,
    watchOptions: {
      aggregateTimeout: 300,
      poll            : 1000
    },
		stats: {
			colors: true
		}
  })
  .listen(PORT, IP, function(err) {
		if(err) throw new gutil.PluginError("webpack-dev-server", err);
		gutil.log("[webpack-dev-server]", "http://" + IP + ":" + PORT + "/webpack-dev-server")
  });
});

/**
 * TODO : test 
 */
gulp.task('test', function (done) {
  new KarmaServer({
    configFile: __dirname + '/karma.conf.js',
    singleRun: true
  }, done).start();
});

/**
 * start task
 */
gulp.task("dev", ['ncu'], function(callback){
  gulp.start("webpack-dev-server");
});