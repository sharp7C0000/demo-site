<style lang="sass">
$font-stack:    Helvetica, sans-serif;
  .blue {
    color: blue;
  }
</style>

<template>

  <div>

    <h1>HI <span class="blue">{{message}}</blue></h1>
    <!-- Button trigger modal -->
    <button type="button" class="btn btn-primary btn-lg" data-toggle="modal" data-target="#myModal">
      Launch demo modal
    </button>

    <!-- Modal -->
    <div class="modal fade" id="myModal" tabindex="-1" role="dialog" aria-labelledby="myModalLabel" aria-hidden="true">
      <div class="modal-dialog" role="document">
        <div class="modal-content">
          <div class="modal-header">
            <button type="button" class="close" data-dismiss="modal" aria-label="Close">
              <span aria-hidden="true">&times;</span>
            </button>
            <h4 class="modal-title" id="myModalLabel">Modal title</h4>
          </div>
          <div class="modal-body">
            ...
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
            <button type="button" class="btn btn-primary">Save changes</button>
          </div>
        </div>
      </div>
    </div>

  </div>
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