<script>
  import { Router, Route } from "svelte-navigator";
  import DashboardPage from "./Dashboard/DashboardPage.svelte";
  import LoginPage from "./Login/LoginPage.svelte";
</script>

<main>
  <Router>
    <Route path="/">
      <LoginPage />
    </Route>
    <Route primary={false} path="dashboard">
      <DashboardPage />
    </Route>
  </Router>
</main>

