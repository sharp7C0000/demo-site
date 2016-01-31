'use strict';

var browserify = require('browserify');
var browserifyCss = require('browserify-css');
var gulp = require('gulp');
var source = require('vinyl-source-stream');
var buffer = require('vinyl-buffer');
var uglify = require('gulp-uglify');
var sourcemaps = require('gulp-sourcemaps');
var gutil = require('gulp-util');
var sass = require('gulp-sass');
var merge = require('merge-stream');
var concat = require('gulp-concat');

// gulp.task('sass', function () {
//   gulp.src('./sass/**/*.scss', './node_modules/bootstrap/dist/css')
//     .pipe(sass({includePaths: ['./node_modules/bootstrap/dist/css']}).on('error', sass.logError))
//     .pipe(gulp.dest('../app/public/css'));
// });

gulp.task('concat', function () {
    var sassStream,
        cssStream;

    //compile sass
    sassStream = gulp.src('./sass/**/*.scss')
      .pipe(sass().on('error', sass.logError))
      .pipe(gulp.dest('./temp/test.css'));

    //select additional css files
    cssStream = gulp.src('./node_modules/bootstrap/dist/css/bootstrap.css');

    //merge the two streams and concatenate their contents into a single file
    return merge(sassStream, cssStream)
        .pipe(concat('test.css'))
        .pipe(gulp.dest('../app/public/css'));
});

gulp.task('javascript', function () {
  // set up the browserify instance on a task basis
  var b = browserify({
    entries: './scripts/test.js',
    debug: true
  });

  return b.bundle()
    .pipe(source('app.js'))
    .pipe(buffer())
    //.pipe(sourcemaps.init({loadMaps: true}))
        // Add transformation tasks to the pipeline here.
        //.pipe(uglify())
        //.pipe(browserifyCss())
        .on('error', gutil.log)
    //.pipe(sourcemaps.write('./'))
    .pipe(gulp.dest('../app/public/js/'));
});

gulp.task('default', ['javascript', 'concat']);
