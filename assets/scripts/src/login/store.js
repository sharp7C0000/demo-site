import Vue       from 'vue/dist/vue.js';
import Vuex from 'vuex';

Vue.use(Vuex);

const state = {
  error   : null,
  formData: {
    email   : null,
    password: null
  }
};

const mutations = {
  ERROR (state, message) {
    state.error = message;
  },
  FORM_DATA_EMAIL (state, val) {
    state.formData.email = val;
  },
  FORM_DATA_PASSWORD (state, val) {
    state.formData.password = val;
  }
};

const actions = {
  errorOccur: function ({ commit, state }) {
    commit('ERROR', "has error");
  },
  updateEmail: ({ commit }, e) => {
    commit('FORM_DATA_EMAIL', e.target.value);

    if(e.target.value.length < 10) {
      commit('ERROR', "글자 수가 작음");
    } else {
      commit('ERROR', null);
    }
  },
  updatePassword: ({ commit }, e) => {
    commit('FORM_DATA_PASSWORD', e.target.value);
  }
};

// getters are functions
const getters = {
  getError   : state => state.error,
  getFormData: state => state.formData
};

// Combine the initial state and the mutations to create a Vuex store.
// This store can be linked to our app.
export default new Vuex.Store({
  state,
  getters,
  actions,
  mutations
});