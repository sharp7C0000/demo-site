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

gulp.task('browserify', function(){
  browserifyTask(false);
});

gulp.task('browserify-watch', function(){
  browserifyTask(true);
});

var browserifyTask = function(watch) {

  function bundle(b) {
    b.bundle()
      .pipe(source('app.js'))
      .pipe(buffer())
      .pipe(sourcemaps.init({loadMaps: true}))
          // Add transformation tasks to the pipeline here.
          //.pipe(uglify())
          //.pipe(browserifyCss())
          .on('error', gutil.log)
      .pipe(sourcemaps.write('./'))
      .pipe(gulp.dest('../app/public/js/'));
  }

  // set up the browserify instance on a task basis
  var b = browserify({
    entries: './scripts/app.js',
    debug: true
  })
  .transform(babelify, {presets: ["es2015"]})

  if(watch) {
    b = watchify(b);
    b.on('update', function(){
      bundle(b);
    });
  }

  return bundle(b);
}

gulp.task('sass', function() {
  return gulp.src('./sass/**/*.scss')
    .pipe(sass().on('error', sass.logError))
    .pipe(gulp.dest('../app/public/css'));
});

gulp.task('sass-watch', function(){
  // watch other files, for example .less file
  gulp.watch('./sass/**/*.scss',
             ['sass']);
});

gulp.task('clean', function () {
  return gulp.src('../app/public')
		.pipe(clean({force: true}));
});

// define the browserify-watch as dependencies for this task
gulp.task('watch', ['browserify-watch', 'sass-watch']);
gulp.task('compile', ['browserify', 'sass']);
