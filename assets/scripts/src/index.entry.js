import Vue       from 'vue/dist/vue.js';
import VueRouter from 'vue-router';
import Topbar    from "./index/topbar.vue";
import Test      from "./index/test.vue";

require("../../sass/app.scss");

Vue.use(VueRouter);

Vue.component('topbar', Topbar);

const router = new VueRouter({
  mode: 'history',
  base: __dirname,
  routes: [
    { path: '/', component: Test }
  ]
});

new Vue({
  router,
  template: `
    <div id="app">
      <topbar></topbar>
      <div id="container" class="container">
        <div class="row">
          <router-view></router-view>
        </div>
      </div>
    </div>
  `
}).$mount('#app');
