import Vue  from 'vue/dist/vue.js';
import Vuex from 'vuex';

import Service from './service.js';

Vue.use(Vuex);

const state = {
  formError: {},
  formData: {
    email   : null,
    password: null
  }
};

const mutations = {
  FORM_ERROR (state, message) {
    state.formError = message;
  },
  FORM_DATA_EMAIL (state, val) {
    state.formData.email = val;
  },
  FORM_DATA_PASSWORD (state, val) {
    state.formData.password = val;
  }
};

const actions = {

  submitLogin: ({ commit }, response) => {
    // submit this!!!
    Service.authenticate(response);
  },
  updateEmail: ({ commit }, e) => {
    commit('FORM_DATA_EMAIL', e.target.value);

    if(e.target.value.length < 1) {
      commit('FORM_ERROR', {email: "입력을 해야 합니다"});
    } else {
      commit('FORM_ERROR', {email: null});
    }
  },
  updatePassword: ({ commit }, e) => {
    commit('FORM_DATA_PASSWORD', e.target.value);
  }
};

// getters are functions
const getters = {
  formError: state => state.formError,
  formData : state => state.formData
};

// Combine the initial state and the mutations to create a Vuex store.
// This store can be linked to our app.
export default new Vuex.Store({
  state,
  getters,
  actions,
  mutations
});