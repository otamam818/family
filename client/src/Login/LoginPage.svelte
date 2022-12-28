<script>
  import LoginForm from "./LoginForm.svelte";
  import SignupForm from "./SignupForm.svelte";
  import Splash from "./Splash.svelte";
  import {valid_session} from "../dataHandler/requestHandler";

  let currComponent = "Form"
  /** @param value {string} */
  function updateComponenent (value) {
    currComponent = value;
  };

  valid_session()
    .then((session) => {
      if (session.exists) {
        window.location.href = "dashboard";
      }
    })
    // The uncoming error says that the session is undefined, which happens
    // whenever there is no reference token
    .catch(_ => {});

</script>

<div class="login-page">
  {#if currComponent === "Form"}
    <LoginForm updateComponenent={updateComponenent} />
  {:else}
    <SignupForm updateComponenent={updateComponenent} />
  {/if}
  <Splash />
</div>

<style>
</style>
