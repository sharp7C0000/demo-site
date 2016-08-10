import Vue  from 'vue/dist/vue.js';
import Vuex from 'vuex';

import Service from './service.js';

Vue.use(Vuex);

const state = {
  
  isSubmited: false,
  
  formError: {},
  formData : {
    email   : null,
    password: null
  }
};

const mutations = {

  REQUEST (state) {
    state.formError = {};
    validate(state);
  },

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

function validate(state) {
  for (var key in state.formData) {
    var item = state.formData[key];
    if(!item) {
      state.formError[key] = `${key} cannot be empty`;
    }
  }
}

const actions = {

  // validateForm: ({ commit }, response) => {
  //   // validate local
  //   console.log(response);

  
  // },

  submitLogin: ({ commit, state }) => {
    commit('REQUEST');
    if(Object.keys(state.formError).length == 0) {
      Service.authenticate(state.formData);
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