<!DOCTYPE html>
<html>
  <head>
    {{>partial/header}}
    <link rel="stylesheet" type="text/css" href="css/login.css">

    <title>Shitty Code Storage : login</title>
  </head>

  <body id="login">

    {{>partial/topbar}}

    <!-- body container -->
    <app inline-template>
      <div id="container" class="container">
        <div class="row">
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
        </div>
      </div>
    </app>

    <footer>
      <script src="js/login.js"></script>
    </footer>
  </body>
</html>
