import Vue         from 'vue/dist/vue.js';
import VueResource from 'vue-resource';

Vue.use(VueResource);

export default {
  authenticate(request, cb, ecb) {
    return Vue.http.post('auth/authenticate', request)
    .then((response) => cb(response.data))
    .catch((error)   => ecb(error));
  }
}