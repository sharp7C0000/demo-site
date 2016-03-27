global.jQuery = require('jquery');
global.Tether = require('tether');
require("bootstrap");

var Vue = require("vue");

// TODO : VUE validator

// generate Vue
new Vue({
  el: "#login",

  data: {
    formData: {
      email   : null,
      password: null
    }
  },

  methods: {
    submit () {
      console.log(this.$data);
    }
  },

  ready () {
    // ready
  }

});
