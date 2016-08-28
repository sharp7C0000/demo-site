import Vue  from 'vue/dist/vue.js';
import Http from "../common/http.js";

export default {
  authenticate(request, cb, ecb) {
    return Vue.http.post('login', request)
    .then((response) => cb(response.data))
    .catch((error)   => ecb(error));
  }
}