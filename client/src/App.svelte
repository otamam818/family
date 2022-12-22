<script>
  import { Router, Route } from "svelte-navigator";
  import LoginPage from "./Login/LoginPage.svelte";
</script>

<main>
  <Router>
    <Route path="/">
      <LoginPage />
    </Route>
    <Route path="login">
      <div> Mario </div>
    </Route>
  </Router>
</main>

<style>
</style>
