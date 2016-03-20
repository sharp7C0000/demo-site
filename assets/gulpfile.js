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

  glob('./scripts/main-**.js', function(err, files) {
    if(err) done(err);

    var tasks = files.map(function(entry) {

      var b = browserify({ entries: [entry] })
      .transform(babelify, {presets: ["es2015"]});

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
  gulp.watch('./sass/**/*.scss',
             ['sass']);
});

gulp.task('clean', function () {
  return gulp.src('../app/public/*')
		.pipe(clean({force: true}));
});

// define the browserify-watch as dependencies for this task
gulp.task('watch', ['compile', 'browserify-watch', 'sass-watch']);
gulp.task('compile', ['clean'], function() {
  gulp.start("browserify");
  gulp.start("sass");
  gulp.start("resource");
});
