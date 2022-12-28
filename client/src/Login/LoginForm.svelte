<script>
  import "./form.scss";
  import { onMount } from 'svelte';
  import {formClass} from "./formHandler";
  import {handleSubmit} from "./LoginSubmit.js";
  import Form from "./Form.svelte";
  import {ACCESS_KEY} from "../dataHandler/cookieHandler";
  export let updateComponenent;

	let usernameInput
  onMount(() => {
    setTimeout(() => {
      usernameInput.focus()
    }, 1200);
  })

  onMount(() => {
    setTimeout(() => {
      usernameInput.focus()
    }, 1200);
  })
</script>

<Form>
  <h2> Login </h2>
  <label for="username"> Username </label>
  <input id="username" type="text" bind:this={usernameInput}>
  <label for="password"> Password </label>
  <input id="password" type="password">
  <span
    on:keydown={(e) => console.log(e)}
    on:click={() => {
      formClass.update(() => "fade-out");
      setTimeout(() => {
        updateComponenent("NewUser");
        formClass.update(() => "spread squeeze-in");
      }, 400);
  }}> Make a new user </span>

  <button on:click|preventDefault={() => {
    handleSubmit()
      .then(token => {
        console.log(token);
        if (token === "Inavlid credentials") {
          // Unset the key for safety
          document.cookie = `${ACCESS_KEY}=`;
          return;
        }
        document.cookie = `${ACCESS_KEY}=${token}; SameSite=Strict`;
        // Go to the dashboard page
        window.location.href = "/dashboard";
    });
  }}> Submit </button>
</Form>
