import Vue       from 'vue/dist/vue.js';
import Comp      from "./index/comp.vue";
import Topbar    from "./index/topbar.vue";

require("../../sass/app.scss");

new Vue({
  el: "#app",

  ready () {
    console.log("readey");
  },

  components: {
    "topbar": Topbar,
    "test"  : Comp
  }

});