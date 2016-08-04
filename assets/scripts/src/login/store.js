import Vue       from 'vue/dist/vue.js';
import Vuex from 'vuex';

Vue.use(Vuex);

const state = {
  error: false
};

const mutations = {
  ERROR (state, message) {
    state.error = message;
  }
};

const actions = {
  errorOccur: function ({ commit, state }) {
    commit('ERROR', "has error");
  }
};

// getters are functions
const getters = {
  getError: state => state.error
};

// Combine the initial state and the mutations to create a Vuex store.
// This store can be linked to our app.
export default new Vuex.Store({
  state,
  getters,
  actions,
  mutations
});