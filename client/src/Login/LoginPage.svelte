<script>
  import LoginForm from "./LoginForm.svelte";
  import SignupForm from "./SignupForm.svelte";
  import Splash from "./Splash.svelte";

  let currComponent = "Form"
  function updateComponenent (value) {
    currComponent = value;
  };
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
