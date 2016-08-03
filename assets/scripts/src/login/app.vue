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
  <div class="login-container">
    <div class="card">
      <div class="card-block">
        <h3 class="card-title">LOGIN</h3>
        <form action="#" v-on:submit.prevent="submit">
          <fieldset class="form-group">
            <label>Email</label>
            <input type="text" class="form-control" placeholder="Email" v-model="formData.email">
          </fieldset>
          <fieldset class="form-group">
            <label>Password</label>
            <input type="password" class="form-control" placeholder="Password" v-model="formData.password">
          </fieldset>
          <button type="submit" class="btn btn-primary btn-block">Login</button>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
  export default {

    data () {
      return {
        formData: {}
      }
    },

    ready () {
      console.log("ready!!");
    }
  }
</script>
