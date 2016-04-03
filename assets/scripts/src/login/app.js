/** app component **/

// generate Vue

export default {

  data () {
    return { formData: {
      email   : null,
      password: null
    }}
  },

  methods: {
    submit () {
      console.log(this.$data);
    }
  },

  ready () {
    // ready
  }

}
