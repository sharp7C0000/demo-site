import Vue  from 'vue/dist/vue.js';
import Vuex from 'vuex';

import Service from './service.js';

Vue.use(Vuex);

const state = {
  
  loginStatus: null,
  
  // local error
  formError: {},
  formData : {
    email   : null,
    password: null
  }
};

const mutations = {

  LOGIN_REQUEST (state) {
    state.formError   = {};
    state.loginStatus = null;
    validate(state);
  },

  LOGIN_READY (state) {
    state.loginStatus = 'requesting';
  },

  LOGIN_SUCCESS (state) {
    state.loginStatus = 'successful';
  },

  LOGIN_FAIL (state, reason) {
    state.loginStatus   = 'fail';
    if(!reason) {
      state.formError[""] = "Cannot connect the server";
    }
  },

  FORM_DATA_EMAIL (state, val) {
    state.formData.email = val;
  },
  FORM_DATA_PASSWORD (state, val) {
    state.formData.password = val;
  }
};

function validate(state) {
  for (var key in state.formData) {
    var item = state.formData[key];
    if(!item) {
      state.formError[key] = `${key} cannot be empty`;
    }
  }
}

const actions = {

  submitLogin: ({ commit, state }) => {

    if(state.loginStatus == "requesting") {
      return ;
    }

    commit('LOGIN_REQUEST');
    if(Object.keys(state.formError).length == 0) {
      commit('LOGIN_READY');
      Service.authenticate(state.formData, (resp) => {
        commit('LOGIN_SUCCESS');
      }, (resp) => {
        commit('LOGIN_FAIL', resp.data);
      });
    }
  },

  updateEmail: ({ commit }, e) => {
    commit('FORM_DATA_EMAIL', e.target.value);
  },
  updatePassword: ({ commit }, e) => {
    commit('FORM_DATA_PASSWORD', e.target.value);
  }
};

// getters are functions
const getters = {
  formError  : state => state.formError,
  formData   : state => state.formData,
  loginStatus: state => state.loginStatus
};

// Combine the initial state and the mutations to create a Vuex store.
// This store can be linked to our app.
export default new Vuex.Store({
  state,
  getters,
  actions,
  mutations
});