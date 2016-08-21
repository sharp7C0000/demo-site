import Vue  from 'vue/dist/vue.js';
import Http from "../common/http.js";

export default {
  authenticate(request, cb, ecb) {
    return Vue.http.post('auth/authenticate', request)
    .then((response) => cb(response.data))
    .catch((error)   => ecb(error));
  }
}