//import A from "./mtest/a.js";
//import B from "./mtest/b.js";

import Vue from 'vue/dist/vue.js';
import $ from "jquery";

global.$ = $;

import Comp from "./index/comp.vue";

let k = 3000;

let array = [1,2,3,4,5,6];

array.map((a) => {
  console.log(a);
  console.log(a);
});

console.log("hear!!!!");

new Vue({
  el: "#app",

  ready () {
    console.log("readey");
  },

  components: {
    "test": Comp
  }

});

//alert("k");