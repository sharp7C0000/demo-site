global.jQuery = require('jquery');
global.Tether = require('tether');
require("bootstrap");

import { default as App } from "./login/app.js";
import Vue from "vue";

// TODO : VUE validator

// generate Vue
new Vue({el: "#login", components: {app: App}});
