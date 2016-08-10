<style lang="sass">
  @import '~bootstrap/scss/bootstrap-flex';
  @import '.~bootstrap/scss/mixins/_breakpoints';
  @import '~bootstrap/scss/mixins/_grid';
  @import '~bootstrap/scss/_variables';

  @include media-breakpoint-up(xs) {
    .login-container {
      @include make-col(12);
      padding-right: 20px;
      padding-left : 20px;
    }
  }

  @include media-breakpoint-up(sm) {
    .login-container {
      @include make-col-offset(3);
      @include make-col(6);
      @include make-col-offset(3);
    }
  }

  @include media-breakpoint-up(lg) {
    .login-container {
      @include make-col-offset(4);
      @include make-col(4);
      @include make-col-offset(4);
    }
  }
</style>

<template>
  <login-form inline-template>
    <div class="login-container">
      <div class="card">
        <div class="card-block">
          <h3 class="card-title">LOGIN</h3>
          <form action="/dologin" method="POST" v-on:submit.prevent="submitLogin">
            <fieldset class="form-group" v-bind:class="{'has-danger': formError.email}">
              <label>Email</label>
              <input type="text" class="form-control"  v-bind:class="{'form-control-danger': formError.email}" placeholder="Email" :value="formData.email" @input="updateEmail">
              <div v-if="formError.email" class="form-control-feedback" v-text="formError.email"></div>
            </fieldset>

            <fieldset class="form-group" v-bind:class="{'has-danger': formError.password}">
              <label>Password</label>
              <input type="password" class="form-control" v-bind:class="{'form-control-danger': formError.password}" placeholder="Password" :value="formData.password" @input="updatePassword">
              <div v-if="formError.password" class="form-control-feedback" v-text="formError.password"></div>
            </fieldset>
            <!--<div class="alert alert-danger" role="alert" v-if="formError">
              <strong>Oh snap!</strong> {{formError}}
            </div>-->
            <button type="submit" v-on:click="submitLogin" class="btn btn-primary btn-block">Login</button>
          </form>
        </div>
      </div>
    </div>
  </login-form>
</template>

<script>

  import { mapGetters, mapActions } from 'vuex';

  export default {

    ready () {
      console.log("ready!!");
    },

    components: {
      
      "login-form": {
        
        computed: mapGetters([
          'formError',
          'formData'
        ]),
        
        methods: mapActions([
          'submitLogin',
          'updateEmail',
          'updatePassword'
        ])
      }
    },
    
    //store

  }
</script>
