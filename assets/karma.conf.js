'use strict';

module.exports = function(karma) {

  karma.set({

    frameworks: ['jasmine-jquery', 'jasmine', 'browserify'],

    files: [
      {pattern: 'scripts/test/spec-*.js'},
      {pattern: 'scripts/test/fixtures/*.html', watched: true, included: false, served: true}
    ],

    reporters: ['dots'],

    preprocessors: {
      'scripts/test/spec-*.js': [ 'browserify' ]
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
