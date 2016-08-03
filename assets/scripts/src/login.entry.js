import Vue       from 'vue/dist/vue.js';
import VueRouter from 'vue-router';
import Topbar    from "./common/topbar.vue";
import App       from "./login/app.vue";

require("../../sass/app.scss");

Vue.use(VueRouter);

Vue.component('topbar', Topbar);

const router = new VueRouter({
  mode: 'history',
  base: __dirname,
  routes: [
    { path: '/login', component: App }
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
