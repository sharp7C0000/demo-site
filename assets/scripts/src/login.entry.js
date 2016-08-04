import Vue       from 'vue/dist/vue.js';
import Vuex      from 'vuex';
import VueRouter from 'vue-router';
import Topbar    from "./common/topbar.vue";
import App       from "./login/app.vue";


import store from "./login/store.js";


require("../../sass/app.scss");

Vue.use(VueRouter);
Vue.use(Vuex);

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
  store,
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
