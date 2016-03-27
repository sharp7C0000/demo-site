'use strict';

module.exports = function(karma) {
  karma.set({

    frameworks: [ 'jasmine', 'browserify' ],

    files: [
      'scripts/test/**/*.js'
    ],

    reporters: [ 'dots' ],

    preprocessors: {
      //'scripts/src/*.js': [ 'browserify' ],
      'scripts/test/*.js': [ 'browserify' ]
    },

    browsers: [ 'Chrome' ],

    logLevel: 'LOG_DEBUG',

    singleRun: true,
    autoWatch: false,

    // browserify configuration
    browserify: {
      debug: true,
      transform: [['babelify', {presets: ["es2015"]}]]
    }
  });
};
