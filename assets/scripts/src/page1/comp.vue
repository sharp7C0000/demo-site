<style lang="sass">
  BODY {
    background-color: blue;
  }
</style>

<template>
  <h1>HI {{message}}</h1>
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