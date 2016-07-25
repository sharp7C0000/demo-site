<style lang="sass">
$font-stack:    Helvetica, sans-serif;
  .blue {
    color: blue;
  }
</style>

<template>
  <h1>HI <span class="blue">{{message}}</blue></h1>
</template>

<script>
  import Vue from "vue";

  export default {

    data () {
      return {
        message: "hihi"
      }
    },

    ready () {
      console.log("readed");
    }
  }
</script>