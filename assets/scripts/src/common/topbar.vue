<style lang="sass">
  @import '~bootstrap/scss/bootstrap-flex';
  @import '~bootstrap/scss/mixins/_breakpoints';

  @include media-breakpoint-up(xs) {
    .top-bar {
      .button-title {
        display: none;
      }
    }
  }

  @include media-breakpoint-up(sm) {
    .top-bar {
      .button-title {
        display: inline!important;
      }
    }
  }
</style>

<template>
  <nav class="top-bar navbar navbar-full navbar-fixed-top navbar-dark bg-primary">
    <div class="navbar-toggleable-xs">
      <router-link class="navbar-brand" tag="a" :to="{path: '/'}" v-text="title"></router-link>
      <div class="pull-xs-right">
        <a class="btn btn-outline-success" href="http://devblog-sharp7c0000.rhcloud.com/" title="Go to dev blog" target="_blank">
          <span class="button-title">Go to dev blog</span> <i class="fa fa-external-link"></i>
        </a>
      </div>
    </div>
  </nav>
</template>

<script>

  export default {

    data () {
      return {
        title: "Shitty Code Storage"
      }
    },

    mounted   () {
      console.log("readed", this);

    }
  }
</script>