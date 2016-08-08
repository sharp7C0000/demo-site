import Vue         from 'vue/dist/vue.js';
import VueResource from 'vue-resource';

Vue.use(VueResource);

export default {
  authenticate(request) {
    return Vue.http.post('auth/authenticate', request)
    .then((response) => Promise.resolve(response.data))
    .catch((error) => Promise.reject(error));
  }
}