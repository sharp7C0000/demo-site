'use strict';

var browserify = require('browserify');
var watchify = require('watchify');
var babelify = require('babelify');
var browserifyCss = require('browserify-css');
var gulp = require('gulp');
var clean = require('gulp-clean');
var source = require('vinyl-source-stream');
var buffer = require('vinyl-buffer');
var uglify = require('gulp-uglify');
var sourcemaps = require('gulp-sourcemaps');
var gutil = require('gulp-util');
var sass = require('gulp-sass');
var merge = require('merge-stream');
var concat = require('gulp-concat');
var glob   = require('glob');
var es     = require('event-stream');
var rename = require('gulp-rename');
var es2015 = require('babel-preset-es2015');


var ncu     = require('npm-check-updates');
var webpack = require('webpack-stream');
var babelLoader = require('babel-loader');

const SCRIPT_PATH = "./scripts";


var Server = require('karma').Server;

gulp.task('browserify', function(done){
  browserifyTask(false, done);
});

gulp.task('browserify-watch', function(done){
  browserifyTask(true, done);
});

var browserifyTask = function(watch, done) {

  function bundle(b, entry) {

    var fileNameRegex = /[\w-]+\./;
    var fileName      = fileNameRegex.exec(entry)[0].replace(".", ".js").replace("main-", "");

    return b
    .bundle()
    .pipe(source(fileName))
    .pipe(gulp.dest('../app/public/js/'));
  };

  glob('./scripts/src/main-**.js', function(err, files) {
    if(err) done(err);

    var tasks = files.map(function(entry) {

      var b = browserify({ entries: [entry] })
      .transform(babelify, {presets: [es2015]});

      if(watch) {
        var wb = watchify(b);
        wb.on('update', function(){
          return bundle(b, entry);
        });
      }

      return bundle(b, entry);
    });

    es.merge(tasks).on('end', done);
  });
}

gulp.task('sass', function() {
  return gulp.src('./sass/**/*.scss')
    .pipe(sass().on('error', sass.logError))
    .pipe(gulp.dest('../app/public/css'));
});

gulp.task('resource', function () {
  return gulp.src('./node_modules/font-awesome/fonts/**.*', {base: './node_modules/font-awesome/fonts/'})
  .pipe(gulp.dest('../app/public/fonts'));
});

gulp.task('sass-watch', function(){
  // watch other files, for example .less file
  gulp.watch('./sass/**/*.scss', ['sass']);
});

gulp.task('clean', function () {
  return gulp.src('../app/public/*')
		.pipe(clean({force: true}));
});

gulp.task('test', function (done) {
  new Server({
    configFile: __dirname + '/karma.conf.js',
    singleRun: true
  }, done).start();
});

// define the browserify-watch as dependencies for this task
gulp.task('watch', ['ncu'], function () {
  gulp.start("compile");
  gulp.start('browserify-watch');
  gulp.start('sass-watch');
  gulp.start("compile");
});

gulp.task('compile', ['clean'], function() {
  gulp.start("browserify");
  gulp.start("sass");
  gulp.start("resource");
});

//////////////////////////////////////////////////////////////////////////////////

///////////////// new task //////////////////////////////////////////////////////

gulp.task('ncu', function(done) {
  ncu.run({
    // Always specify the path to the package file 
    packageFile: 'package.json',
    // Any command-line option can be specified here. 
    // These are set by default: 
    silent: true,
    jsonUpgraded: true
  }).then(function(upgraded) {
    gutil.log('dependencies to upgrade:', upgraded);
    done();
  });
});

gulp.task('webtest', function() {
  return gulp.src(SCRIPT_PATH + '/src/main-mtest.js')
    .pipe(webpack({
   // watch: true,
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
        }
      ],
    },

    output: {
            filename: "[name].js"
          }

  }))
    .pipe(gulp.dest('dist/'));
});

