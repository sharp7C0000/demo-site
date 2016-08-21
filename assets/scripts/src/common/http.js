import Vue         from 'vue/dist/vue.js';
import VueResource from 'vue-resource';

import NProgress from "nprogress";
require("../../../sass/components/nprogress.scss");

Vue.use(VueResource);

NProgress.configure({showSpinner: false});

// common interceptor
Vue.http.interceptors.push((request, next) => {
  NProgress.start();
  next((response) => {
    NProgress.done();
  });
});