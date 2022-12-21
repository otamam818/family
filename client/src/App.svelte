<script>
  import { Router, Route } from "svelte-navigator";
  import Login from "./Login/Login.svelte";
</script>

<main>
  <Router>
    <Route path="/">
      <Login />
    </Route>
    <Route path="login">
      <div> Mario </div>
    </Route>
  </Router>
</main>

<style>
</style>
